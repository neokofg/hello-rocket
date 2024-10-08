#[macro_use] extern crate rocket;

use std::collections::HashMap;
use tokio::sync::RwLock;
use crate::models::chat::ChatServer;

pub mod app;
pub mod database;
pub mod models;
pub mod schema;
#[rocket::main]
async fn main() -> Result<(), rocket::Error> {

    let _rocket = rocket::build()
        .manage(ChatServer {
            rooms: RwLock::new(HashMap::new())
        })
        .mount("/", routes![
            app::handlers::post_handlers::index_handler,
            app::handlers::post_handlers::post_handler,
            app::handlers::post_handlers::put_handler,
            app::handlers::post_handlers::delete_handler
        ])
        .mount("/chat", routes![
            app::handlers::chat_handlers::index_handler,
            app::handlers::chat_handlers::post_handler,
            app::handlers::message_handlers::post_handler,
            app::handlers::message_handlers::index_handler,
            app::handlers::message_handlers::echo_handler
        ])
        .launch()
        .await?;

    Ok(())
}
