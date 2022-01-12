#[macro_use]
extern crate rocket;
extern crate diesel;
extern crate pes_book_api;

use crate::diesel::prelude::*;
use pes_book_api::schema::posts;
use rocket::serde::json::Json;

use self::models::*;
use self::pes_book_api::*;

#[get("/")]
pub fn get_posts() -> Json<Vec<Post>> {
    let conn = establish_connection();
    let results = posts::table
        .load::<Post>(&conn)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    return Json(results);
}

#[post("/", format = "application/json", data = "<post>")]
pub fn create_post(post: Json<NewPost>) -> Json<Post> {
    let conn = establish_connection();
    let new_post = NewPost {
        title: &post.title,
        body: &post.body,
    };

    let post = diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(&conn)
        .expect("Error saving new post");

    Json(post)
}

// Publish a post

#[put("/publish/<post_id>")]
pub fn publish_post(post_id: i32) -> Json<Post> {
    use pes_book_api::schema::posts::dsl::{posts, published};
    let conn = establish_connection();

    let post = diesel::update(posts.find(post_id))
        .set(published.eq(true))
        .get_result::<Post>(&conn)
        .expect(&format!("Unable to find post {}", post_id));

    Json(post)
}

// pub fn routes() -> Vec<rocket::Route> {
//     return [get_posts, create_post, publish_post];
// }

fn main() {
    println!("Hello, Posts!");
    // routes![get_posts, create_post, publish_post]
}
