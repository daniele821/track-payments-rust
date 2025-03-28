//! Crypto related utility functions.

#![allow(unused)]

use aes_gcm::{
    Aes256Gcm, Key, Nonce,
    aead::{Aead, AeadCore, KeyInit, OsRng},
};

/// Encrypt plaintext.
#[must_use]
pub fn encrypt(key_str: &str, plaintext: &str) -> Option<String> {
    let key = Key::<Aes256Gcm>::from_slice(key_str.as_bytes());
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let cipher = Aes256Gcm::new(key);
    let ciphered_data = cipher.encrypt(&nonce, plaintext.as_bytes()).ok()?;
    let mut encrypted_data: Vec<u8> = nonce.to_vec();
    encrypted_data.extend_from_slice(&ciphered_data);
    Some(hex::encode(encrypted_data))
}

/// Decrypt text encrypted.
///
/// # Panics
///
/// Panics if input string is encrypted incorrectly, or for internal failures.
#[must_use]
pub fn decrypt(key_str: &str, encrypted_data: &str) -> Option<String> {
    let encrypted_data = hex::decode(encrypted_data).expect("failed to decode hex string into vec");
    let key = Key::<Aes256Gcm>::from_slice(key_str.as_bytes());
    let (nonce_arr, ciphered_data) = encrypted_data.split_at(12);
    let nonce = Nonce::from_slice(nonce_arr);
    let cipher = Aes256Gcm::new(key);
    let plaintext = cipher.decrypt(nonce, ciphered_data).ok()?;
    Some(String::from_utf8(plaintext).expect("failed to convert vector of bytes to string"))
}

#[cfg(test)]
mod tests {
    use super::{decrypt, encrypt};

    #[test]
    fn encryption() {
        let plain_text = "Random text which i want to cipher!";
        let key_str = "12345678901234567890123456789012";
        let cipher_text = encrypt(key_str, plain_text).unwrap();
        let decipher_text = decrypt(key_str, &cipher_text).unwrap();
        assert_eq!(plain_text, decipher_text);
    }
}
