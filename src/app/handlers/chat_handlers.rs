use futures::TryStreamExt;
use rocket::serde::json::Json;
use mongodb::bson::{doc, from_document, Document};
use crate::models::chat::Chat;
use crate::database;
use crate::app::requests::chat_requests::CreateChat;
#[get("/")]
pub async fn index_handler() -> Json<Vec<Chat>> {
    let database = database::mongo_connection().await;
    let my_coll = database.collection::<Document>("chats");

    let mut cursor = my_coll.find(doc! {}).await.expect("Failed to get chats");

    let mut chats = Vec::new();

    while let Some(doc) = cursor.try_next().await.expect("Failed to retrieve chat") {
        let chat: Chat = from_document(doc).expect("Invalid chat");
        chats.push(chat);
    }

    Json(chats)
}

#[post("/", data = "<data>")]
pub async fn post_handler(data: Json<CreateChat<'_>>) -> Json<Chat> {
    let database = database::mongo_connection().await;
    let my_coll = database.collection("chats");
    let chat = Chat {
        id: None,
        name: data.name.to_string(),
    };
    let res = my_coll.insert_one(chat).await.expect("Failed to create chat");
    let chat = my_coll.find_one(
        doc! {"_id": res.inserted_id},
    ).await.expect("Failed to find chat").unwrap();
    Json(chat)
}