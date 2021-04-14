
use crypto::aes_params::AESParams;
use crypto::kdf_params::KdfParams;
use crypto::aes::ctr::encrypt;

pub struct EncryptionParameters<T: KdfParams> {
    data: Vec<u8>,
    cipher: String,
    cipherParams: AESParams,
    mac: Vec<u8>,
    pub kdfParams: T
}

impl<T> EncryptionParameters<T> where T: KdfParams {
    pub fn default() -> EncryptionParameters<T> {
        EncryptionParameters {
            data: vec![],
            cipher: "aes-128-ctr".to_owned(),
            cipherParams: AESParams::default(),
            mac: vec![],
            kdfParams: T::default()
        }
    }

    pub fn new(password: &[u8], data: &[u8]) {
        let encyption_params = T::default();
        let derived_key = encyption_params.generate_derived_key(password);

    }
}