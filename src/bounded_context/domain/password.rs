use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Clone)]
pub struct Password {
    pub id: Uuid,
    pub service: String,
    pub nonce: String,
    pub cipher: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>
}
