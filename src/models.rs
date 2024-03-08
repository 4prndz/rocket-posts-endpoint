use crate::schema::posts;
use diesel::{prelude::Insertable, AsChangeset, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Queryable, AsChangeset)]
pub struct Post {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub name: String,
    pub title: String,
    #[serde(skip_deserializing)]
    pub content: Option<String>,
    #[serde(skip_deserializing)]
    pub likes: Option<i32>,
    #[serde(skip_deserializing)]
    pub is_published: bool,
    #[serde(skip_deserializing)]
    pub created_at: String,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = posts)]
pub struct NewPost {
    pub name: String,
    pub title: String,
}
