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
    mac: Vec<u8>,
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
            mac,
            kdf_params,
        })
    }

    pub fn new_from_json_struct(
        json_struct: &KeyStoreJson,
        password: &[u8],
    ) -> Result<(EncryptionParams, Vec<u8>), Error> {
        let cipher = AesType::from_str(&json_struct.crypto.cipher)?;
        let unverified_encryption_param = Self {
            encrypted: json_struct.crypto.ciphertext.as_bytes().to_vec(),
            cipher,
            cipher_params: json_struct.crypto.cipherparams.clone(),
            mac: json_struct.crypto.mac.as_bytes().to_vec(),
            kdf_params: json_struct.crypto.kdfparams.clone(),
        };
        let decrypted = unverified_encryption_param.decrypt(&password)?;
        Ok((unverified_encryption_param, decrypted))
    }

    pub fn decrypt(&self, password: &[u8]) -> Result<Vec<u8>, Error> {
        let derived_key = self.kdf_params.generate_derived_key(&password)?;
        let mac = hash::compute_mac(&derived_key[16..32], &self.encrypted);
        if mac != self.mac {
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

        let new_encrypted_text = std::str::from_utf8(&new_encryption_param.encrypted)
            .map_err(|_| CryptoError::PasswordIncorrect)?;
        let new_mac = std::str::from_utf8(&new_encryption_param.mac)
            .map_err(|_| CryptoError::PasswordIncorrect)?;
        let kdf = match new_encryption_param.kdf_params {
            KdfParams::ScryptParam(_) => "scrypt".to_owned(),
        };
        let crypto = Crypto {
            cipher: new_encryption_param.cipher.to_string(),
            cipherparams: new_encryption_param.cipher_params,
            ciphertext: new_encrypted_text.to_owned(),
            kdf,
            kdfparams: new_encryption_param.kdf_params,
            mac: new_mac.to_owned(),
        };
        let key_store_json = KeyStoreJson {
            crypto,
            version: 3,
            id: Uuid::new_v4().to_string(),
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
}
