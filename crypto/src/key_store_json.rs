use super::aes_params::AesParams;
use super::kdf_params::KdfParams;
use crate::Error;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Serialize, Deserialize)]
pub struct KeyStoreJson {
    pub crypto: Crypto,
    pub id: String,
    pub version: u32,
}

#[derive(Serialize, Deserialize)]
pub struct Crypto {
    pub cipher: String,
    pub cipherparams: AesParams,
    pub ciphertext: String,
    pub kdf: String,
    pub kdfparams: KdfParams,
    pub mac: String,
}

impl FromStr for KeyStoreJson {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s).map_err(|_| Error::InvalidKeyStoreJSON)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kdf_params::KdfParams;
    #[test]
    fn test_import_keystore_json() {
        let data = r#"
        {
            "crypto" : {
                "cipher" : "aes-128-ctr",
                "cipherparams" : {
                    "iv" : "83dbcc02d8ccb40e466191a123791e0e"
                },
                "ciphertext" : "d172bf743a674da9cdad04534d56926ef8358534d458fffccd4e6ad2fbde479c",
                "kdf" : "scrypt",
                "kdfparams" : {
                    "dklen" : 32,
                    "n" : 262144,
                    "r" : 1,
                    "p" : 8,
                    "salt" : "ab0c7876052600dd703518d6fc3fe8984592145b591fc8fb5c6d43190334ba19"
                },
                "mac" : "2103ac29920d71da29f15d75b4a16dbe95cfd7ff8faea1056c33131d846e3097"
            },
            "id" : "3198bc9c-6672-5ab3-d995-4942343ae5b6",
            "version" : 3
        }
        "#;
        let key_store_json_struct: KeyStoreJson = serde_json::from_str(data).unwrap();
        assert_eq!(
            key_store_json_struct.id,
            "3198bc9c-6672-5ab3-d995-4942343ae5b6"
        );
        assert_eq!(key_store_json_struct.version, 3);
        assert_eq!(key_store_json_struct.crypto.cipher, "aes-128-ctr");
        assert_eq!(
            key_store_json_struct.crypto.cipherparams.iv,
            "83dbcc02d8ccb40e466191a123791e0e"
        );
        let scrypt_param = match key_store_json_struct.crypto.kdfparams {
            KdfParams::ScryptParam(param) => param,
        };
        assert_eq!(scrypt_param.n, 262144);
        assert_eq!(scrypt_param.dklen, 32);
        assert_eq!(scrypt_param.r, 1);
        assert_eq!(scrypt_param.p, 8);
        assert_eq!(
            scrypt_param.salt,
            "ab0c7876052600dd703518d6fc3fe8984592145b591fc8fb5c6d43190334ba19"
        );
        assert_eq!(
            key_store_json_struct.crypto.ciphertext,
            "d172bf743a674da9cdad04534d56926ef8358534d458fffccd4e6ad2fbde479c"
        );
        assert_eq!(key_store_json_struct.crypto.kdf, "scrypt");
        assert_eq!(
            key_store_json_struct.crypto.mac,
            "2103ac29920d71da29f15d75b4a16dbe95cfd7ff8faea1056c33131d846e3097"
        );
    }
}
