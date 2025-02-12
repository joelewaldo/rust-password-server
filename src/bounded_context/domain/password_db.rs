use super::password::Password;
use uuid::Uuid;
use async_trait::async_trait;
use std::error::Error;

#[async_trait]
pub trait PasswordDb {
    async fn save(&mut self, password: Password) -> Result<(), Box<dyn Error>>;
    async fn get_by_id(&mut self, id: Uuid) -> Result<Password, Box<dyn Error>>;
}