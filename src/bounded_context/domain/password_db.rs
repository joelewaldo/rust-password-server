use super::password::Password;
use uuid::Uuid;
use async_trait::async_trait;
use std::error::Error;

#[derive(Debug)]
pub enum SortBy {
    CreatedAtAsc,
    CreatedAtDesc,
    UpdatedAtAsc,
    UpdatedAtDesc,
}

#[async_trait]
pub trait PasswordDb {
    async fn save(&mut self, password: Password) -> Result<(), Box<dyn Error>>;
    async fn get_by_id(&mut self, id: Uuid) -> Result<Password, Box<dyn Error>>;
    async fn delete(&mut self, id: Uuid) -> Result<(), Box<dyn Error>>;

    async fn search_by_service(
        &mut self,
        search_term: &str,
    ) -> Result<Vec<Password>, Box<dyn std::error::Error>>;

    async fn list_sorted(
        &mut self,
        sort_by: &SortBy,
    ) -> Result<Vec<Password>, Box<dyn std::error::Error>>;
}
