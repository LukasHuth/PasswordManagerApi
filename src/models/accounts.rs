use diesel::Queryable;
use rocket::serde::Serialize;


#[derive(Serialize, Queryable)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = accounts)]
pub struct AccountDTO {
    pub id: String,
    pub username: Option<String>,
}
