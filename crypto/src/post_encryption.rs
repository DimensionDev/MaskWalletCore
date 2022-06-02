use super::number_util::random_iv;
use super::payload_encode_v37::encode_with_container as encode_v37;
use super::payload_encode_v38::encode_v38;
use super::Error;
use std::collections::HashMap;

use super::aes_gcm::{aes_decrypt, aes_encrypt};

use rmp::encode::*;

use std::str;

const IV_SIZE: usize = 16;
const AES_KEY_SIZE: usize = 32;

pub enum Version {
    V37 = -37,
    V38 = -38,
}

pub struct EncryptionResultE2E {
    pub target: String,
    pub encrypted_post_key: Vec<u8>,
    pub iv_to_be_published: Option<Vec<u8>>,
}

pub fn encrypt(
    version: Version,
    is_public: bool,
    network: Option<&str>,
    author_id: Option<&str>,
    algr: Option<u8>,
    author_pub_key: Option<&[u8]>,
    message: &[u8],
    local_key_data: Option<&[u8]>,
    target: HashMap<String, Vec<u8>>,
    author_private_key: Option<&str>,
) -> Result<(String, Option<HashMap<String, EncryptionResultE2E>>), Error> {
    let post_iv = random_iv(IV_SIZE);
    let post_key_iv = random_iv(AES_KEY_SIZE);

    let encrypted_message = aes_encrypt(&post_iv, &post_key_iv, &message)?;

    match version {
        Version::V37 => Err(Error::NotSupportedCipher),
        Version::V38 => {
            let output = encode_v38(
                is_public,
                network,
                author_id,
                &post_iv,
                &post_key_iv,
                &encrypted_message,
                author_pub_key,
                local_key_data,
                target,
                author_private_key,
            )
            .map_err(|_| Error::InvalidCiphertext)?;
            Ok(output)
        }
    }
}

impl From<ValueWriteError> for Error {
    fn from(_: ValueWriteError) -> Error {
        Error::InvalidCiphertext
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // content text: "sample text"
    const ENCODED_MESSAGE: [u8; 18] = [
        146, 0, 148, 1, 1, 192, 171, 115, 97, 109, 112, 108, 101, 32, 116, 101, 120, 116,
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
        write_str(&mut buf, "sample text").unwrap();
        println!("{:?}", &buf[..]);
        assert_eq!(&buf[..], &ENCODED_MESSAGE);
    }

    #[test]
    fn test_aes_encrypt() {
        let iv: [u8; 16] = [1; 16];
        let key: [u8; 32] = [2; 32];
        let content = "sample text";
        let encrypted = aes_encrypt(&iv, &key, content.as_bytes()).unwrap();
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
            true,
            Some(network),
            Some(author_id),
            Some(algr),
            Some(&public_key_data),
            message.as_bytes(),
            None,
            HashMap::new(),
            None,
        )
        .unwrap();
    }
}
