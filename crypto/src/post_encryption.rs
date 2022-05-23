use super::number_util::random_iv;
use super::Error;

use aes_gcm::aead::{Aead, NewAead};
use aes_gcm::aes::{cipher::consts::U16, Aes256};
use aes_gcm::{AesGcm, Key, Nonce};

use rmp::encode::*;

use std::str;

type Aes256GCM = AesGcm<Aes256, U16>;

const IV_SIZE: usize = 16;
const AES_KEY_SIZE: usize = 32;

enum Index {
    Version = 0,
    AuthorNetwork = 1,
    AuthorID = 2,
    AuthorPublicKeyAlgorithm = 3,
    AuthorPublicKey = 4,
    Encryption = 5,
    Data = 6,
}

#[derive(Debug)]
struct Payload {
    author_network: String,
    author_id: String,
    author_public_key_algorithm: u8,
    author_public_key: Vec<u8>,
    encryption: Vec<Vec<u8>>,
    data: Vec<u8>,
}

impl Default for Payload {
    fn default() -> Self {
        Payload {
            author_network: String::new(),
            author_id: String::new(),
            author_public_key_algorithm: 0,
            author_public_key: Vec::new(),
            encryption: Vec::new(),
            data: Vec::new(),
        }
    }
}

pub fn encrypt(
    network: &str,
    authorId: &str,
    algr: u8,
    author_pub_key: &[u8],
    message: &[u8],
) -> Result<Vec<u8>, Error> {
    let post_iv = random_iv(IV_SIZE);
    let post_key_iv = random_iv(AES_KEY_SIZE);

    let encrypted_message = aes_encrypt(&post_iv, &post_key_iv, &message)?;
    let output = encode_with_container(
        network,
        authorId,
        algr,
        &author_pub_key,
        &post_key_iv,
        &post_iv,
        &encrypted_message,
    )
    .map_err(|_| Error::InvalidCiphertext)?;
    Ok(output)
}

fn aes_encrypt(iv: &[u8], key: &[u8], content: &[u8]) -> Result<Vec<u8>, Error> {
    let key = Key::from_slice(&key);
    let nonce = Nonce::from_slice(&iv);
    let cipher = Aes256GCM::new(key);
    cipher
        .encrypt(nonce, content)
        .map_err(|_| Error::InvalidCiphertext)
}

fn aes_decrypt(iv: &[u8], key: &[u8], encrypted_content: &[u8]) -> Result<Vec<u8>, Error> {
    let nonce = Nonce::from_slice(iv);
    let key = Key::from_slice(&key);
    let cipher = Aes256GCM::new(key);

    cipher
        .decrypt(nonce, encrypted_content.as_ref())
        .map_err(|_| Error::InvalidCiphertext)
}

fn encode_with_container(
    network: &str,
    authorId: &str,
    algr: u8,
    author_pub_key: &[u8],
    aes_key: &[u8],
    iv: &[u8],
    encrypted: &[u8],
) -> Result<Vec<u8>, Error> {
    let encoded_without_container = encode_v37(
        &network,
        &authorId,
        algr,
        &author_pub_key,
        &aes_key,
        &iv,
        &encrypted,
    )
    .map_err(|_| Error::InvalidCiphertext)?;
    let mut buf = Vec::new();
    write_map_len(&mut buf, 2).map_err(|_| Error::InvalidCiphertext)?;
    write_sint(&mut buf, 0).map_err(|_| Error::InvalidCiphertext)?;
    write_bin(&mut buf, &encoded_without_container).map_err(|_| Error::InvalidCiphertext)?;
    Ok(buf)
}

fn encode_v37(
    network: &str,
    authorId: &str,
    algr: u8,
    author_pub_key: &[u8],
    aes_key: &[u8],
    iv: &[u8],
    encrypted: &[u8],
) -> Result<Vec<u8>, Error> {
    let mut buf = Vec::new();
    write_map_len(&mut buf, 6).map_err(|_| Error::InvalidCiphertext)?;

    write_sint(&mut buf, Index::AuthorNetwork as i64).map_err(|_| Error::InvalidCiphertext)?;
    write_str(&mut buf, &network).map_err(|_| Error::InvalidCiphertext)?;

    write_sint(&mut buf, Index::AuthorID as i64).map_err(|_| Error::InvalidCiphertext)?;
    write_str(&mut buf, &authorId).map_err(|_| Error::InvalidCiphertext)?;

    write_sint(&mut buf, Index::AuthorPublicKeyAlgorithm as i64)
        .map_err(|_| Error::InvalidCiphertext)?;
    write_sint(&mut buf, algr as i64).map_err(|_| Error::InvalidCiphertext)?;

    write_sint(&mut buf, Index::AuthorPublicKey as i64).map_err(|_| Error::InvalidCiphertext)?;
    write_bin(&mut buf, &author_pub_key);

    write_sint(&mut buf, Index::Encryption as i64).map_err(|_| Error::InvalidCiphertext)?;
    write_array_len(&mut buf, 3).map_err(|_| Error::InvalidCiphertext)?;
    write_sint(&mut buf, 0).map_err(|_| Error::InvalidCiphertext)?;
    write_bin(&mut buf, &aes_key);
    write_bin(&mut buf, &iv);

    write_sint(&mut buf, Index::Data as i64).map_err(|_| Error::InvalidCiphertext)?;
    write_bin(&mut buf, &encrypted).map_err(|_| Error::InvalidCiphertext)?;

    Ok(buf.to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rmp::encode::*;

    const IV: [u8; 16] = [
        150, 13, 224, 121, 241, 237, 66, 179, 38, 88, 203, 177, 192, 239, 197, 189,
    ];
    // content text: "sample text"
    const ENCODED_MESSAGE: [u8; 18] = [
        146, 0, 148, 1, 1, 192, 171, 115, 97, 109, 112, 108, 101, 32, 116, 101, 120, 116,
    ];

    const RANDOM_IV1: [u8; 16] = [
        103, 255, 64, 75, 77, 251, 1, 164, 34, 237, 4, 16, 126, 175, 142, 35,
    ];
    const RANDOM_IV2: [u8; 16] = [
        150, 164, 124, 165, 4, 65, 142, 140, 96, 64, 241, 15, 128, 231, 32, 186,
    ];

    #[test]
    fn test_encoding() {
        let mut buf = Vec::new();
        write_array_len(&mut buf, 2).unwrap();
        write_sint(&mut buf, 0).unwrap();
        write_array_len(&mut buf, 4).unwrap();
        write_sint(&mut buf, 1).unwrap();
        write_sint(&mut buf, 1).unwrap();
        write_nil(&mut buf).unwrap();
        write_str(&mut buf, &"sample text").unwrap();
        println!("{:?}", &buf[..]);
        assert_eq!(&buf[..], &ENCODED_MESSAGE);
    }

    #[test]
    fn test_aes_encrypt() {
        let iv: [u8; 16] = [1; 16];
        let key: [u8; 32] = [2; 32];
        let content = "sample text";
        let encrypted = aes_encrypt(&iv, &key, &content.as_bytes()).unwrap();
        let decrypted = aes_decrypt(&iv, &key, &encrypted).unwrap();
        assert_eq!(decrypted, content.as_bytes());
    }

    #[test]
    fn test_encode_v37() {
        let post_iv = random_iv(IV_SIZE);
        let post_key_iv = random_iv(AES_KEY_SIZE);
        let author_key = random_iv(33);
        let content = "sample text";

        let encrypted_message = aes_encrypt(&post_iv, &post_key_iv, &content.as_bytes()).unwrap();
        let message = "hello world";
        let network = "localhost";
        let authorId = "alice";
        let algr = 2;
        let encode_with_no_sign = encode_with_container(
            &network,
            &authorId,
            algr,
            &author_key,
            &post_key_iv,
            &post_iv,
            &encrypted_message,
        )
        .unwrap();
        assert_eq!(&encode_with_no_sign, "1".as_bytes());
    }

    #[test]
    fn test_encrypt() {
        let post_iv = random_iv(IV_SIZE);
        let post_key_iv = random_iv(AES_KEY_SIZE);
        let author_key = random_iv(33);
        let content = "sample text";

        let encrypted_message = aes_encrypt(&post_iv, &post_key_iv, &content.as_bytes()).unwrap();
        let message = "hello world";
        let network = "localhost";
        let authorId = "alice";
        let algr = 2;
        let output = encrypt(&network, &authorId, algr, &author_key, &content.as_bytes()).unwrap();

        assert_eq!(&output, "1".as_bytes());
    }
}
