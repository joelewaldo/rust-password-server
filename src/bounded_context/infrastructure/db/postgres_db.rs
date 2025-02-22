use sqlx::postgres::PgPoolOptions;
use sqlx::{PgPool, query, query_as};
use sqlx::pool::PoolConnection;
use sqlx::Postgres;
use uuid::Uuid;
use async_trait::async_trait;
use std::sync::Arc;
use crate::bounded_context::domain::{password::Password, password_db::PasswordDb, password_db::SortBy};
use crate::bounded_context::infrastructure::config::app_config::AppConfig;

#[derive(Clone)]
pub struct Database {
    pool: Arc<PgPool>,
    pub config: AppConfig,
}

impl Database {
    /// Creates a new database connection pool
    pub async fn new(connection: &str, max_connections: u32, config: AppConfig) -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(max_connections)
            .connect(connection)
            .await?;
        Ok(Self { pool: Arc::new(pool), config })
    }

    /// Get a reference to the connection pool
    pub fn get_pool(&self) -> &PgPool {
        &self.pool
    }

    /// Acquire a new database connection for transactional queries
    pub async fn get_connection(&self) -> Result<PoolConnection<Postgres>, sqlx::Error> {
        self.pool.acquire().await
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
        .bind(password.id)
        .bind(password.service)
        .bind(password.nonce)
        .bind(password.cipher)
        .bind(password.created_at)
        .bind(password.updated_at)
        .execute(&*self.pool)
        .await?;

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
        .fetch_optional(&*self.pool)
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
        .execute(&*self.pool)
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
        page: u32,
        page_size: u32,
    ) -> Result<Vec<Password>, Box<dyn std::error::Error>> {
        let search_pattern = format!("%{}%", search_term);
    
        let offset = (page - 1) * page_size;
    
        let passwords = query_as(
            r#"
            SELECT id, service, nonce, cipher, created_at, updated_at
            FROM passwords
            WHERE service ILIKE $1
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(search_pattern)
        .bind(page_size as i64)
        .bind(offset as i64)
        .fetch_all(&*self.pool)
        .await?;
    
        Ok(passwords)
    }

    async fn list_sorted(
        &mut self,
        sort_by: &SortBy,
        page: u32,
        page_size: u32,
    ) -> Result<Vec<Password>, Box<dyn std::error::Error>> {
        let order_clause = match sort_by {
            SortBy::CreatedAtAsc => "created_at ASC",
            SortBy::CreatedAtDesc => "created_at DESC",
            SortBy::UpdatedAtAsc => "updated_at ASC",
            SortBy::UpdatedAtDesc => "updated_at DESC",
        };
    
        let offset = (page - 1) * page_size;
    
        let query_str = format!(
            r#"
            SELECT id, service, nonce, cipher, created_at, updated_at
            FROM passwords
            ORDER BY {}
            LIMIT $1 OFFSET $2
            "#,
            order_clause
        );
    
        let passwords = query_as(&query_str)
            .bind(page_size as i64)
            .bind(offset as i64)
            .fetch_all(&*self.pool)
            .await?;
    
        Ok(passwords)
    }
}
