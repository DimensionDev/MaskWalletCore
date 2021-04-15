use uuid::Uuid;

use crate::Error;
use super::account::Account;
use super::encryption_parameters::{ EncryptionParameters };
use super::private_key::PrivateKey;
use super::coin::Coin;
use crypto::curve::Curve;
use crypto::kdf_params::KdfParams;
use crypto::scrypt_params::ScryptParameters;

pub enum StoredKeyType {
    PrivateKey,
    Mnemonic,
}

pub struct StoredKey {
    pub r#type: StoredKeyType,

    pub id: String,

    pub name: String,

    payload: EncryptionParameters,

    accounts: Vec<Account>,
}

impl StoredKey {
    pub fn create_with_private_key(name: &str, password: &str, private_key: &str) -> Result<StoredKey, Error> {
        let uuid = Uuid::new_v4();
        let payload = EncryptionParameters::new(password.as_bytes(), private_key.as_bytes())?;
        Ok(StoredKey {
            r#type: StoredKeyType::PrivateKey,
            name: String::from(name),
            id: uuid.to_string(),
            payload: payload,
            accounts: vec![]
        })
    }

    pub fn create_with_private_key_and_default_address(name: &str, password: &str, private_key: &str, coin: Coin) -> Result<StoredKey, Error> {
        PrivateKey::is_valid(private_key.as_bytes(), &coin.curve)?;
        let stored_key = StoredKey::create_with_private_key(name, password, private_key)?;

        let private_key_struct = PrivateKey::new(private_key.as_bytes())?;

        
        Ok(stored_key)
    }
}