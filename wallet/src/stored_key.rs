use uuid::Uuid;
use serde::{ Serialize, Deserialize };

use crate::Error;
use crypto::Error as CryptoError;
use super::account::Account;
use super::encryption_params::{ EncryptionParams };
use super::coin_dispatcher::get_dispatcher;
use chain_common::coin::Coin;
use chain_common::private_key::PrivateKey;

#[derive(Serialize, Deserialize)]
pub enum StoredKeyType {
    PrivateKey = 1,
    Mnemonic,
}

#[derive(Serialize, Deserialize)]
pub struct StoredKey {
    pub r#type: StoredKeyType,

    pub id: String,

    pub name: String,

    payload: EncryptionParams,

    accounts: Vec<Account>,
}

impl StoredKey {
    pub fn create_with_private_key(name: &str, password: &str, private_key: &str) -> Result<StoredKey, Error> {
        let uuid = Uuid::new_v4();
        let payload = EncryptionParams::new(password.as_bytes(), private_key.as_bytes())?;
        Ok(StoredKey {
            r#type: StoredKeyType::PrivateKey,
            name: String::from(name),
            id: uuid.to_string(),
            payload: payload,
            accounts: vec![]
        })
    }

    pub fn create_with_private_key_and_default_address(name: &str, password: &str, private_key: &str, coin: Coin) -> Result<StoredKey, Error> {
        let priv_key_bytes = hex::decode(private_key).map_err(|_| CryptoError::InvalidPrivateKey)?;
        PrivateKey::is_valid(&priv_key_bytes, &coin.curve)?;
        let mut stored_key = StoredKey::create_with_private_key(name, password, private_key)?;

        let private_key_struct = PrivateKey::new(&priv_key_bytes)?;

        let public_key = private_key_struct.get_public_key(&coin.public_key_type)?;
        let address = get_dispatcher(&coin).derive_address(&coin, &public_key, &[], &[])?;
        
        stored_key.accounts.push(Account {
            address: address,
            coin: coin,
            derivationPath: "".to_owned(),
        });
        Ok(stored_key)
    }
}