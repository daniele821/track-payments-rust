use crate::error::{Error, Result};
use aes_gcm::aead::{Aead, AeadCore, KeyInit, OsRng};
use aes_gcm::{Aes256Gcm, Key, Nonce};

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

pub fn encrypt_str(key_str: &[u8], plaintext: &str) -> Result<Vec<u8>> {
    encrypt(key_str, plaintext.as_bytes())
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

pub fn decrypt_str(key_str: &[u8], encrypted_data: &[u8]) -> Result<String> {
    String::from_utf8(decrypt(key_str, encrypted_data)?).map_err(Error::from_generic)
}

#[cfg(test)]
mod tests {
    use super::{decrypt, decrypt_str, encrypt, encrypt_str};

    #[test]
    fn encryption() {
        let plain_text = "Random text which i want to cipher!".as_bytes();
        let key_str = "12345678901234567890123456789012".as_bytes();
        let cipher_text = encrypt(key_str, plain_text).unwrap();
        let decipher_text = decrypt(key_str, &cipher_text).unwrap();
        assert_eq!(plain_text, decipher_text);
    }

    #[test]
    fn encryption_str() {
        let plain_text = "Random text which i want to cipher!";
        let key_str = "12345678901234567890123456789012".as_bytes();
        let cipher_text = encrypt_str(key_str, plain_text).unwrap();
        let decipher_text = decrypt_str(key_str, &cipher_text).unwrap();
        assert_eq!(plain_text, decipher_text);
    }
}
