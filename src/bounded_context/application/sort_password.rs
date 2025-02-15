use crate::bounded_context::domain::{password_db::PasswordDb, password::Password, password_db::SortBy};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SortPasswordError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] Box<dyn std::error::Error>),
}

pub struct SortPassword {
    password_db: Box<dyn PasswordDb>,
}

impl SortPassword {
    pub fn new(password_db: Box<dyn PasswordDb>) -> Self {
        SortPassword { password_db }
    }

    pub async fn execute(
        &mut self,
        sort_by: SortBy,
    ) -> Result<Vec<Password>, SortPasswordError> {
        let passwords = self.password_db.list_sorted(&sort_by).await.map_err(SortPasswordError::DatabaseError)?;
        Ok(passwords)
    }
}