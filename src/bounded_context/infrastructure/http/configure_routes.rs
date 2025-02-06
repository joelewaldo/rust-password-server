use crate::bounded_context::infrastructure::http::{
    index_controller::index
};
use axum::{
    routing::{get, post},
    Router
};

pub fn configure_routes() -> Router {
    Router::new()
        .route("/test", get(index))
}