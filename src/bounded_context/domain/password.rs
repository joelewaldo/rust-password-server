use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Clone)]
pub struct Password {
    pub id: Uuid,
    pub service: String,
    pub encrypted_password: String,
    pub salt: Vec<u8>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>
}