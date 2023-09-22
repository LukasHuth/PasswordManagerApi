use diesel::{dsl::exists, query_dsl::methods::FilterDsl, ExpressionMethods, select, RunQueryDsl, result::Error};
use rocket::{request::{FromRequest, Outcome}, Request, http::{Status, Header}, Build, Rocket, fairing::{Fairing, Info, Kind}, Response};

#[macro_use]
extern crate rocket;


mod database;
mod models;
mod schema;
mod controller;


#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build().attach(CORS).mount("/", routes![
        controller::get_websites,
        controller::get_password_with_id,
        controller::get_passwords_with_website,
        controller::new_website,
        controller::new_password,
        controller::get_login,
        controller::get_website_by_name,
        all_options,
    ])
}
pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}
#[options("/<_..>")]
fn all_options() {
    /* Intentionally left empty */
}
pub struct ApiKey<'r>(&'r str);

#[derive(Debug)]
pub enum ApiKeyError {
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey<'r> {
    type Error = ApiKeyError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        fn is_valid(key: &str) -> bool {
            let connection = &mut database::establish_connection();
            let uuid_exists: Result<bool, Error> = select(
                exists(
                    crate::schema::logins::table.filter(
                        crate::schema::logins::token.eq(key)
                    )
                )
            ).get_result(connection);
            match uuid_exists {
                Ok(v) => return v,
                Err(e) => panic!("{}", e),
            }
        }

        match req.headers().get_one("x-api-key") {
            None => Outcome::Failure((Status::Unauthorized, ApiKeyError::Missing)),
            Some(key) if is_valid(key) => Outcome::Success(ApiKey(key)),
            Some(_) => Outcome::Failure((Status::Forbidden, ApiKeyError::Invalid)),
        }
    }
}

