use aes_gcm::{Aes256Gcm, aead::{generic_array::GenericArray, OsRng, heapless, Aead}, KeyInit, AeadCore, AeadInPlace, Nonce, Key};
use base64::{engine::general_purpose, Engine};
//use aes_gcm::{AesGcm, aes::{Aes256, cipher::typenum::{UInt, UTerm}}, aead::{consts::{B1, B0}, Aead, generic_array::GenericArray}, Aes256Gcm, Key, KeyInit};
use diesel::{QueryResult, QueryDsl, ExpressionMethods, RunQueryDsl, Queryable, JoinOnDsl};
use regex::Regex;
use rocket::{serde::{json::Json, Serialize, Deserialize}, http::Status};
use crate::{models::passwords as passwords, database, ApiKey};
// use crypto::cipher::aes;
//
pub mod utils;

#[get("/passwords/<website_id>")]
pub fn get_passwords_with_website(website_id: i32, key: ApiKey<'_>) -> Json<Vec<passwords::SimplePasswordDTO>> {
    let connection = &mut database::establish_connection();
    use crate::models::passwords as m_passwords;
    use crate::schema::{logins::dsl as logins, passwords::dsl as passwords};

    let query_result: QueryResult<Vec<m_passwords::SimplePasswordDTO>> = 
        passwords::passwords
        .left_join(logins::logins.on(
            logins::account.eq(passwords::account)))
        .filter(logins::token.eq(key.0))
        .filter(passwords::website.eq(website_id))
        .select((passwords::id, passwords::username, passwords::website))
        .load(connection);

    query_result.map(Json).expect("Error loading Passwords")
}

#[post("/passwords/<website_id>/<passwd_id>", data="<info>")]
pub fn get_password_with_id(website_id: i32, passwd_id: i32, info: Json<passwords::Info>, key: ApiKey<'_>) -> Json<Vec<passwords::PasswordDTO>> {
    let connection = &mut database::establish_connection();
    use crate::models::passwords as m_passwords;
    use crate::schema::{logins::dsl as logins, passwords::dsl as passwords};

    let query_result: QueryResult<Vec<m_passwords::MiddlePasswordDTO>> = 
    passwords::passwords
        .left_join(logins::logins.on(
            logins::account.eq(passwords::account)))
        .filter(passwords::website.eq(website_id))
        .filter(logins::token.eq(key.0))
        .filter(passwords::id.eq(passwd_id))
        .select((
            passwords::id,
            passwords::username,
            passwords::password,
            passwords::nonce,
            passwords::website))
        .load(connection);
    let result = query_result.unwrap_or(Vec::new());

    let res = result.into_iter().map(|mpdto| {

        let res = String::from_utf8(
            utils::decrypt(
                &mpdto.password,
                &info.password
            ).unwrap_or(vec![])
        ).unwrap_or(String::new());

        let password = res;
        m_passwords::PasswordDTO {
            id: mpdto.id,
            username: mpdto.username,
            website: mpdto.website,
            password
        }
    }).collect();

    Json(res)
}

#[derive(Serialize, Queryable)]
#[serde(crate = "rocket::serde")]
struct AccountId
    {
        token: String,
        account: String,
    }

#[post("/passwords", data = "<passwd>")]
pub fn new_password(passwd: Json<passwords::PasswordInput>, key: ApiKey<'_>) -> Result<Json<passwords::MiddlePasswordDTO>, Status> {
    use crate::models::passwords as m_passwords;
    use crate::schema::{logins::dsl as logins, passwords::dsl as passwords};

    let connection = &mut database::establish_connection();
    let res: QueryResult<Vec<AccountId>> = 
    logins::logins
        .filter(logins::token.eq(key.0))
        .load(connection);
    
    let res = res.unwrap();
    let valid_masterpassword = Regex::new(r".{16,}").unwrap();
    if !valid_masterpassword.is_match(&passwd.passphrase) {
        return Err(Status::NotAcceptable);
    }
    if res.len() == 0 {
        return Err(Status::NotFound);
        // return Json(m_passwords::PasswordDTO { password: String::from("null"), nonce: String::from("null"), username: String::from("null"), id: 0, website: 0});
    }
    let acc = res.first();
    if acc.is_none() {
        // return Json(m_passwords::PasswordDTO { password: String::from("null"), nonce: String::from("null"), username: String::from("null"), id: 0, website: 0});
        return Err(Status::NotFound);
    }
    let acc = acc.unwrap();
    let phrase = passwd.passphrase.clone();
    let password = passwd.password.clone();
    let res = utils::encrypt(password.as_bytes(), &phrase);
    // let password = base64::engine::general_purpose::STANDARD.encode(passwd.password.clone());
    // let a = encript(passwd.password.clone().as_bytes(), _cipher);
    let passwd = Json(m_passwords::PasswdTemplate {
        website: passwd.website,
        username: passwd.username.clone(),
        password: res.to_string(),
        nonce: String::new(),
        account: acc.account.clone(),
    });
    diesel::insert_into(passwords::passwords)
        .values(passwd.into_inner())
        .execute(connection)
        .expect("error adding sighting");


    Ok(
        Json(passwords::passwords
            .order(passwords::id.desc())
            .select((
                passwords::id,
                passwords::username,
                passwords::password,
                passwords::nonce,
                passwords::website))
            .first(connection).unwrap()
        )
    )
}
