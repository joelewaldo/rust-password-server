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
    assert_eq!(master_key.len(), MASTER_KEY_SIZE);

    let key_bytes = master_key.as_bytes();
    let key = Key::<Aes256Gcm>::from_slice(key_bytes);
    let cipher = Aes256Gcm::new(key);

    // Random Nonce
    let nonce_bytes: [u8; NONCE_SIZE] = Aes256Gcm::generate_nonce(OsRng).into();
    let nonce = Nonce::from_slice(&nonce_bytes); 
    let cipher_text = cipher.encrypt(nonce, password.as_bytes()).expect("Failed to Encrypt");

    (hex::encode(nonce_bytes), hex::encode(cipher_text))    
}

pub fn decrypt(master_key: &str, nonce_hex: &str, cipher_hex: &str) -> String {
   assert_eq!(master_key.len(), MASTER_KEY_SIZE);

   let key_bytes = master_key.as_bytes();
   let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
   let cipher = Aes256Gcm::new(key);

   let nonce_bytes = hex::decode(nonce_hex).expect("Invalid Nonce Hex");
   let nonce = Nonce::from_slice(&nonce_bytes);
   let cipher_text = hex::decode(cipher_hex).expect("Invalid Cipher Hex");

   let password = cipher.decrypt(nonce, cipher_text.as_ref()).expect("Decryption Failed");
   
   String::from_utf8(password).expect("Invalid UTF-8")
}
