use crate::{
    db::schema::posts,
    domain::{
        errors::infra_error::{adapt_infra_error, InfrastructureError},
        models::post::PostModel,
    },
};
use chrono::NaiveDateTime;
use deadpool_diesel::postgres::Pool;
use diesel::{QueryDsl, Queryable, RunQueryDsl, Selectable, SelectableHelper};
use uuid::Uuid;

// Define a struct representing the database schema for posts
#[derive(Queryable, Selectable)]
#[diesel(table_name = posts)] // Use the 'posts' table
#[diesel(check_for_backend(diesel::pg::Pg))] // Check compatibility with PostgreSQL
pub struct PostDb {
    pub id: Uuid,
    pub body: String,
    pub imageurl: String,
    pub date_created: NaiveDateTime,
}

pub async fn get_all(pool: &Pool, offset: i64) -> Result<Vec<PostModel>, InfrastructureError> {
    let conn = pool.get().await.map_err(adapt_infra_error)?;
    let res = conn
        .interact(move |conn| {
            let query = posts::table
                .limit(6)
                .offset(offset)
                .into_boxed::<diesel::pg::Pg>();
            // Select the posts matching the query
            query.select(PostDb::as_select()).load::<PostDb>(conn)
        })
        .await
        .map_err(adapt_infra_error)? // return type is nested Result<Result<>>
        .map_err(adapt_infra_error)?;

    // Handle the result of the query
    let posts: Vec<PostModel> = res
        .into_iter()
        .map(|post_db| adapt_post_db_to_post(post_db))
        .collect();

    println!("{:?}", posts);
    Ok(posts)
}

fn adapt_post_db_to_post(post_db: PostDb) -> PostModel {
    PostModel {
        id: post_db.id,
        body: post_db.body,
        imageurl: post_db.imageurl,
        date_created: post_db.date_created,
    }
}
