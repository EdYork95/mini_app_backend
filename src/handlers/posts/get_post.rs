use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, Json};
use repository::post_repository;

use crate::{repository, AppState};

pub async fn get_all_posts(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    return Json(post_repository::get_all(&state.db).await.unwrap());
}
