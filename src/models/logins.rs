use diesel::{Insertable, Queryable};
use rocket::serde::{Deserialize, Serialize};

use crate::schema::logins;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct LoginInput {
    pub password: String,
    pub username: String
}

#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = logins)]
pub struct LoginInsertInput {
    pub token: String,
    pub account: String,
}

#[derive(Serialize, Queryable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = logins)]
pub struct TokenDTO {
    pub token: String,
}
