use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::bounded_context::utility::encryption::{self, decrypt};

use sqlx::FromRow;
use sqlx::postgres::PgRow;
use sqlx::Row;

#[derive(Clone, Debug, PartialEq)]
pub struct Password {
    pub id: Uuid,
    pub service: String,
    pub nonce: String,
    pub cipher: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>
}

impl Password {
    pub fn new(id: Uuid, service: String, nonce: String, cipher: String) -> Password {
        let now = Utc::now();
        Password {
            id,
            service,
            nonce,
            cipher,
            created_at: now,
            updated_at: now,
        }
    }
}

impl<'r> FromRow<'r, PgRow> for Password {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(Password {
            id: row.get("id"),
            service: row.get("service"),
            nonce: row.get("nonce"),
            cipher: row.get("cipher"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }
}