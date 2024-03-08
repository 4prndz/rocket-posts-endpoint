#[macro_use]
extern crate rocket;

mod auth;
mod models;
mod repositories;
mod schema;

use crate::auth::BasicAuth;
use crate::repositories::PostRepository;
use diesel::result::Error::NotFound;
use diesel_migrations::EmbeddedMigrations;
use models::{NewPost, Post};
use rocket::{
    fairing::AdHoc,
    http::Status,
    response::status::{self, Custom},
    serde::json::{json, Json, Value},
    Build, Rocket,
};
use rocket_sync_db_pools::database;

#[database("sqlite")]
struct DbConn(diesel::SqliteConnection);

#[get("/posts")]
async fn get_posts(db: DbConn) -> Result<Value, Custom<Value>> {
    db.run(|c| {
        PostRepository::find_all(c, 1000)
            .map(|posts| json!(posts))
            .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}

#[get("/posts/<id>")]
async fn view_post(id: i32, db: DbConn) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        PostRepository::find(c, id)
            .map(|post| json!(post))
            .map_err(|e| match e {
                NotFound => Custom(Status::NotFound, json!(e.to_string())),
                _ => Custom(Status::InternalServerError, json!(e.to_string())),
            })
    })
    .await
}

#[post("/posts", format = "json", data = "<new_post>")]
async fn create_post(
    _auth: BasicAuth,
    db: DbConn,
    new_post: Json<NewPost>,
) -> Result<Value, Custom<Value>> {
    db.run(|c| {
        PostRepository::create(c, new_post.into_inner())
            .map(|id| json!(id))
            .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}

#[put("/posts/<id>", format = "json", data = "<post>")]
async fn update_post(
    id: i32,
    _auth: BasicAuth,
    db: DbConn,
    post: Json<Post>,
) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        PostRepository::update(c, id, post.into_inner())
            .map(|post| json!(post))
            .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}

#[delete("/posts/<id>")]
async fn delete_post(
    id: i32,
    _auth: BasicAuth,
    db: DbConn,
) -> Result<status::NoContent, Custom<Value>> {
    db.run(move |c| {
        PostRepository::delete(c, id)
            .map(|_| status::NoContent)
            .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}

async fn run_db_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
    use diesel_migrations::{embed_migrations, MigrationHarness};
    const MIGRATIONS: EmbeddedMigrations = embed_migrations!();
    DbConn::get_one(&rocket)
        .await
        .expect("Unable to retrieve connection")
        .run(|c| {
            c.run_pending_migrations(MIGRATIONS)
                .expect("Migrations failed");
        })
        .await;
    rocket
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
        .attach(DbConn::fairing())
        .attach(AdHoc::on_ignite("Diesel migrations", run_db_migrations))
        .launch()
        .await;
}
