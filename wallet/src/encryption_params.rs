
use serde::{ Serialize, Deserialize };
use crate::Error;
use crypto::aes_params::AesParams;
use crypto::kdf_params::{ KdfParams, KdfParamsType };
use crypto::scrypt_params::ScryptParams;
use crypto::aes;
use crypto::hash;

#[derive(Serialize, Deserialize)]
pub struct EncryptionParams {
    encrypted: Vec<u8>,
    cipher: String,
    pub cipher_params: AesParams,
    mac: String,
    kdf_params: KdfParams
}

impl EncryptionParams {

    pub fn new(password: &[u8], data: &[u8]) -> Result<EncryptionParams, Error> {
        let kdf_params = KdfParams::ScryptParam(ScryptParams::default());
        let derived_key = kdf_params.generate_derived_key(password)?;
        let cipher_params = AesParams::default();
        let hex_iv = hex::encode(&cipher_params.iv);
        let encrypted = aes::ctr::encrypt(data, &derived_key[0..16], hex_iv.as_bytes())?;
        let mac = hash::compute_mac(&derived_key[16..32], &encrypted);
        let mac_hex = hex::encode(mac);

        Ok(EncryptionParams {
            encrypted,
            cipher: "aes-128-ctr".to_owned(),
            cipher_params,
            mac: mac_hex,
            kdf_params,
        })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
