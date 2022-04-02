use crate::Error;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::string::ToString;

#[derive(Serialize, Deserialize, PartialEq)]
pub enum AesType {
    Ctr(u32),
    Cbc(u32),
}

impl FromStr for AesType {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Error> {
        match s.to_lowercase().as_str() {
            "aes-128-ctr" => Ok(Self::Ctr(128)),
            "aes-192-ctr" => Ok(Self::Ctr(192)),
            "aes-256-ctr" => Ok(Self::Ctr(256)),
            "aes-128-cbc" => Ok(Self::Cbc(128)),
            "aes-192-cbc" => Ok(Self::Cbc(192)),
            "aes-256-cbc" => Ok(Self::Cbc(256)),
            _ => Err(Error::NotSupportedCipher),
        }
    }
}

impl ToString for AesType {
    fn to_string(&self) -> String {
        match self {
            Self::Ctr(128) => "aes-128-ctr".to_owned(),
            Self::Ctr(192) => "aes-192-ctr".to_owned(),
            Self::Ctr(256) => "aes-256-ctr".to_owned(),
            Self::Cbc(128) => "aes-128-cbc".to_owned(),
            Self::Cbc(192) => "aes-192-cbc".to_owned(),
            Self::Cbc(256) => "aes-256-cbc".to_owned(),
            _ => "Unknown".to_owned(),
        }
    }
}

pub mod ctr {
    use aes::cipher::generic_array::GenericArray;
    use aes::cipher::{NewCipher, StreamCipher};
    use aes::{Aes128Ctr, Aes192Ctr, Aes256Ctr};

    use crate::Error;

    type CryptoResult<T> = Result<T, Error>;

    pub fn encrypt(data: &[u8], key: &[u8], iv: &[u8], bits: u32) -> CryptoResult<Vec<u8>> {
        if key.len() != 16 || iv.len() != 16 {
            return Err(Error::InvalidKeyIvLength);
        }
        if ![128, 192, 256].contains(&bits) {
            return Err(Error::NotSupportedCipher);
        }
        let mut data_copy = vec![0; data.len()];
        data_copy.copy_from_slice(data);

        match bits {
            128 => {
                let key = GenericArray::from_slice(key);
                let iv = GenericArray::from_slice(iv);
                Aes128Ctr::new(key, iv).apply_keystream(&mut data_copy)
            }
            192 => {
                let key = GenericArray::from_slice(key);
                let iv = GenericArray::from_slice(iv);
                Aes192Ctr::new(key, iv).apply_keystream(&mut data_copy)
            }
            256 => {
                let key = GenericArray::from_slice(key);
                let iv = GenericArray::from_slice(iv);
                Aes256Ctr::new(key, iv).apply_keystream(&mut data_copy)
            }
            _ => return Err(Error::NotSupportedCipher),
        };
        Ok(data_copy)
    }

    pub fn decrypt(data: &[u8], key: &[u8], iv: &[u8], bits: u32) -> CryptoResult<Vec<u8>> {
        if key.len() != 16 || iv.len() != 16 {
            return Err(Error::InvalidKeyIvLength);
        }
        if ![128, 192, 256].contains(&bits) {
            return Err(Error::NotSupportedCipher);
        }
        let mut data_copy = vec![0; data.len()];
        data_copy.copy_from_slice(data);
        match bits {
            128 => {
                let key = GenericArray::from_slice(key);
                let iv = GenericArray::from_slice(iv);
                Aes128Ctr::new(key, iv).apply_keystream(&mut data_copy)
            }
            192 => {
                let key = GenericArray::from_slice(key);
                let iv = GenericArray::from_slice(iv);
                Aes192Ctr::new(key, iv).apply_keystream(&mut data_copy)
            }
            256 => {
                let key = GenericArray::from_slice(key);
                let iv = GenericArray::from_slice(iv);
                Aes256Ctr::new(key, iv).apply_keystream(&mut data_copy)
            }
            _ => return Err(Error::NotSupportedCipher),
        };
        Ok(data_copy)
    }
}

#[cfg(test)]
mod tests {

    use hex::ToHex;

    #[test]
    fn ctr_encrypt_test() {
        use crate::aes::ctr::encrypt;

        let data = "MaskWallet".as_bytes();
        let key = hex::decode("01020304010203040102030401020304").unwrap();
        let iv = hex::decode("01020304010203040102030401020304").unwrap();
        let ret = encrypt(data, &key, &iv, 128).expect("encrypt nopadding data");
        let ret_hex = ret.encode_hex::<String>();

        assert_eq!("f89074571af13f467cd4", ret_hex);

        let wrong_len_key = hex::decode("010203040102030401020304").unwrap();
        let ret = encrypt(data, &wrong_len_key, &iv, 128);
        assert!(ret.is_err());

        let wrong_len_iv = hex::decode("010203040102030401020304").unwrap();
        let ret = encrypt(data, &key, &wrong_len_iv, 128);
        assert!(ret.is_err());
    }

    #[test]
    fn decrypted_data_test() {
        use crate::aes::ctr::decrypt;

        let data = "MaskWallet".as_bytes();
        let encrypted_data = hex::decode("f89074571af13f467cd4").unwrap();
        let key = hex::decode("01020304010203040102030401020304").unwrap();
        let iv = hex::decode("01020304010203040102030401020304").unwrap();
        let ret = decrypt(&encrypted_data, &key, &iv, 128).expect("decrypted data error");

        assert_eq!(
            "MaskWallet",
            String::from_utf8(ret).expect("decrypted failed")
        );

        let wrong_len_key = hex::decode("010203040102030401020304").unwrap();
        let ret = decrypt(data, &wrong_len_key, &iv, 128);
        assert!(ret.is_err());

        let wrong_len_iv = hex::decode("010203040102030401020304").unwrap();
        let ret = decrypt(data, &key, &wrong_len_iv, 128);
        assert!(ret.is_err());
    }
}
