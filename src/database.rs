use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use mongodb::Database;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub async fn mongo_connection() -> Database {
    dotenv().ok();

    let database_url = env::var("MONGODB_URL").expect("MONGODB_URL must be set");

    let client = mongodb::Client::with_uri_str(&database_url).await.unwrap();

    client.database("chat")
}