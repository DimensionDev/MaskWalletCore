use super::number_util::random_iv;
use super::Error;

use aes_gcm::aead::{
    // rand_core::{CryptoRng, RngCore},
    Aead,
    NewAead,
};
use aes_gcm::aes::{cipher::consts::U16, Aes256};
use aes_gcm::{AesGcm, Key, Nonce};

use rmp_serde as rmps;
use rmps::{Deserializer, Serializer};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use std::str;

type Aes256GCM = AesGcm<Aes256, U16>;
pub struct EncryptOptions {}

pub struct EncryptedMessage {
    pub cipher: Vec<u8>,
    pub key_slice: Vec<u8>,
    pub nonce: Vec<u8>,
}

trait Encrypting {
    fn encrypt(&self, message: &str) -> Result<EncryptedMessage, Error>;
    fn decrypt(&self, encrypted_message: EncryptedMessage) -> Result<Vec<u8>, Error>;
}

struct TestKeyGenerator;

impl Encrypting for TestKeyGenerator {
    fn encrypt(&self, message: &str) -> Result<EncryptedMessage, Error> {
        let iv = random_iv(16);
        let random_key_slice = random_iv(32);
        // key size must use u32
        let key = Key::from_slice(&random_key_slice);
        let nonce = Nonce::from_slice(&iv);
        let cipher = Aes256GCM::new(key);

        cipher
            .encrypt(nonce, message.as_bytes())
            .map_err(|_| Error::InvalidCiphertext)
            .map(|result| EncryptedMessage {
                cipher: result,
                key_slice: random_key_slice,
                nonce: iv,
            })
    }

    fn decrypt(&self, encrypted_message: EncryptedMessage) -> Result<Vec<u8>, Error> {
        let random_key_slice = encrypted_message.key_slice;
        let key = Key::from_slice(&random_key_slice);
        let iv = encrypted_message.nonce;
        let nonce = Nonce::from_slice(&iv);
        let cipher = Aes256GCM::new(key);
        let verbs = encrypted_message.cipher;
        cipher
            .decrypt(nonce, verbs.as_ref())
            .map_err(|_| Error::InvalidCiphertext)
    }
}

pub fn encrypt(
    content: String,
    authot_key: String,
    network: String,
) -> Result<EncryptedMessage, Error> {
    // iv
    let post_iv = random_iv(16);

    // post_key
    // let key = Key::from_slice(b"131");
    // let cipher = Aes256Gcm::generate_key(&post_iv);
    // let nonce = Nonce::from_slice(&post_iv);
    // let cipher_text = cipher.encrypt(nonce, b"sample text".as_ref());

    Err(Error::InvalidCiphertext)
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::io::Cursor;

    const IV: [u8; 16] = [
        150, 13, 224, 121, 241, 237, 66, 179, 38, 88, 203, 177, 192, 239, 197,
        189,
        // 150, 13, 224, 121, 241, 237, 66, 179, 38, 88, 203, 177, 192, 239, 197, 189,
    ];
    // content text: "sample text"
    // js code will be [115, 97, 109, 112, 108, 101, 32, 116, 101, 120, 116]
    const MESSAGE: [u8; 12] = [171, 115, 97, 109, 112, 108, 101, 32, 116, 101, 120, 116];

    // const AESRESULT: [u8; 27] = [
    //     111, 240, 132, 248, 234, 237, 4, 148, 98, 135, 219, 174, 16, 118, 48, 212, 157, 202, 116,
    //     11, 38, 156, 158, 167, 185, 64, 29,
    // ];

    #[test]
    fn test_encryption() {
        let message = &MESSAGE[1..];
        println!("message --{message:?}");
        let message_text: Result<&str, _> = rmps::from_slice(&MESSAGE[1..]);
        println!("{message_text:?}");
        let generator = TestKeyGenerator;
        let result = generator.encrypt("sample text").unwrap();
        println!("{:?}", result.cipher);

        // Type Message
        let mut buf: Vec<u8> = Vec::new();
        let mut encoder = Serializer::new(&mut buf);
        let _ = "sample text".serialize(&mut encoder);
        assert_eq!(&buf[..], &MESSAGE);

        let result = rmps::to_vec_named(&(0, "sample text")).unwrap();
        let decoded: Result<(u8, &str), _> = rmps::from_slice(&result);

        let text: &str = rmps::from_slice(&MESSAGE).unwrap();
        println!("{:?}", text);

        #[derive(Debug, Serialize, Deserialize)]
        struct TestModel {
            number: u64,
            test: String,
        }

        let mut model_buf = Vec::new();
        let mut model_encoder = Serializer::new(&mut model_buf);
        let model = TestModel {
            number: 0,
            test: "safe".to_owned(),
        };
        let _ = model.serialize(&mut model_encoder);
        let model_result: Result<TestModel, rmps::decode::Error> = rmps::from_slice(&buf);
    }
}
