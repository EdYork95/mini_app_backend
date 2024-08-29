use crate::{db::schema::posts, domain::models::post::PostModel};
use chrono::NaiveDateTime;
use deadpool_diesel::{postgres::Pool, InteractError};
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

pub async fn get_all(pool: &Pool) -> Result<Vec<PostModel>, InteractError> {
    let conn = pool.get().await.unwrap();
    let res: Result<Vec<PostDb>, _> = conn
        .interact(|conn| {
            let query = posts::table.into_boxed::<diesel::pg::Pg>();
            // Select the posts matching the query
            query.select(PostDb::as_select()).load::<PostDb>(conn)
        })
        .await
        .unwrap();

    // Handle the result of the query
    let posts: Vec<PostModel> = res
        .unwrap()
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
