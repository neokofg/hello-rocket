use futures::{SinkExt, StreamExt, TryStreamExt};
use mongodb::bson::{doc, from_document};
use rocket::serde::json::Json;
use rocket::State;
use tokio::sync::mpsc;
use crate::models::message::Message;
use crate::app::requests::message_requests::CreateMessage;
use crate::database;
use crate::models::chat::{Tx, Rx, ChatServer};

#[post("/<id>/message", data = "<data>")]
pub async fn post_handler(id: &str, data: Json<CreateMessage<'_>>) -> Json<Message> {
    let database = database::mongo_connection().await;
    let my_coll = database.collection::<Message>("messages");

    let message = Message {
        id: None,
        sender_name: data.sender_name.to_string(),
        message: data.message.to_string(),
        chat_id: id.to_string()
    };
    let res = my_coll.insert_one(message).await.expect("Failed to create message");
    let message = my_coll.find_one(
        doc! {"_id": res.inserted_id},
    ).await.expect("Failed to find message").unwrap();

    Json(message)
}

#[get("/<id>")]
pub async fn index_handler(id: &str) -> Json<Vec<Message>> {
    let database = database::mongo_connection().await;
    let my_coll = database.collection("messages");

    let mut cursor = my_coll.find(doc! {"chat_id": id}).await.expect("Failed to get messages");

    let mut messages = Vec::new();

    while let Some(doc) = cursor.try_next().await.expect("Failed to retrieve messages") {
        let message: Message = from_document(doc).expect("Invalid message");
        messages.push(message);
    }

    Json(messages)
}

#[get("/<id>/echo")]
pub async fn echo_handler(
    ws: ws::WebSocket,
    id: String
) -> ws::Channel<'static> {
    ws.channel(move |mut stream| Box::pin(async move {
        let message = format!("Hello, {}!", id);
        let _ = stream.send(message.into()).await;
        Ok(())
    }))
}