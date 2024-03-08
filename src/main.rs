#[macro_use]
extern crate rocket;

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
fn create_post() -> Value {
    json!({"id": 1, "name": "4prndz", "title": "Quarter of Silence"})
}

#[put("/posts/<_id>", format = "json")]
fn update_post(_id: u32) -> Value {
    json!({"id": 1, "name": "4prndz", "title": "Quarter of Silence"})
}

#[delete("/posts/<_id>")]
fn delete_post(_id: u32) -> status::NoContent {
    status::NoContent
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount(
            "/",
            routes![get_posts, view_post, create_post, update_post, delete_post],
        )
        .launch()
        .await;
}
