use crate::bounded_context::infrastructure::{http::run_server, config::app_config};

mod bounded_context;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = app_config::load_config();
    run_server::run_server(config).await;

    Ok(())
}
