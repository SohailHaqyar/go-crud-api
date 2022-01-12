#[macro_use]
extern crate rocket;

mod posts;

use posts::*;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/posts", routes![get_posts, create_post, publish_post])
}
