use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CreateMessage<'r> {
    pub sender_name: &'r str,
    pub message: &'r str,
}