use diesel::{Queryable, Insertable};
use rocket::serde::{Serialize, Deserialize};

use crate::schema::websites;

#[derive(Serialize, Queryable)]
#[serde(crate = "rocket::serde")]
pub struct WebsiteDTO {
    pub id: i32,
    pub name: String,
}


#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = websites)]
pub struct WebsiteInput {
    pub name: String
}
