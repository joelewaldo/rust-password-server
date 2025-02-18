use axum::{Json, extract::State};
use crate::bounded_context::infrastructure::db::postgres_db::Database;
use crate::bounded_context::domain::password_db::PasswordDb;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct DeletePasswordInput {
    id: String,
}

#[derive(Serialize)]
pub struct ResponseMessage {
    message: String,
}

pub async fn delete_password(
    State(database): State<Database>,
    Json(payload): Json<DeletePasswordInput>,
) -> Result<Json<ResponseMessage>, (axum::http::StatusCode, String)> {
    let id = match Uuid::parse_str(&payload.id) {
        Ok(uuid) => uuid,
        Err(_) => return Err((axum::http::StatusCode::BAD_REQUEST, "Invalid password ID.".to_string())),
    };

    let mut db = database;

    match db.delete(id).await {
        Ok(_) => Ok(Json(ResponseMessage {
            message: "Password deleted successfully".to_string(),
        })),
        Err(err) => Err((axum::http::StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}
