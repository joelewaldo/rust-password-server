use crate::bounded_context::utility::encryption::{is_valid_cipher, is_valid_nonce};
use crate::bounded_context::domain::{password_db::PasswordDb, password::Password};
use uuid::Uuid;
use thiserror::Error;

#[derive(Debug, PartialEq, Clone)]
pub struct CreatePasswordInput {
    pub service: String,
    pub nonce: String,
    pub cipher: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CreatePasswordOutput {
    pub id: String,
    pub service: String,
    pub nonce: String,
    pub cipher: String,
}

#[derive(Debug, Error)]
pub enum CreatePasswordError {
    #[error("Invalid nonce provided.")]
    InvalidNonce,
    
    #[error("Invalid cipher provided.")]
    InvalidCipher,
    
    #[error("Database error: {0}")]
    DatabaseError(#[from] Box<dyn std::error::Error>),
}

pub struct CreatePassword {
    password_db: Box<dyn PasswordDb>,
}

impl CreatePassword {
    pub fn new(password_db: Box<dyn PasswordDb>) -> Self {
        CreatePassword { password_db }
    }

    pub async fn execute(
        &mut self,
        input: CreatePasswordInput,
    ) -> Result<CreatePasswordOutput, CreatePasswordError> {
        if !is_valid_nonce(&input.nonce) {
            return Err(CreatePasswordError::InvalidNonce);
        }

        if !is_valid_cipher(&input.cipher) {
            return Err(CreatePasswordError::InvalidCipher);
        }

        let id = Uuid::new_v4();
        let password = Password::new(id, input.service.clone(), input.nonce.clone(), input.cipher.clone());
        self.password_db.save(password).await.map_err(CreatePasswordError::DatabaseError)?;

        Ok(CreatePasswordOutput {
            id: id.to_string(),
            service: input.service,
            nonce: input.nonce,
            cipher: input.cipher,
        })
    }
}