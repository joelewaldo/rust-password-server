use rust_password_server::bounded_context::infrastructure::db::postgres_db::*;
use rust_password_server::bounded_context::domain::{ password::Password, password_db::PasswordDb };
use sqlx::postgres::PgPoolOptions;
use sqlx::{PgPool, query};
use uuid::Uuid;
use chrono::Utc;
use rust_password_server::bounded_context::infrastructure::config::app_config;
use chrono::{DateTime, Duration};

async fn create_test_db() -> Result<Database, sqlx::Error> {
    let config = app_config::load_config();
    let connection_str = &config.test_db_url;
    Database::new(connection_str, 1).await
}

async fn setup_db(database: &Database) {
    query(
        r#"
        CREATE TABLE IF NOT EXISTS passwords (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            service TEXT NOT NULL,
            nonce TEXT NOT NULL,
            cipher TEXT NOT NULL,
            created_at TIMESTAMPTZ DEFAULT NOW() NOT NULL,
            updated_at TIMESTAMPTZ DEFAULT NOW() NOT NULL
        )
        "#,
    )
    .execute(database.get_pool())
    .await
    .expect("Failed to setup the database");
}

fn assert_datetime_approx_eq(left: DateTime<Utc>, right: DateTime<Utc>, tolerance: Duration) {
    let diff = (left - right).abs();
    assert!(diff <= tolerance, "Timestamps differ by more than the allowed tolerance");
}

#[tokio::test]
async fn test_save_and_get_password() {
    let mut database = create_test_db().await.expect("Failed to create test db");
    setup_db(&database).await;

    let test_password = Password {
        id: Uuid::new_v4(),
        service: "test_service".to_string(),
        nonce: "test_nonce".to_string(),
        cipher: "test_cipher".to_string(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    database.save(test_password.clone()).await.expect("Failed to save password");

    let retrieved_password = database
        .get_by_id(test_password.id)
        .await
        .expect("Failed to retrieve password");

    assert_eq!(test_password.id, retrieved_password.id);
    assert_eq!(test_password.service, retrieved_password.service);
    assert_eq!(test_password.nonce, retrieved_password.nonce);
    assert_eq!(test_password.cipher, retrieved_password.cipher);

    assert!(retrieved_password.created_at <= Utc::now());
    assert!(retrieved_password.updated_at <= Utc::now());
}

#[tokio::test]
async fn test_get_by_id() {
    let mut database = create_test_db().await.expect("Failed to create test db");
    setup_db(&database).await;

    let test_password = Password {
        id: Uuid::new_v4(),
        service: "test_service".to_string(),
        nonce: "test_nonce".to_string(),
        cipher: "test_cipher".to_string(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    database
        .save(test_password.clone())
        .await
        .expect("Failed to save password");

    let retrieved_password = database
        .get_by_id(test_password.id)
        .await
        .expect("Failed to retrieve password");

    assert_eq!(test_password.id, retrieved_password.id);
    assert_eq!(test_password.service, retrieved_password.service);
    assert_eq!(test_password.nonce, retrieved_password.nonce);
    assert_eq!(test_password.cipher, retrieved_password.cipher);
    
    let tolerance = Duration::milliseconds(1);
    assert_datetime_approx_eq(test_password.created_at, retrieved_password.created_at, tolerance);
    assert_datetime_approx_eq(test_password.updated_at, retrieved_password.updated_at, tolerance);

    let non_existent_id = Uuid::new_v4();
    let result = database.get_by_id(non_existent_id).await;

    assert!(result.is_err(), "Expected an error for non-existent password");
}

#[tokio::test]
async fn test_delete_password() {
    let mut database = create_test_db().await.expect("Failed to create test db");
    setup_db(&database).await;

    let test_password = Password {
        id: Uuid::new_v4(),
        service: "test_service".to_string(),
        nonce: "test_nonce".to_string(),
        cipher: "test_cipher".to_string(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    database.save(test_password.clone()).await.expect("Failed to save password");

    database
        .delete(test_password.id)
        .await
        .expect("Failed to delete password");

    let result = database.get_by_id(test_password.id).await;
    assert!(result.is_err(), "Password should be deleted");
}