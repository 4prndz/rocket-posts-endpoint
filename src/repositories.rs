use diesel::prelude::*;

use crate::{
    models::{NewPost, Post},
    schema::posts,
};

pub struct PostRepository;

impl PostRepository {
    pub fn find_all(c: &mut SqliteConnection, limit: i64) -> QueryResult<Vec<Post>> {
        posts::table
            .order(posts::id.desc())
            .limit(limit)
            .load::<Post>(c)
    }

    pub fn find(c: &mut SqliteConnection, id: i32) -> QueryResult<Post> {
        posts::table.find(id).get_result::<Post>(c)
    }

    pub fn create(c: &mut SqliteConnection, new_post: NewPost) -> QueryResult<Post> {
        diesel::insert_into(posts::table)
            .values(new_post)
            .execute(c)?;
        let last_id = Self::last_inserted_id(c)?;
        Self::find(c, last_id)
    }

    pub fn update(c: &mut SqliteConnection, id: i32, post: Post) -> QueryResult<Post> {
        diesel::update(posts::table.find(id))
            .set((
                posts::name.eq(post.name.to_owned()),
                posts::title.eq(post.title.to_owned()),
            ))
            .execute(c)?;
        Self::find(c, id)
    }

    pub fn delete(c: &mut SqliteConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(posts::table.find(id)).execute(c)
    }

    fn last_inserted_id(c: &mut SqliteConnection) -> QueryResult<i32> {
        posts::table
            .select(posts::id)
            .order(posts::id.desc())
            .first(c)
    }
}
