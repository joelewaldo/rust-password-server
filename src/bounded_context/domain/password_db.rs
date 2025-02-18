use super::password::Password;
use uuid::Uuid;
use async_trait::async_trait;
use std::error::Error;
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SortByError {
    #[error("Invalid sort option: {0}")]
    InvalidOption(String),
}

#[derive(Debug)]
pub enum SortBy {
    CreatedAtAsc,
    CreatedAtDesc,
    UpdatedAtAsc,
    UpdatedAtDesc,
}

impl FromStr for SortBy {
    type Err = SortByError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "CreatedAtAsc" => Ok(SortBy::CreatedAtAsc),
            "CreatedAtDesc" => Ok(SortBy::CreatedAtDesc),
            "UpdatedAtAsc" => Ok(SortBy::UpdatedAtAsc),
            "UpdatedAtDesc" => Ok(SortBy::UpdatedAtDesc),
            _ => Err(SortByError::InvalidOption(s.to_string())),
        }
    }
}

#[async_trait]
pub trait PasswordDb {
    async fn save(&mut self, password: Password) -> Result<(), Box<dyn Error>>;
    async fn get_by_id(&mut self, id: Uuid) -> Result<Password, Box<dyn Error>>;
    async fn delete(&mut self, id: Uuid) -> Result<(), Box<dyn Error>>;

    async fn search_by_service(
        &mut self,
        search_term: &str,
        page: u32,
        page_size: u32,
    ) -> Result<Vec<Password>, Box<dyn std::error::Error>>;

    async fn list_sorted(
        &mut self,
        sort_by: &SortBy,
        page: u32,
        page_size: u32,
    ) -> Result<Vec<Password>, Box<dyn std::error::Error>>;
}
