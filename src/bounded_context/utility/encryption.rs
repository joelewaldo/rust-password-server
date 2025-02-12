use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Nonce, Key
};

use hex;

const NONCE_SIZE: usize = 12;
const MASTER_KEY_SIZE: usize = 32;

pub fn generate_key() -> String {
    let key_bytes: [u8; MASTER_KEY_SIZE] = Aes256Gcm::generate_key(OsRng).into();

    hex::encode(key_bytes)
}

pub fn encrypt(master_key: &str, password: String) -> (String, String) {
    let key_bytes = hex::decode(master_key).expect("Invalid Master Key Hex");
    assert_eq!(key_bytes.len(), MASTER_KEY_SIZE);

    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);

    // Random Nonce
    let nonce_bytes: [u8; NONCE_SIZE] = Aes256Gcm::generate_nonce(OsRng).into();
    let nonce = Nonce::from_slice(&nonce_bytes); 
    let cipher_text = cipher.encrypt(nonce, password.as_bytes()).expect("Failed to Encrypt");

    (hex::encode(nonce_bytes), hex::encode(cipher_text))    
}

pub fn decrypt(master_key: &str, nonce_hex: &str, cipher_hex: &str) -> String {
    let key_bytes = hex::decode(master_key).expect("Invalid Master Key Hex");
    assert_eq!(key_bytes.len(), MASTER_KEY_SIZE);

   let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
   let cipher = Aes256Gcm::new(key);

   let nonce_bytes = hex::decode(nonce_hex).expect("Invalid Nonce Hex");
   let nonce = Nonce::from_slice(&nonce_bytes);
   let cipher_text = hex::decode(cipher_hex).expect("Invalid Cipher Hex");

   let password = cipher.decrypt(nonce, cipher_text.as_ref()).expect("Decryption Failed");
   
   String::from_utf8(password).expect("Invalid UTF-8")
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex;

    #[test]
    fn test_generate_key() {
        let key = generate_key();
        assert_eq!(key.len(), MASTER_KEY_SIZE * 2);
    }

    #[test]
    fn test_encrypt_decrypt() {
        let master_key = generate_key();
        let password = "super_secure_password".to_string();
        
        let (nonce, cipher_text) = encrypt(&master_key, password.clone());
        let decrypted_password = decrypt(&master_key, &nonce, &cipher_text);
        
        assert_eq!(password, decrypted_password);
    }

    #[test]
    fn test_decrypt_with_wrong_key() {
        let master_key = generate_key();
        let wrong_key = generate_key();
        let password = "super_secure_password".to_string();
        
        let (nonce, cipher_text) = encrypt(&master_key, password);
        let result = std::panic::catch_unwind(|| decrypt(&wrong_key, &nonce, &cipher_text));
        
        assert!(result.is_err());
    }

    #[test]
    fn test_decrypt_with_modified_cipher_text() {
        let master_key = generate_key();
        let password = "super_secure_password".to_string();
        
        let (nonce, mut cipher_text) = encrypt(&master_key, password);
        cipher_text.pop();
        
        let result = std::panic::catch_unwind(|| decrypt(&master_key, &nonce, &cipher_text));
        
        assert!(result.is_err());
    }
}
