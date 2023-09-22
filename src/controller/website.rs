use diesel::{RunQueryDsl, QueryDsl, ExpressionMethods, TextExpressionMethods, JoinOnDsl, dsl::exists, select, result::Error};
use regex::Regex;
use rocket::{serde::json::Json, http::Status};

use crate::{models::websites, database, ApiKey};


#[get("/websites")]
pub fn get_websites(key: ApiKey<'_>) -> Json<Vec<websites::WebsiteDTO>> {
    let connection = &mut database::establish_connection();
    use crate::models::websites as m_websites;
    use crate::schema::{websites::dsl as websites, logins::dsl as logins, passwords::dsl as passwords};
    websites::websites
        .left_join(passwords::passwords.on(passwords::website.eq(websites::id)))
        .left_join(logins::logins.on(logins::account.eq(passwords::account)))
        .filter(logins::token.eq(key.0))
        .select((websites::id, websites::name))
        .distinct()
        .load::<m_websites::WebsiteDTO>(connection)
        .map(Json)
        .expect("Error loading websites")
}
#[get("/websites/<name>")]
pub fn get_website_by_name(name: String, _key: ApiKey<'_>) -> Result<Json<Vec<websites::WebsiteDTO>>, Status> {
    let connection = &mut database::establish_connection();
    let ip_regex = Regex::new(r#"\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}(?::\d{0,4})?"#).unwrap();
    let ip_regex_res = ip_regex.is_match(&name);
    if ip_regex_res {
        return Err(Status::NotFound);
    }
    use crate::models::websites as m_websites;
    use crate::schema::websites::dsl as websites;
    let search_query = "%".to_string()+&name+"%";
    if search_query.len() <= 4 {
        return Err(Status::NotFound);
    }
    Ok(
        websites::websites
            .filter(websites::name.like(search_query))
            .load::<m_websites::WebsiteDTO>(connection)
            .map(Json)
            .expect("Error loading websites")
    )
}
#[post("/websites", data = "<website>")]
pub fn new_website(website: Json<websites::WebsiteInput>, _key: ApiKey<'_>) -> Result<Json<websites::WebsiteDTO>, Status> {
    use crate::schema::websites::dsl as websites;

    let ip_regex = Regex::new(r#"\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}(?::\d{0,4})?"#).unwrap();
    let ip_regex_res = ip_regex.is_match(&website.name);
    if ip_regex_res {
        return Err(Status::NotAcceptable);
    }

    let websitename = "%".to_string()+&website.name+"%";
    let connection = &mut database::establish_connection();
    let website_exists: bool = select(exists(websites::websites.filter(websites::name.like(websitename.clone())))).get_result(connection).unwrap();
    println!("{}", website_exists);
    if !website_exists {
        diesel::insert_into(websites::websites)
            .values(website.into_inner())
            .execute(connection)
            .expect("error adding sighting");
    }


    Ok(
        Json(websites::websites
            .order(websites::id.asc())
            .filter(websites::name.like(websitename))
            .first(connection).unwrap()
        )
    )
}
