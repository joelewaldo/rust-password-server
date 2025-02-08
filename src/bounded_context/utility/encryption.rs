use aes_gcm::Aes256Gcm;
use cipher::KeyInit;
use pbkdf2::pbkdf2;

fn generate_key(master_key: &[u8], salt: &[u8]) -> Aes256Gcm {
    let mut cipher = Aes256Gcm::new(&[]);
    pbkdf2::<_, _, sha2::Sha256>(master_key, salt, 100_000, &mut cipher);
    cipher
}

fn encrypt(cipher: &Aes256Gcm, plaintext: &[u8]) -> Vec<u8> {
    let nonce = cipher.nonce();
    cipher.encrypt(nonce, plaintext)
}

fn decrypt(cipher: &Aes256Gcm, ciphertext: &[u8]) -> Result<Vec<u8>, aes_gcm::Error> {
    let nonce = cipher.nonce();
    cipher.decrypt(nonce, ciphertext)
}