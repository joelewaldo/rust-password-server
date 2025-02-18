use axum::{Json, extract::State, extract::Query};
use crate::bounded_context::infrastructure::db::postgres_db::Database;
use crate::bounded_context::domain::password_db::PasswordDb;
use crate::bounded_context::domain::password::Password;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Serialize)]
pub struct ResponseMessage {
    message: String,
}

#[derive(Error, Debug)]
pub enum SearchPasswordError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] Box<dyn std::error::Error>),
}

#[derive(Deserialize)]
pub struct SearchPasswordInput {
    search_term: String,
    page: Option<u32>,
    page_size: Option<u32>,
}

pub async fn search_password(
    State(database): State<Database>,
    Query(payload): Query<SearchPasswordInput>,
) -> Result<Json<Vec<Password>>, (axum::http::StatusCode, String)> {
    let mut db = database;

    let page = payload.page.unwrap_or(1);
    let page_size = payload.page_size.unwrap_or(20);

    match db.search_by_service(&payload.search_term, page, page_size).await {
        Ok(passwords) => Ok(Json(passwords)),
        Err(err) => Err((axum::http::StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}
