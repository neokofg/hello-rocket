use diesel::Insertable;
use rocket::serde::Deserialize;
use crate::schema::posts;

#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = posts)]
pub struct NewPost<'r> {
    pub title: &'r str,
    pub body: &'r str,
}
#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct PutPost<'r> {
    pub title: &'r str,
    pub body: &'r str,
    pub published: bool,
}
