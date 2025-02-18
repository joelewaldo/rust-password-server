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
}

pub async fn search_password(
    State(database): State<Database>,
    Query(payload): Query<SearchPasswordInput>,
) -> Result<Json<Vec<Password>>, (axum::http::StatusCode, String)> {
    let mut db = database;

    match db.search_by_service(&payload.search_term).await {
        Ok(passwords) => Ok(Json(passwords)),
        Err(err) => Err((axum::http::StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}
