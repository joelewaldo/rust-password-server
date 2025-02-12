use sqlx::postgres::PgPoolOptions;
use sqlx::{PgPool, query, query_as};
use uuid::Uuid;
use async_trait::async_trait;
use chrono::{DateTime, Utc};

use crate::bounded_context::domain::{password::Password, password_db::PasswordDb};

pub struct Database {
    pool: PgPool,
}

impl Database {
    pub async fn new(connection: &str, max_connections: u32) -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(max_connections)
            .connect(connection)
            .await?;
        Ok(Self { pool })
    }
}

#[async_trait]
impl PasswordDb for Database {
    async fn save(&mut self, password: Password) -> Result<(), Box<dyn std::error::Error>> {
        query(
            r#"
            INSERT INTO passwords (id, service, nonce, cipher, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
        )
        .bind(password.id)           // ID as Uuid
        .bind(password.service)      // Service
        .bind(password.nonce)        // Nonce
        .bind(password.cipher)       // Cipher
        .bind(password.created_at)
        .bind(password.updated_at)
        .execute(&self.pool)
        .await?; // Propagate any errors

        Ok(())
    }

    async fn get_by_id(&mut self, id: Uuid) -> Result<Password, Box<dyn std::error::Error>> {
        let result: Option<Password> = query_as(
            r#"
            SELECT id, service, nonce, cipher, created_at, updated_at
            FROM passwords
            WHERE id = $1
            "#
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        match result {
            Some(password) => Ok(password),
            None => Err(Box::new(sqlx::Error::RowNotFound)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::PgPool;
    use uuid::Uuid;
    use chrono::Utc;
    use crate::bounded_context::infrastructure::config::app_config;

    async fn create_test_db() -> Result<PgPool, sqlx::Error> {
        let config = app_config::load_config();
        let connection_str = &config.test_db_url;
        PgPoolOptions::new()
            .max_connections(1)
            .connect(connection_str)
            .await
    }

    async fn setup_db(pool: &PgPool) {
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
        .execute(pool)
        .await
        .expect("Failed to setup the database");
    }

    #[tokio::test]
    async fn test_save_and_get_password() {
        let pool = create_test_db().await.expect("Failed to create test db pool");

        setup_db(&pool).await;

        let mut database = Database { pool };

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
}