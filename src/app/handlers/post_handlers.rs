use diesel::prelude::*;
use rocket::serde::json::Json;
use crate::models::post::Post;
use crate::app::requests::post_requests::{NewPost, PutPost};
use crate::schema::posts;
use crate::schema::posts::dsl::posts as posts_table;
use crate::schema::posts::{body, published, title};

#[get("/")]
pub fn index_handler() -> Json<Vec<Post>> {
    let connection = &mut crate::lib::establish_connection();
    let results = posts_table
        .filter(published.eq(true))
        .limit(5)
        .select(Post::as_select())
        .load(connection)
        .expect("Error loading posts");

    Json(results)
}
#[post("/", data = "<post>")]
pub fn post_handler(post: Json<NewPost<'_>>) -> Json<Post> {
    let connection = &mut crate::lib::establish_connection();

    Json(diesel::insert_into(posts::table)
        .values(&post.into_inner())
        .returning(Post::as_returning())
        .get_result(connection)
        .expect("Error saving new post"))
}
#[put("/<post_id>", data = "<post>")]
pub fn put_handler(post_id: i32, post: Json<PutPost<'_>>) -> Json<Post> {
    let connection = &mut crate::lib::establish_connection();
    let post = post.into_inner();
    Json(diesel::update(posts_table.find(post_id))
        .set((published.eq(post.published), title.eq(post.title), body.eq(post.body)))
        .returning(Post::as_returning())
        .get_result(connection)
        .expect("Error updating post"))
}
#[delete("/<post_id>")]
pub fn delete_handler(post_id: i32) -> Json<&'static str> {
    let connection = &mut crate::lib::establish_connection();
    diesel::delete(posts_table.find(post_id))
        .execute(connection)
        .expect("Error deleting post");

    Json("Successfully deleted")
}