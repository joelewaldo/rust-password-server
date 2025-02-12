use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::bounded_context::utility::encryption::{self, decrypt};

#[derive(Clone)]
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

    pub fn decrypt_password(&self, master_key: String) -> String {
        decrypt(&master_key, &self.nonce, &self.cipher)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bounded_context::utility::encryption::{encrypt, generate_key};
    use uuid::{Uuid, uuid};
    const ID: Uuid = uuid!("00000000-0000-0000-0000-000000000001");

    #[test]
    fn test_password_creation() {
        let service = "example_service".to_string();
        let nonce = "random_nonce".to_string();
        let cipher = "encrypted_data".to_string();

        let password = Password::new(ID, service.clone(), nonce.clone(), cipher.clone());

        assert_eq!(password.id, ID);
        assert_eq!(password.service, service);
        assert_eq!(password.nonce, nonce);
        assert_eq!(password.cipher, cipher);
    }

    #[test]
    fn test_password_decryption() {
        let master_key = generate_key();
        let plaintext_password = "my_secure_password".to_string();

        let (nonce, cipher) = encrypt(&master_key, plaintext_password.clone());

        let password = Password::new(ID, "example_service".to_string(), nonce.clone(), cipher.clone());

        let decrypted_password = password.decrypt_password(master_key);
        assert_eq!(decrypted_password, plaintext_password);
    }

    #[test]
    fn test_password_decryption_static_key() {
        let master_key = "3d93f9d51efb1786ec11f0e40c7bd75c79ab4969cc6aa4aa31ae40667ef5ac52".to_string();
        let plaintext_password = "my_secure_password".to_string();

        let (nonce, cipher) = encrypt(&master_key, plaintext_password.clone());

        let password = Password::new(ID, "example_service".to_string(), nonce.clone(), cipher.clone());

        let decrypted_password = password.decrypt_password(master_key);
        assert_eq!(decrypted_password, plaintext_password);
    }
}