use axum::{Json, extract::State, extract::Query};
use crate::bounded_context::infrastructure::db::postgres_db::Database;
use crate::bounded_context::domain::password_db::{PasswordDb, SortBy};
use crate::bounded_context::domain::password::Password;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use std::str::FromStr;

#[derive(Serialize)]
pub struct ResponseMessage {
    message: String,
}

#[derive(Error, Debug)]
pub enum SortPasswordError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] Box<dyn std::error::Error>),
}

#[derive(Deserialize)]
pub struct SortPasswordInput {
    sort_by: String,
    page: Option<u32>,
    page_size: Option<u32>,
}

pub async fn sort_passwords(
    State(database): State<Database>,
    Query(payload): Query<SortPasswordInput>,
) -> Result<Json<Vec<Password>>, (axum::http::StatusCode, String)> {
    let mut db = database;
    let convert_sort_by = SortBy::from_str(&payload.sort_by);

    let page = payload.page.unwrap_or(1);
    let page_size = payload.page_size.unwrap_or(db.config.pagination_max_size);

    if page_size >= db.config.pagination_max_size {
        return Err((axum::http::StatusCode::BAD_REQUEST, "Max Pagination Size Exceeded".to_string()))
    }

    let sort_by = match convert_sort_by {
        Ok(sort_by) => sort_by,
        Err(err) => return Err((axum::http::StatusCode::BAD_REQUEST, err.to_string())),
    };

    match db.list_sorted(&sort_by, page, page_size).await {
        Ok(passwords) => Ok(Json(passwords)),
        Err(err) => Err((axum::http::StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}