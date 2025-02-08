use crate::bounded_context::infrastructure::{http::run_server, config::app_config};

mod bounded_context;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // dotenvy::dotenv().ok();

    // let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    // println!("{}", database_url);
    // let (client, connection) = tokio_postgres::connect(&database_url, NoTls).await?;

    // tokio::spawn(async move {
    //     if let Err(e) = connection.await {
    //         eprintln!("Database connection error: {}", e);
    //     }
    // });

    // println!("Connected to the database successfully.");

    // Ok(())

    let config = app_config::load_config();
    run_server::run_server(config).await;

    Ok(())
}
