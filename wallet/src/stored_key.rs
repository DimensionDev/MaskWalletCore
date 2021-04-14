use uuid::Uuid;

use super::account::Account;
use super::encryption_parameters::{ EncryptionParameters };
use crypto::kdf_params::KdfParams;
use crypto::scrypt_params::ScryptParameters;

pub enum StoredKeyType {
    PrivateKey,
    Mnemonic,
}

pub struct StoredKey<T: KdfParams> {
    pub r#type: StoredKeyType,

    pub id: String,

    pub name: String,

    payload: EncryptionParameters<T>,

    accounts: Vec<Account>,
}

impl<T: KdfParams> StoredKey<T> {
    pub fn create_with_private_key(name: &str, password: &str, private_key: &str) -> StoredKey<ScryptParameters> {
        let uuid = Uuid::new_v4();
        let payload = EncryptionParameters::<ScryptParameters>::default();
        StoredKey {
            r#type: StoredKeyType::PrivateKey,
            name: String::from(name),
            id: uuid.to_string(),
            payload: payload,
            accounts: vec![]
        }
    }
}