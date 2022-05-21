use super::number_util::random_iv;
use super::Error;

use aes_gcm::aead::{Aead, NewAead};
use aes_gcm::aes::{cipher::consts::U16, Aes256};
use aes_gcm::{AesGcm, Key, Nonce};

use rmp_serde as rmps;
use rmps::{Deserializer, Serializer};
use serde::{Deserialize, Serialize};

use std::str;

type Aes256GCM = AesGcm<Aes256, U16>;
pub struct EncryptOptions {}

trait Encrypting {
    fn encrypt(&self, iv: &[u8], message: &str) -> Result<EncryptedMessage, Error>;
    fn decrypt(&self, iv: &[u8], encrypted_message: EncryptedMessage) -> Result<Vec<u8>, Error>;
}

struct EncryptedMessage {
    cipher: Vec<u8>,
    key_slice: Vec<u8>,
}

struct TestKeyGenerator;

impl Encrypting for TestKeyGenerator {
    fn encrypt(&self, iv: &[u8], message: &str) -> Result<EncryptedMessage, Error> {
        let random_key_slice = random_iv(32);
        // key size must use u32
        let key = Key::from_slice(&random_key_slice);

        let nonce = Nonce::from_slice(iv);
        let cipher = Aes256GCM::new(key);

        cipher
            .encrypt(nonce, message.as_bytes())
            .map_err(|_| Error::InvalidCiphertext)
            .map(|result| EncryptedMessage {
                cipher: result,
                key_slice: random_key_slice,
            })
    }

    fn decrypt(&self, iv: &[u8], encrypted_message: EncryptedMessage) -> Result<Vec<u8>, Error> {
        let random_key_slice = encrypted_message.key_slice;
        let key = Key::from_slice(&random_key_slice);
        let nonce = Nonce::from_slice(iv);
        let cipher = Aes256GCM::new(key);

        let verbs = encrypted_message.cipher;
        cipher
            .decrypt(nonce, verbs.as_ref())
            .map_err(|_| Error::InvalidCiphertext)
    }
}

fn encrypt(content: String, version: u8) -> String {
    // iv
    let post_iv = random_iv(16);

    // post_key
    // let key = Key::from_slice(b"131");
    // let cipher = Aes256Gcm::generate_key(&post_iv);
    // let nonce = Nonce::from_slice(&post_iv);
    // let cipher_text = cipher.encrypt(nonce, b"sample text".as_ref());

    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let post_iv = Vec::from(IV);
        let generator = TestKeyGenerator;
        let result = generator.encrypt(&post_iv, "sample text").unwrap();
        println!("{:?}", result.cipher);

        // Type Message
        let mut buf: Vec<u8> = Vec::new();
        let mut encoder = Serializer::new(&mut buf);
        let _ = "sample text".serialize(&mut encoder);
        assert_eq!(&buf[..], &MESSAGE);

        // let result = rmps::to_vec_named(&(0, "sample text")).unwrap();
        // let decoded: (u8, &str) = rmps::from_slice(&result).unwrap();

        let text: &str = rmps::from_slice(&MESSAGE).unwrap();
        println!("{:?}", text);

        #[derive(Debug, Serialize, Deserialize)]
        struct TestModle {
            number: u64,
            test: String,
        }

        let mut model_buf = Vec::new();
        let mut model_encoder = Serializer::new(&mut model_buf);
        let model = TestModle {
            number: 0,
            test: "safe".to_owned(),
        };
        let _ = model.serialize(&mut model_encoder);
        // let (number, string): Result<(u8, String), Error> = rmps::from_slice(&buf);
    }
}
