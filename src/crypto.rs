use aes_gcm::aead::{Aead, AeadCore, KeyInit, OsRng};
use aes_gcm::{Aes256Gcm, Key, Nonce};

use crate::error::{Error, Result};

pub fn encrypt(key_str: &[u8], plaintext: &[u8]) -> Result<Vec<u8>> {
    let key = Key::<Aes256Gcm>::from_slice(key_str);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let cipher = Aes256Gcm::new(key);
    let ciphered_data = cipher
        .encrypt(&nonce, plaintext)
        .map_err(|_| Error::EncryptionFailed)?;
    let mut encrypted_data: Vec<u8> = nonce.to_vec();
    encrypted_data.extend_from_slice(&ciphered_data);
    Ok(encrypted_data)
}

pub fn decrypt(key_str: &[u8], encrypted_data: &[u8]) -> Result<Vec<u8>> {
    let key = Key::<Aes256Gcm>::from_slice(key_str);
    let (nonce_arr, ciphered_data) = encrypted_data.split_at(12);
    let nonce = Nonce::from_slice(nonce_arr);
    let cipher = Aes256Gcm::new(key);
    cipher
        .decrypt(nonce, ciphered_data)
        .map_err(|_| Error::DecryptionFailed)
}

#[cfg(test)]
mod tests {
    use super::{decrypt, encrypt};

    #[test]
    fn encryption() {
        let plain_text = "Random text which i want to cipher!".as_bytes();
        let key_str = "12345678901234567890123456789012".as_bytes();
        let cipher_text = encrypt(key_str, plain_text).unwrap();
        let decipher_text = decrypt(key_str, &cipher_text).unwrap();
        assert_eq!(plain_text, decipher_text);
    }
}
