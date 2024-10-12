use std::sync::Arc;

use axum::response::Html;
use axum::routing::get;
use axum::Router;

use crate::handlers::posts::get_post::get_all_posts;
use crate::AppState;

pub fn all_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .merge(handler_hello())
        .merge(handler_posts(state))
}

pub fn handler_hello() -> Router {
    Router::new().route("/hello", get(|| async { Html("Hello World") }))
}

pub fn handler_posts(state: Arc<AppState>) -> Router {
    return Router::new()
        .route("/posts", get(get_all_posts))
        .with_state(state);
}
