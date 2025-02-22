#[derive(Clone)]
pub struct AppConfig {
    pub host: String,
    pub port: u64,
    pub db_url: String,
    pub test_db_url: String,
    pub max_connections: u32,
    pub log_level: String,
    pub graceful_shutdown_time: u64,
    
    pub pagination_default_size: u32,
    pub pagination_max_size: u32,
}

impl Default for AppConfig {
    fn default() -> Self {
        load_config()
    }
}

pub fn load_config() -> AppConfig {
    dotenvy::dotenv().ok();

    let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string()).parse().unwrap_or(3000);
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let test_db_url = std::env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must be set");
    let max_connections = std::env::var("MAX_CONNECTIONS").unwrap_or_else(|_| "16".to_string()).parse().unwrap_or(16);
    let log_level = std::env::var("LOG_LEVEL").unwrap_or_else(|_| "debug".to_string());
    let graceful_shutdown_time = std::env::var("GRACEFUL_SHUTDOWN_TIME").unwrap_or_else(|_| "10".to_string()).parse().unwrap_or(10);

    let pagination_default_size = std::env::var("PAGINATION_DEFAULT_SIZE").unwrap_or_else(|_| "20".to_string()).parse().unwrap_or(20);
    let pagination_max_size = std::env::var("PAGINATION_MAX_SIZE").unwrap_or_else(|_| "20".to_string()).parse().unwrap_or(20);

    AppConfig { host, port, db_url, test_db_url, max_connections, log_level, graceful_shutdown_time, pagination_default_size, pagination_max_size }
}