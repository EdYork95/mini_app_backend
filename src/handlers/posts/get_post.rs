use axum::{
    extract::{Query, State},
    response::IntoResponse,
    Json,
};
use repository::post_repository;
use serde::Deserialize;
use std::sync::Arc;

use crate::{domain::errors::infra_error::InfrastructureError, repository, AppState};

#[derive(Deserialize)]
pub struct Pagination {
    offset: i64,
}

pub async fn get_all_posts(
    State(state): State<Arc<AppState>>,
    pagination: Query<Pagination>,
) -> impl IntoResponse {
    match post_repository::get_all(&state.db, pagination.offset).await {
        Ok(posts) => Json(posts).into_response(),
        Err(_) => InfrastructureError::InternalError.into_response(),
    }
}
