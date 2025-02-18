use crate::bounded_context::infrastructure::http::index_controller::index;
use crate::bounded_context::application::{
    create_password::create_password,
    delete_password::delete_password,
    search_password::search_password,
    sort_password::sort_passwords,
};
use crate::bounded_context::infrastructure::db::postgres_db::Database;

use axum::{
    routing::{get, post},
    Router
};

pub fn configure_routes(database: Database) -> Router {
    Router::new()
        .route("/test", get(index))
        .route("/password/search", get(search_password))
        .route("/password/passwords", get(sort_passwords))
        .route("/password/create", post(create_password))
        .route("/password/delete", post(delete_password))
        .with_state(database)
}