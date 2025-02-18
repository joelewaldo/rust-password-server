use axum::{Json, extract::State};
use crate::bounded_context::domain::password::Password;
use crate::bounded_context::infrastructure::db::postgres_db::Database;
use crate::bounded_context::domain::password_db::PasswordDb;
use crate::bounded_context::utility::encryption::{is_valid_cipher, is_valid_nonce};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Deserialize)]
pub struct NewPassword {
    service: String,
    nonce: String,
    cipher: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Serialize)]
pub struct ResponseMessage {
    message: String,
}

pub async fn create_password(
    State(database): State<Database>,
    Json(payload): Json<NewPassword>,
) -> Result<Json<ResponseMessage>, (axum::http::StatusCode, String)> {
    if !is_valid_nonce(&payload.nonce) {
        return Err((axum::http::StatusCode::BAD_REQUEST, "Invalid nonce provided.".to_string()));
    }

    if !is_valid_cipher(&payload.cipher) {
        return Err((axum::http::StatusCode::BAD_REQUEST, "Invalid cipher provided.".to_string()));
    }

    let password = Password {
        id: Uuid::new_v4(),
        service: payload.service,
        nonce: payload.nonce,
        cipher: payload.cipher,
        created_at: payload.created_at,
        updated_at: payload.updated_at,
    };

    let mut db = database;

    match db.save(password).await {
        Ok(_) => Ok(Json(ResponseMessage {
            message: "Password saved successfully".to_string(),
        })),
        Err(err) => Err((axum::http::StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    }
}
