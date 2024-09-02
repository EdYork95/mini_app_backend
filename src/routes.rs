use axum::response::Html;
use axum::routing::get;
use axum::Router;

pub fn handler_hello() -> Router {
    Router::new().route("/hello", get(|| async { Html("Hello World") }))
}
