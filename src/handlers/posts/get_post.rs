use axum::{extract::State, response::IntoResponse, Json};
use repository::post_repository;
use std::sync::Arc;

use crate::{domain::errors::infra_error::InfrastructureError, repository, AppState};

pub async fn get_all_posts(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    match post_repository::get_all(&state.db).await {
        Ok(posts) => Json(posts).into_response(),
        Err(_) => InfrastructureError::InternalError.into_response(),
    }
}
