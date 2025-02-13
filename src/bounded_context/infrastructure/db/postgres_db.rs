use sqlx::postgres::PgPoolOptions;
use sqlx::{PgPool, query, query_as};
use uuid::Uuid;
use async_trait::async_trait;

use crate::bounded_context::domain::{password::Password, password_db::PasswordDb, password_db::SortBy};

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

    async fn search_by_service(
        &mut self,
        search_term: &str,
    ) -> Result<Vec<Password>, Box<dyn std::error::Error>> {
        let search_pattern = format!("%{}%", search_term);
        let passwords = query_as(
            r#"
            SELECT id, service, nonce, cipher, created_at, updated_at
            FROM passwords
            WHERE service ILIKE $1
            "#,
        )
        .bind(search_pattern)
        .fetch_all(&self.pool)
        .await?;

        Ok(passwords)
    }

    async fn list_sorted(
        &mut self,
        sort_by: &SortBy,
    ) -> Result<Vec<Password>, Box<dyn std::error::Error>> {
        let order_clause = match sort_by {
            SortBy::CreatedAtAsc => "created_at ASC",
            SortBy::CreatedAtDesc => "created_at DESC",
            SortBy::UpdatedAtAsc => "updated_at ASC",
            SortBy::UpdatedAtDesc => "updated_at DESC",
        };

        let query_str = format!(
            r#"
            SELECT id, service, nonce, cipher, created_at, updated_at
            FROM passwords
            ORDER BY {}
            "#,
            order_clause
        );

        let passwords = query_as(&query_str)
            .fetch_all(&self.pool)
            .await?;

        Ok(passwords)
    }
}