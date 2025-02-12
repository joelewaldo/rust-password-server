use crate::bounded_context::domain::{password_db::PasswordDb, password::Password};
use uuid::Uuid;
use thiserror::Error;

#[derive(Debug, PartialEq, Clone)]
pub struct DeletePasswordInput {
    pub id: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct DeletePasswordOutput {
    pub id: String,
}

#[derive(Debug, Error)]
pub enum DeletePasswordError {
    #[error("Password not found.")]
    PasswordNotFound,

    #[error("Database error: {0}")]
    DatabaseError(#[from] Box<dyn std::error::Error>),
}

pub struct DeletePassword {
    password_db: Box<dyn PasswordDb>,
}

impl DeletePassword {
    pub fn new(password_db: Box<dyn PasswordDb>) -> Self {
        DeletePassword { password_db }
    }

    pub async fn execute(
        &mut self,
        input: DeletePasswordInput,
    ) -> Result<DeletePasswordOutput, DeletePasswordError> {
        let id = Uuid::parse_str(&input.id).map_err(|_| DeletePasswordError::PasswordNotFound)?;

        self.password_db.delete(id).await.map_err(DeletePasswordError::DatabaseError)?;

        Ok(DeletePasswordOutput {
            id: input.id,
        })
    }
}