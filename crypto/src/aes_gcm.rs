use aes_gcm::aead::{Aead, NewAead};
use aes_gcm::aes::{cipher::consts::U16, Aes256};
use aes_gcm::{AesGcm, Key, Nonce};

use super::Error;

type Aes256GCM = AesGcm<Aes256, U16>;

pub fn aes_encrypt(iv: &[u8], key: &[u8], content: &[u8]) -> Result<Vec<u8>, Error> {
    let key = Key::from_slice(&key);
    let nonce = Nonce::from_slice(&iv);
    let cipher = Aes256GCM::new(key);
    cipher
        .encrypt(nonce, content)
        .map_err(|_| Error::InvalidCiphertext)
}

pub fn aes_decrypt(iv: &[u8], key: &[u8], encrypted_content: &[u8]) -> Result<Vec<u8>, Error> {
    let nonce = Nonce::from_slice(iv);
    let key = Key::from_slice(&key);
    let cipher = Aes256GCM::new(key);

    cipher
        .decrypt(nonce, encrypted_content.as_ref())
        .map_err(|_| Error::InvalidCiphertext)
}
