use diesel::{dsl::exists, ExpressionMethods, select, RunQueryDsl, result::Error, QueryResult};
use rocket::serde::json::Json;

use sha256::digest;

use crate::{models::{logins as m_logins, accounts as m_accounts}, database};

use diesel::QueryDsl;

#[post("/login", data = "<password>")]
pub fn get_login(password: Json<m_logins::LoginInput>) -> Json<m_logins::TokenDTO> {
    use crate::schema::{accounts::dsl as accounts, logins::dsl as logins};
    let connection = &mut database::establish_connection();
    // websites.load::<WebsiteDTO>(connection).map(Json).expect("Error loading websites")
    let username = password.username.clone();
    let password = password.password.clone();
    let hashed_password = digest(password);
    let exists: Result<bool, Error> = select(
        exists(
            crate::schema::accounts::table.filter(
                crate::schema::accounts::password.eq(hashed_password.clone())
            ).filter(
                crate::schema::accounts::username.eq(username))
        )
    ).get_result(connection);
    println!("hash: {}", hashed_password);
    let exists = match exists { Ok(v) => v, Err(e) => panic!("{}", e) };
    if !exists {
        println!("No Account found");
        return Json(m_logins::TokenDTO { token: String::new() });
    }


    let account: QueryResult<m_accounts::AccountDTO> =
    QueryDsl::filter(accounts::accounts, accounts::password.eq(hashed_password))
        .select((accounts::id, accounts::password))
        .first(connection);
    let account = match account {
        Ok(v) => v,
        Err(e) => panic!("{}", e)
    };

    let access_token = uuid::Uuid::new_v4();

    let _ = diesel::delete(logins::logins.filter(logins::account.eq(account.id.clone()))).execute(connection);
    let login_insert_input = m_logins::LoginInsertInput { token: access_token.to_string(), account: account.id };

    diesel::insert_into(logins::logins)
        .values(login_insert_input)
        .execute(connection)
        .expect("error adding login");

    Json(m_logins::TokenDTO {token: access_token.to_string()})
}
/*
#[post("/websites", data = "<website>")]
pub fn login(website: Json<WebsiteInput>) -> Json<WebsiteDTO> {
    use crate::schema::websites;


    let connection = &mut database::establish_connection();
    diesel::insert_into(websites::table)
        .values(website.into_inner())
        .execute(connection)
        .expect("error adding sighting");


    Json(websites::table
        .order(websites::id.desc())
        .first(connection).unwrap()
    )
}
*/
