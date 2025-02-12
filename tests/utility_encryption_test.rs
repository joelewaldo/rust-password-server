use rust_password_server::bounded_context::utility::encryption::*;

const NONCE_SIZE: usize = 12;
const MASTER_KEY_SIZE: usize = 32;

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

#[test]
fn test_valid_masterkey() {
    let master_key = generate_key();
    assert!(is_valid_masterkey(&master_key));
}

#[test]
fn test_invalid_masterkey_length() {
    let invalid_key = "1234567890abcdef";
    assert!(!is_valid_masterkey(invalid_key));
}

#[test]
fn test_invalid_masterkey_hex() {
    let invalid_key = "zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzz";
    assert!(!is_valid_masterkey(invalid_key));
}

#[test]
fn test_encrypt_decrypt_with_fixed_masterkey() {
    let master_key = "a3f1b2c4d5e6f7890123456789abcdef0123456789abcdef0123456789abcdea";
    let password = "super_secure_password".to_string();

    assert!(is_valid_masterkey(master_key));

    let (nonce, cipher_text) = encrypt(master_key, password.clone());
    let decrypted_password = decrypt(master_key, &nonce, &cipher_text);

    assert_eq!(password, decrypted_password);
}

#[test]
fn test_encrypt_decrypt_with_custom_valid_masterkey() {
    let master_key = "abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789";
    let password = "super_secure_password".to_string();

    assert!(is_valid_masterkey(master_key));

    let (nonce, cipher_text) = encrypt(master_key, password.clone());
    let decrypted_password = decrypt(master_key, &nonce, &cipher_text);

    assert_eq!(password, decrypted_password);
}

#[test]
fn test_valid_cipher() {
    let master_key = generate_key();
    let password = "test_password".to_string();
    let (_nonce, cipher_hex) = encrypt(&master_key, password);
    assert!(is_valid_cipher(&cipher_hex));
}

#[test]
fn test_invalid_cipher_hex() {
    let invalid_cipher = "zzzzzzzz";
    assert!(!is_valid_cipher(invalid_cipher));
}

#[test]
fn test_invalid_cipher_length() {
    let small_cipher = hex::encode(&[0u8; 15]);
    assert!(!is_valid_cipher(&small_cipher));
}

#[test]
fn test_valid_nonce() {
    let master_key = generate_key();
    let password = "test_password".to_string();
    let (nonce, _cipher_hex) = encrypt(&master_key, password);
    assert!(is_valid_nonce(&nonce));
}

#[test]
fn test_invalid_nonce_hex() {
    let invalid_nonce = "zzzzzzzzzzzz";
    assert!(!is_valid_nonce(invalid_nonce));
}

#[test]
fn test_invalid_nonce_length() {
    let smaller_nonce = hex::encode(&[0u8; 11]);
    assert!(!is_valid_nonce(&smaller_nonce));
}