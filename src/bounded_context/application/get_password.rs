use axum::{Json, extract::State, extract::Query};
use crate::bounded_context::infrastructure::db::postgres_db::Database;
use crate::bounded_context::domain::password_db::PasswordDb;
use crate::bounded_context::domain::password::Password;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct GetPasswordInput {
    id: String,
}

#[derive(Serialize)]
pub struct ResponseMessage {
    message: String,
}

pub async fn get_password(
    State(database): State<Database>,
    Query(payload): Query<GetPasswordInput>,
) -> Result<Json<Password>, (axum::http::StatusCode, String)> {
    let id = match Uuid::parse_str(&payload.id) {
        Ok(uuid) => uuid,
        Err(_) => return Err((axum::http::StatusCode::BAD_REQUEST, "Invalid password ID.".to_string())),
    };

    let mut db = database;

    match db.get_by_id(id).await {
        Ok(password) => Ok(Json(password)),
        Err(err) => Err((axum::http::StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}