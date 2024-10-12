use crate::db::schema::posts;
use axum::{response::IntoResponse, Json};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Queryable, Selectable, Serialize)]
#[diesel(table_name = posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PostModel {
    pub id: Uuid,
    pub body: String,
    pub imageurl: String,
    pub date_created: NaiveDateTime,
}

impl IntoResponse for PostModel {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}
