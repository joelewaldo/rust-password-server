use sqlx::postgres::PgPoolOptions;
use sqlx::{PgPool, query, query_as};
use uuid::Uuid;
use async_trait::async_trait;

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

    pub fn get_pool(&self) -> &PgPool {
        &self.pool
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

    async fn delete(&mut self, id: Uuid) -> Result<(), Box<dyn std::error::Error>> {
        let rows_affected = query(
            r#"
            DELETE FROM passwords
            WHERE id = $1
            "#,
        )
        .bind(id)
        .execute(&self.pool)
        .await?
        .rows_affected();

        if rows_affected == 0 {
            Err(Box::new(sqlx::Error::RowNotFound))
        } else {
            Ok(())
        }
    }
}