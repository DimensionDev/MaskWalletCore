use crate::Error;
use crypto::aes;
use crypto::aes::AesType;
use crypto::aes_params::AesParams;
use crypto::hash;
use crypto::kdf_params::{KdfParams, KdfParamsType};
use crypto::key_store_json::{Crypto, KeyStoreJson};
use crypto::scrypt_params::ScryptParams;
use crypto::Error as CryptoError;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct EncryptionParams {
    encrypted: Vec<u8>,
    cipher: AesType,
    pub cipher_params: AesParams,
    mac: String,
    kdf_params: KdfParams,
}

impl EncryptionParams {
    pub fn new(password: &[u8], data: &[u8]) -> Result<EncryptionParams, Error> {
        let kdf_params = KdfParams::ScryptParam(ScryptParams::default());
        let derived_key = kdf_params.generate_derived_key(password)?;
        let cipher_params = AesParams::default();
        let iv = hex::decode(&cipher_params.iv).expect("fail to decode iv");
        let encrypted = aes::ctr::encrypt(data, &derived_key[0..16], &iv, 128)?;
        let mac = hash::compute_mac(&derived_key[16..32], &encrypted);

        Ok(EncryptionParams {
            encrypted,
            cipher: AesType::Ctr(128),
            cipher_params,
            mac: hex::encode(mac),
            kdf_params,
        })
    }

    pub fn new_from_json_struct(
        json_struct: &KeyStoreJson,
        password: &[u8],
    ) -> Result<(EncryptionParams, Vec<u8>), Error> {
        let cipher = AesType::from_str(&json_struct.crypto.cipher)?;
        let encrypted_hexdecoded = hex::decode(&json_struct.crypto.ciphertext)
            .or(Err(Error::CryptoError(CryptoError::KdfParamsInvalid)))?;
        let unverified_encryption_param = Self {
            encrypted: encrypted_hexdecoded,
            cipher,
            cipher_params: json_struct.crypto.cipherparams.clone(),
            mac: json_struct.crypto.mac.clone(),
            kdf_params: json_struct.crypto.kdfparams.clone(),
        };
        let decrypted = unverified_encryption_param.decrypt(&password)?;
        Ok((unverified_encryption_param, decrypted))
    }

    pub fn decrypt(&self, password: &[u8]) -> Result<Vec<u8>, Error> {
        let derived_key = self.kdf_params.generate_derived_key(&password)?;
        let mac = hash::compute_mac(&derived_key[16..32], &self.encrypted);
        let mac_hex = hex::encode(mac);
        if mac_hex != self.mac {
            return Err(Error::CryptoError(CryptoError::PasswordIncorrect));
        }
        let iv = hex::decode(&self.cipher_params.iv).expect("fail to decode iv");
        match self.cipher {
            AesType::Ctr(bits) => Ok(aes::ctr::decrypt(
                &self.encrypted,
                &derived_key[0..16],
                &iv,
                bits,
            )?),
            AesType::Cbc(_) => Err(Error::CryptoError(CryptoError::NotSupportedCipher)),
        }
    }

    pub fn export_to_key_store_json(
        &self,
        password: &str,
        new_password: &str,
    ) -> Result<String, Error> {
        // 1. Check the password by using the decrypt method
        let decrypted = self.decrypt(&password.as_bytes())?;

        // 2. Generate a temp new EncryptionParam using the new_password
        let new_encryption_param = Self::new(&new_password.as_bytes(), &decrypted)?;

        let new_encrypted_text = hex::encode(&new_encryption_param.encrypted);

        // let new_encrypted_text = std::str::from_utf8(&new_encryption_param.encrypted)
        //     .map_err(|_| CryptoError::PasswordIncorrect)?;
        let kdf = match new_encryption_param.kdf_params {
            KdfParams::ScryptParam(_) => "scrypt".to_owned(),
        };
        let crypto = Crypto {
            cipher: new_encryption_param.cipher.to_string(),
            cipherparams: new_encryption_param.cipher_params,
            ciphertext: new_encrypted_text.to_owned(),
            kdf,
            kdfparams: new_encryption_param.kdf_params,
            mac: new_encryption_param.mac.clone(),
        };
        let key_store_json = KeyStoreJson {
            crypto,
            version: 3,
            id: Uuid::new_v4().to_string().to_uppercase(),
        };
        let json_str = serde_json::to_string_pretty(&key_store_json)
            .map_err(|_| Error::JsonSerializationError)?;
        Ok(json_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_decrypt() {
        let password = "mask";
        let data = "team engine square letter hero song dizzy scrub tornado fabric divert saddle";
        let enc_param = EncryptionParams::new(password.as_bytes(), data.as_bytes()).unwrap();
        let decrypted = enc_param.decrypt(password.as_bytes()).unwrap();
        assert_eq!(data.as_bytes(), &decrypted);
    }

    #[test]
    fn test_decrypt_json_export_json() {
        let json = r#"
        {
            "version":3,
            "id":"E511D153-EB10-484A-A649-56A3E015E4D3",
            "crypto":{
                "ciphertext":"5c74a0c7513168a602e8fc32892c4c2c0371099073a6a4f504be041c571e2781",
                "cipherparams":{
                    "iv":"e83921ccf41447518b27dd1a22129494"
                },
                "kdf":"scrypt",
                "kdfparams":{
                    "r":8,
                    "p":1,
                    "n":1024,
                    "dklen":32,
                    "salt":"ae2ef76580540174997df3191d32e577fb44693c037eae3cf1842a22b892c02a"
                },
                "mac":"4b85aff1322e833507b574db2471daf80c51663cd00a256c80711eba91cfd47f",
                "cipher":"aes-128-ctr"
            }
        }
        "#;
        let key_store_json_password = "Maskbook123";
        let json_struct = KeyStoreJson::from_str(&json).unwrap();
        let cipher = AesType::from_str(&json_struct.crypto.cipher).unwrap();
        let encrypted_hexdecoded = hex::decode(&json_struct.crypto.ciphertext).unwrap();
        let unverified_encryption_param = EncryptionParams {
            encrypted: encrypted_hexdecoded,
            cipher,
            cipher_params: json_struct.crypto.cipherparams.clone(),
            mac: json_struct.crypto.mac.clone(),
            kdf_params: json_struct.crypto.kdfparams.clone(),
        };
        let derived_key = unverified_encryption_param
            .kdf_params
            .generate_derived_key(&key_store_json_password.as_bytes())
            .unwrap();
        let mac = hash::compute_mac(&derived_key[16..32], &unverified_encryption_param.encrypted);
        let hex_mac = hex::encode(mac);
        assert_eq!(hex_mac, unverified_encryption_param.mac);

        let iv =
            hex::decode(&unverified_encryption_param.cipher_params.iv).expect("fail to decode iv");
        let result_bits = match unverified_encryption_param.cipher {
            AesType::Ctr(bits) => aes::ctr::decrypt(
                &unverified_encryption_param.encrypted,
                &derived_key[0..16],
                &iv,
                bits,
            )
            .unwrap(),
            AesType::Cbc(_) => vec![],
        };
        let test_encrypted =
            aes::ctr::encrypt(&result_bits, &derived_key[0..16], &iv, 128).unwrap();
        assert_eq!(test_encrypted.len() as u8, 32);
        assert_eq!(result_bits.len() as u8, 32);
    }
}
