#[macro_use]
extern crate rocket;

mod auth;
use crate::auth::BasicAuth;

use rocket::{
    response::status,
    serde::json::{json, Value},
};

#[get("/posts")]
fn get_posts() -> Value {
    json!([{"id": 1, "name": "4prndz"}, {"id": 2, "name": "Kouichi"}])
}

#[get("/posts/<_id>")]
fn view_post(_id: u32) -> Value {
    json!({"id": 1, "name": "4prndz", "title": "Quarter of Silence"})
}

#[post("/posts", format = "json")]
fn create_post(_auth: BasicAuth) -> Value {
    json!({"id": 1, "name": "4prndz", "title": "Quarter of Silence"})
}

#[put("/posts/<_id>", format = "json")]
fn update_post(_id: u32, _auth: BasicAuth) -> Value {
    json!({"id": 1, "name": "4prndz", "title": "Quarter of Silence"})
}

#[delete("/posts/<_id>")]
fn delete_post(_id: u32, _auth: BasicAuth) -> status::NoContent {
    status::NoContent
}

#[catch(404)]
fn not_found() -> Value {
    json!("Not Found")
}

#[catch(401)]
fn unauthorized() -> Value {
    json!("unauthorized")
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount(
            "/",
            routes![get_posts, view_post, create_post, update_post, delete_post],
        )
        .register("/", catchers![not_found, unauthorized])
        .launch()
        .await;
}
