use aes_gcm::aead::{Aead, AeadCore, KeyInit, OsRng};
use aes_gcm::{Aes256Gcm, Key, Nonce};

pub fn encrypt(key_str: &[u8], plaintext: &[u8]) -> Option<Vec<u8>> {
    let key = Key::<Aes256Gcm>::from_slice(key_str);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let cipher = Aes256Gcm::new(key);
    let ciphered_data = cipher.encrypt(&nonce, plaintext).ok()?;
    let mut encrypted_data: Vec<u8> = nonce.to_vec();
    encrypted_data.extend_from_slice(&ciphered_data);
    Some(encrypted_data)
}

pub fn decrypt(key_str: &[u8], encrypted_data: &[u8]) -> Option<Vec<u8>> {
    let key = Key::<Aes256Gcm>::from_slice(key_str);
    let (nonce_arr, ciphered_data) = encrypted_data.split_at(12);
    let nonce = Nonce::from_slice(nonce_arr);
    let cipher = Aes256Gcm::new(key);
    let plaintext = cipher.decrypt(nonce, ciphered_data).ok()?;
    Some(plaintext)
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
