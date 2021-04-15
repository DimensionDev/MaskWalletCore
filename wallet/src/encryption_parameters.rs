
use crypto::aes_params::AESParams;
use crypto::kdf_params::{ KdfParams, KdfParamsType };
use crypto::scrypt_params::ScryptParameters;
use crypto::aes;
use crypto::hash;

pub struct EncryptionParameters{
    encrypted: Vec<u8>,
    cipher: String,
    pub cipher_params: AESParams,
    mac: String,
    pub kdf_params: KdfParams
}

impl EncryptionParameters {

    pub fn new(password: &[u8], data: &[u8]) -> EncryptionParameters {
        let kdf_param_type = ScryptParameters::default();
        let derived_key = kdf_param_type.generate_derived_key(password);
        let kdf_params = KdfParams::ScryptParam(Box::new(kdf_param_type));
        let cipher_params = AESParams::default();
        let hex_iv = hex::encode(&cipher_params.iv);
        let encrypted = aes::ctr::encrypt(data, &derived_key[0..16], hex_iv.as_bytes()).expect("ASE_CTR encrypt failed");
        let mac = hash::compute_mac(&derived_key[16..32], &encrypted);
        let mac_hex = hex::encode(mac);

        EncryptionParameters {
            encrypted: encrypted,
            cipher: "aes-128-ctr".to_owned(),
            cipher_params: cipher_params,
            mac: mac_hex,
            kdf_params: kdf_params,
        }
    }
}

#[cfg(test)]
mod tests {
    // use EncryptionParameters;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
