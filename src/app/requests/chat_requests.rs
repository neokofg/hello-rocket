use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateChat<'r> {
    pub name: &'r str,
}