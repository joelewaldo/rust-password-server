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
        .nest("/password", 
        Router::new()
            .route("/search", get(search_password))
            .route("/passwords", get(sort_passwords))
            .route("/create", post(create_password))
            .route("/delete", post(delete_password))
            .with_state(database)
        )
}