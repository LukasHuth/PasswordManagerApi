use diesel::{Queryable, Identifiable, Insertable};
use rocket::serde::{Serialize, Deserialize};

use crate::schema::passwords;

#[derive(Serialize, Queryable, Identifiable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = passwords)]
pub struct SimplePasswordDTO {
    pub id: i32,
    pub username: String,
    pub website: i32,
}

#[derive(Serialize, Queryable)]
#[serde(crate = "rocket::serde")]
pub struct MiddlePasswordDTO {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub nonce: String,
    pub website: i32,
}

#[derive(Serialize, Queryable)]
#[serde(crate = "rocket::serde")]
pub struct PasswordDTO {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub website: i32,
}

#[derive(Serialize, Queryable, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Info {
    pub password: String,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct PasswordInput {
    pub username: String,
    pub password: String,
    pub passphrase: String,
    pub website: i32,
}

#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = passwords)]
pub struct PasswdTemplate {
    pub website: i32,
    pub username: String,
    pub password: String,
    pub nonce: String,
    pub account: String,
}

impl From<PasswordDTO> for SimplePasswordDTO {
    fn from(p: PasswordDTO) -> Self {
        Self {
            id: p.id,
            website: p.website,
            username: p.username,
        }
  }
}

