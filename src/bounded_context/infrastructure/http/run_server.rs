use std::time::Duration;

use axum::{
    Router,
};
use tracing::{info, error};
use tower_http::timeout::TimeoutLayer;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{
    layer::SubscriberExt, 
    util::SubscriberInitExt
};

use crate::bounded_context::infrastructure::{
    http::configure_routes::configure_routes, 
    http::shutdown::shutdown_signal,
    config::app_config::AppConfig, 
    db::postgres_db::Database
};

pub async fn run_server(config: AppConfig) {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!(
                    "{}={},tower_http=debug,axum=trace", 
                    env!("CARGO_CRATE_NAME"), 
                    config.log_level
                )
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer().without_time())
        .init();

    let database = Database::new(&config.db_url, config.max_connections).await.expect("Failed to connect to db.");

    let app = Router::new().nest("/api", configure_routes(database)).layer((
        TraceLayer::new_for_http(),
        TimeoutLayer::new(Duration::from_secs(config.graceful_shutdown_time)),
    ));

    let listener = tokio::net::TcpListener::bind(&format!("{}:{}", config.host, config.port).parse::<std::net::SocketAddr>().unwrap())
        .await
        .unwrap();

    info!("Listening on {}", listener.local_addr().unwrap());

    if let Err(e) = axum::serve(listener, app).with_graceful_shutdown(shutdown_signal())
        .await
    {
        error!("Server failed to start: {}", e);
    }
}