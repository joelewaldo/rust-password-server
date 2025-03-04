use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct StatusResponse {
    healthy: bool,
    version: &'static str,
}

pub async fn status_handler() -> Json<StatusResponse> {
    Json(StatusResponse {
        healthy: true,
        version: "1.0.0",
    })
}