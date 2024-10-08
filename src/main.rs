#[macro_use] extern crate rocket;
pub mod app;
pub mod lib;
pub mod models;
pub mod schema;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![app::post_handlers::index_handler])
        .mount("/", routes![app::post_handlers::post_handler])
        .mount("/", routes![app::post_handlers::put_handler])
        .mount("/", routes![app::post_handlers::delete_handler])
}
