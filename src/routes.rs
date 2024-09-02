use axum::response::Html;
use axum::routing::get;
use axum::Router;
use deadpool_diesel::postgres::Pool;

use crate::handlers::posts::get_post::get_all_posts;

pub fn handler_hello() -> Router {
    Router::new().route("/hello", get(|| async { Html("Hello World") }))
}

// pub fn handler_posts(pool: &Pool) -> Router {
//     Router::with_state(&pool).route("/posts", get(get_all_posts))
// }
