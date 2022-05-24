use super::number_util::random_iv;
use super::payload_encode_v37::encode_with_container as encode_v37;
use super::payload_encode_v38::encode_v38;
use super::Error;

use super::aes_gcm::{aes_decrypt, aes_encrypt};

use rmp::encode::*;

use std::str;

const IV_SIZE: usize = 16;
const AES_KEY_SIZE: usize = 32;

pub enum Version {
    V37 = -37,
    V38 = -38,
}

pub enum Target {
    Public,
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
    version: Version,
    target: Target,
    network: Option<&str>,
    author_id: Option<&str>,
    algr: u8,
    author_pub_key: Option<&[u8]>,
    message: &[u8],
) -> Result<String, Error> {
    let post_iv = random_iv(IV_SIZE);
    let post_key_iv = random_iv(AES_KEY_SIZE);

    let encrypted_message = aes_encrypt(&post_iv, &post_key_iv, &message)?;

    let output = match version {
        Version::V37 => "1".to_string(),
        Version::V38 => {
            let output = encode_v38(
                target,
                network,
                author_id,
                &post_iv,
                &post_key_iv,
                &encrypted_message,
                author_pub_key,
            )
            .map_err(|_| Error::InvalidCiphertext)?;
            output
        }
    };

    Ok(output)
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
    fn test_encrypt_v38_public() {
        let network = "twitter.com";
        let author_id = "yuan_brad";
        let message = "123";
        let algr = 2;
        let public_key_data = [
            2, 210, 107, 119, 140, 57, 180, 37, 245, 126, 86, 79, 41, 128, 107, 64, 99, 141, 222,
            6, 87, 249, 95, 130, 198, 99, 1, 113, 41, 91, 239, 152, 212,
        ];
        // let output = encrypt(
        let output = encrypt(
            Version::V38,
            Target::Public,
            Some(network),
            Some(author_id),
            algr,
            Some(&public_key_data),
            message.as_bytes(),
        )
        .unwrap();

        assert_eq!(&output, "1");
    }
}
