use crate::bounded_context::domain::{password_db::PasswordDb, password::Password, password_db::SortBy};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SearchPasswordError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] Box<dyn std::error::Error>),
}

pub struct SearchPassword {
    password_db: Box<dyn PasswordDb>,
}

impl SearchPassword {
    pub fn new(password_db: Box<dyn PasswordDb>) -> Self {
        SearchPassword { password_db }
    }

    pub async fn execute(
        &mut self,
        search_term: &str,
    ) -> Result<Vec<Password>, SearchPasswordError> {
        let passwords = self.password_db.search_by_service(search_term).await.map_err(SearchPasswordError::DatabaseError)?;
        Ok(passwords)
    }
}
