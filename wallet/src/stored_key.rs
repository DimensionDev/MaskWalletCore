use serde::{Deserialize, Serialize};
use std::str::FromStr;
use uuid::Uuid;

use super::account::Account;
use super::coin_dispatcher::CoinDispatcher;
use super::derivation_path::DerivationPath;
use super::encryption_params::EncryptionParams;
use super::hd_wallet::HdWallet;
use crate::Error;
use chain_common::api::{StoredKeyInfo, StoredKeyType as ProtoStoreKeyType};
use chain_common::coin::Coin;
use chain_common::private_key::PrivateKey;
use crypto::bip39::Mnemonic;
use crypto::hash;
use crypto::key_store_json::KeyStoreJson;
use crypto::Error as CryptoError;

const VERSION: &str = "0.1.0";

#[derive(Serialize, Deserialize, PartialEq)]
pub enum StoredKeyType {
    PrivateKey = 0,
    Mnemonic,
}

#[derive(Serialize, Deserialize)]
pub struct StoredKey {
    pub r#type: StoredKeyType,

    pub id: String,

    pub hash: String,

    pub version: String,

    payload: EncryptionParams,

    accounts: Vec<Account>,
}

// Create & Import function
impl StoredKey {
    fn create_with_data(
        r#type: StoredKeyType,
        password: &str,
        data: &[u8],
    ) -> Result<StoredKey, Error> {
        let uuid = Uuid::new_v4();
        let payload = EncryptionParams::new(password.as_bytes(), &data)?;
        let hash = match r#type {
            StoredKeyType::PrivateKey => hash::dsha256(&data),
            StoredKeyType::Mnemonic => {
                let mnemonic_str = std::str::from_utf8(&data)
                    .map_err(|_| Error::CryptoError(CryptoError::PasswordIncorrect))?;
                let mnemonic = Mnemonic::new(&mnemonic_str, &password)?;
                hash::dsha256(&mnemonic.seed)
            }
        };
        Ok(StoredKey {
            r#type,
            id: uuid.to_string(),
            hash: hex::encode(hash),
            version: VERSION.to_owned(),
            payload,
            accounts: vec![],
        })
    }

    pub fn create_with_private_key(password: &str, private_key: &str) -> Result<StoredKey, Error> {
        let priv_key_bytes =
            hex::decode(private_key).map_err(|_| CryptoError::InvalidPrivateKey)?;
        Self::create_with_data(StoredKeyType::PrivateKey, &password, &priv_key_bytes)
    }

    pub fn create_with_private_key_and_default_address(
        name: &str,
        password: &str,
        private_key: &str,
        coin: &Coin,
    ) -> Result<StoredKey, Error> {
        let priv_key_bytes =
            hex::decode(private_key).map_err(|_| CryptoError::InvalidPrivateKey)?;
        PrivateKey::is_valid(&priv_key_bytes, &coin.curve)?;
        let mut stored_key = StoredKey::create_with_private_key(password, private_key)?;

        let private_key_struct = PrivateKey::new(&priv_key_bytes)?;

        let public_key = private_key_struct.get_public_key(&coin.public_key_type)?;
        let address =
            CoinDispatcher::get_entry(&coin)?.derive_address(&coin, &public_key, &[], &[])?;
        stored_key.accounts.push(Account::new(
            &address,
            name,
            coin.clone(),
            &coin.derivation_path,
            "",
        )?);
        Ok(stored_key)
    }

    pub fn create_with_mnemonic(password: &str, mnemonic: &str) -> Result<StoredKey, Error> {
        if !Mnemonic::is_valid(mnemonic) {
            return Err(Error::CryptoError(CryptoError::InvalidMnemonic));
        }
        Self::create_with_data(StoredKeyType::Mnemonic, &password, &mnemonic.as_bytes())
    }

    pub fn create_with_mnemonic_random(password: &str) -> Result<(StoredKey, String), Error> {
        let wallet = HdWallet::new(12, "")?;
        let stored_key = Self::create_with_data(
            StoredKeyType::Mnemonic,
            &password,
            &wallet.mnemonic.as_bytes(),
        )?;
        Ok((stored_key, wallet.mnemonic))
    }

    pub fn create_with_json(
        name: &str,
        password: &str,
        json: &str,
        coin: &Coin,
    ) -> Result<StoredKey, Error> {
        let key_store_json_struct = KeyStoreJson::from_str(&json)?;
        let (_, decrypted) =
            EncryptionParams::new_from_json_struct(&key_store_json_struct, password.as_bytes())?;
        let decrypted_str = std::str::from_utf8(&decrypted)
            .map_err(|_| Error::CryptoError(CryptoError::InvalidKeyStoreJson))?;
        if Mnemonic::is_valid(&decrypted_str) {
            return Self::create_with_mnemonic(&password, &decrypted_str);
        }
        let private_key = PrivateKey::new(&decrypted)?;
        let private_key_hex = hex::encode(&private_key.data);
        Self::create_with_private_key_and_default_address(&name, &password, &private_key_hex, coin)
    }
}

// Update methods
impl StoredKey {
    pub fn update_password(&mut self, old_password: &str, new_password: &str) -> Result<(), Error> {
        let decrypted = self.payload.decrypt(&old_password.as_bytes())?;
        self.payload = EncryptionParams::new(new_password.as_bytes(), &decrypted)?;
        Ok(())
    }
}

// Export methods
impl StoredKey {
    pub fn export_private_key(&mut self, password: &str, coin: &Coin) -> Result<String, Error> {
        let private_key = self.decrypt_private_key(&password, &coin)?;
        Ok(private_key.to_string())
    }

    pub fn export_private_key_of_path(
        &mut self,
        password: &str,
        coin: &Coin,
        derivation_path: &str,
    ) -> Result<String, Error> {
        if self.r#type != StoredKeyType::Mnemonic {
            return Err(Error::RequestNotSupportedOnPrivateKeyTypeStoredKey);
        }
        let wallet = self.get_wallet(&password)?;
        let derivation_path = DerivationPath::new(&derivation_path)?;
        let private_key = wallet.get_key(&coin, &derivation_path)?;
        Ok(private_key.to_string())
    }

    pub fn export_mnemonic(&mut self, password: &str) -> Result<String, Error> {
        if self.r#type != StoredKeyType::Mnemonic {
            return Err(Error::RequestNotSupportedOnPrivateKeyTypeStoredKey);
        }
        let mnemonic_bytes = self.payload.decrypt(&password.as_bytes())?;
        let mnemonic = std::str::from_utf8(&mnemonic_bytes)
            .map_err(|_| Error::CryptoError(CryptoError::PasswordIncorrect))?;
        Ok(mnemonic.to_owned())
    }

    pub fn export_key_store_json(
        &mut self,
        password: &str,
        new_password: &str,
    ) -> Result<String, Error> {
        self.payload
            .export_to_key_store_json(&password, &new_password)
    }

    pub fn export_key_store_json_of_address(
        &mut self,
        password: &str,
        new_password: &str,
        coin: &Coin,
        address: &str,
    ) -> Result<String, Error> {
        if let Some(account) = self
            .accounts
            .iter()
            .find(|account| account.address == address)
        {
            if self.r#type == StoredKeyType::PrivateKey {
                // Convert the payload to KeyStoreJSON
                self.payload
                    .export_to_key_store_json(&password, &new_password)
            } else {
                // 1. If this StoredKey is created from a mnemonic, derive to the specific path to get the private key
                let wallet = self.get_wallet(&password)?;
                let private_key = wallet.get_key(&coin, &account.derivation_path)?;

                // 2. Create a temp EncryptionParam with new password for exporting
                let temp_encryption_param =
                    EncryptionParams::new(new_password.as_bytes(), &private_key.data)?;
                temp_encryption_param.export_to_key_store_json(&new_password, &new_password)
            }
        } else {
            Err(Error::RequstedAccountNotFound)
        }
    }

    pub fn export_key_store_json_of_path(
        &mut self,
        password: &str,
        new_password: &str,
        coin: &Coin,
        derivation_path: &str,
    ) -> Result<String, Error> {
        if self.r#type == StoredKeyType::PrivateKey {
            // 1. If this StoredKey is created by importing a private key, simply export it
            return self
                .payload
                .export_to_key_store_json(&password, &new_password);
        }
        // 2. If this StoredKey is created from a mnemonic, derive to the specific path to get the private key
        let wallet = self.get_wallet(&password)?;
        let derivation_path = DerivationPath::new(&derivation_path)?;
        let private_key = wallet.get_key(&coin, &derivation_path)?;

        // 3. Create a temp EncryptionParam with new password for exporting
        let temp_encryption_param =
            EncryptionParams::new(new_password.as_bytes(), &private_key.data)?;
        temp_encryption_param.export_to_key_store_json(&new_password, &new_password)
    }
}

// Get Hd Wallet
impl StoredKey {
    fn get_wallet(&self, password: &str) -> Result<HdWallet, Error> {
        if self.r#type != StoredKeyType::Mnemonic {
            return Err(Error::RequestNotSupportedOnPrivateKeyTypeStoredKey);
        }
        let mnemonic_bytes = self.payload.decrypt(&password.as_bytes())?;
        let mnemonic = std::str::from_utf8(&mnemonic_bytes)
            .map_err(|_| Error::CryptoError(CryptoError::PasswordIncorrect))?;
        HdWallet::new_with_mnemonic(&mnemonic, "")
    }
}

// Account related methods
impl StoredKey {
    pub fn get_accounts_count(&self) -> u32 {
        self.accounts.len() as u32
    }

    pub fn get_account(&self, index: u32) -> Result<&Account, Error> {
        let index = index as usize;
        if index >= self.accounts.len() {
            return Err(Error::IndexOutOfBounds);
        }
        Ok(&self.accounts[index as usize])
    }

    pub fn get_all_accounts(&self) -> &[Account] {
        &self.accounts
    }

    pub fn get_accounts_of_coin(&self, coin: &Coin) -> Vec<Account> {
        self.accounts
            .iter()
            .filter(|account| account.coin == *coin)
            .cloned()
            .collect()
    }

    fn get_or_create_account_for_coin(
        &mut self,
        name: &str,
        coin: &Coin,
        hd_wallet: &HdWallet,
    ) -> Result<Option<&Account>, Error> {
        for (i, account) in self.accounts.iter_mut().enumerate() {
            if account.coin == *coin {
                // Found an account of required coin
                if account.address.is_empty() {
                    account.address = hd_wallet.get_address_for_coin(&coin)?;
                }
                return Ok(self.accounts.get(i));
            }
        }
        // No valid account found for the coin, create a new one
        let address = hd_wallet.get_address_for_coin(&coin)?;
        let extended_public_key = hd_wallet.get_extended_public_key(&coin);
        self.accounts.push(Account::new(
            &address,
            name,
            coin.clone(),
            &coin.derivation_path,
            &extended_public_key,
        )?);
        Ok(self.accounts.last())
    }

    pub fn add_new_account_of_coin_and_derivation_path_by_password(
        &mut self,
        name: &str,
        coin: Coin,
        derivation_path: &str,
        password: &str,
    ) -> Result<Account, Error> {
        let derivation_path_struct = DerivationPath::new(&derivation_path)?;
        let find_account_block = |account: &Account| {
            account.coin == coin && account.derivation_path == derivation_path_struct
        };
        if self.accounts.iter().any(find_account_block) {
            Err(Error::AccountAlreadyExist)
        } else {
            let wallet = self.get_wallet(&password)?;
            let address = wallet.get_address_for_coin_of_path(&coin, &derivation_path)?;
            let extended_public_key =
                wallet.get_extended_public_key_of_path(&coin, &derivation_path);
            self.accounts.push(Account::new(
                &address,
                name,
                coin.clone(),
                &derivation_path,
                &extended_public_key,
            )?);
            Ok(self.accounts.last().unwrap().clone())
        }
    }

    pub fn update_account_name_of_address(
        &mut self,
        address: &str,
        name: &str,
    ) -> Result<Account, Error> {
        if let Some(account) = self
            .accounts
            .iter_mut()
            .find(|account| account.address == address)
        {
            account.name = name.to_owned();
            Ok(account.clone())
        } else {
            Err(Error::RequstedAccountNotFound)
        }
    }

    pub fn remove_accounts_of_coin(&mut self, coin: &Coin) {
        self.accounts.retain(|account| account.coin != *coin);
    }

    pub fn remove_account_of_address(&mut self, address: &str, coin: &Coin) -> Result<(), Error> {
        if !self
            .accounts
            .iter()
            .any(|account| account.address != address && account.coin != *coin)
        {
            return Err(Error::RequstedAccountNotFound);
        }
        self.accounts
            .retain(|account| account.address != address && account.coin != *coin);
        Ok(())
    }
}

// Decrypt methods
impl StoredKey {
    pub fn validate_password(&self, password: &str) -> bool {
        self.payload.decrypt(password.as_bytes()).is_ok()
    }

    pub fn decrypt_private_key(
        &mut self,
        password: &str,
        coin: &Coin,
    ) -> Result<PrivateKey, Error> {
        match self.r#type {
            StoredKeyType::Mnemonic => {
                let wallet = self.get_wallet(&password)?;
                let account = self.get_or_create_account_for_coin("", &coin, &wallet)?;
                if account.is_none() {
                    return Err(Error::CryptoError(CryptoError::PasswordIncorrect));
                }
                wallet.get_key(&coin, &account.unwrap().derivation_path)
            }
            StoredKeyType::PrivateKey => {
                let decrypted = self.payload.decrypt(&password.as_bytes())?;
                Ok(PrivateKey::new(&decrypted)?)
            }
        }
    }
}

// Sign methods
impl StoredKey {
    pub fn sign(
        &mut self,
        coin: &Coin,
        password: &str,
        address: &str,
        payload: &[u8],
    ) -> Result<Vec<u8>, Error> {
        let account = match self
            .accounts
            .iter()
            .find(|account| account.address == address)
        {
            Some(account) => account,
            None => return Err(Error::RequstedAccountNotFound),
        };

        let private_key = match self.r#type {
            StoredKeyType::Mnemonic => {
                let wallet = self.get_wallet(&password)?;
                wallet.get_key(&coin, &account.derivation_path)?
            }
            StoredKeyType::PrivateKey => {
                let decrypted = self.payload.decrypt(&password.as_bytes())?;
                PrivateKey::new(&decrypted)?
            }
        };
        Ok(CoinDispatcher::get_entry(&coin)?.sign(&coin, &private_key, &payload)?)
    }
}

impl From<StoredKey> for StoredKeyInfo {
    fn from(stored_key: StoredKey) -> Self {
        let json = serde_json::to_vec(&stored_key).unwrap().to_vec();
        StoredKeyInfo {
            data: json,
            id: stored_key.id,
            hash: stored_key.hash,
            r#type: stored_key.r#type as i32,
        }
    }
}

impl From<StoredKeyType> for ProtoStoreKeyType {
    fn from(stored_key_type: StoredKeyType) -> Self {
        match stored_key_type {
            StoredKeyType::PrivateKey => ProtoStoreKeyType::PrivateKey,
            StoredKeyType::Mnemonic => ProtoStoreKeyType::Mnemonic,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chain_common::coin::Coin;
    use std::collections::HashMap;
    #[test]
    fn test_create_stored_key_with_private_key() {
        let priv_key_str = "3a1076bf45ab87712ad64ccb3b10217737f7faacbf2872e88fdd9a537d8fe266";
        let password = "mask wallet";
        let derivation_path = "m/44'/60'/0'/0/0";
        let coin = Coin {
            id: "60".to_owned(),
            name: "ethereum".to_owned(),
            coin_id: 60,
            symbol: "ETH".to_owned(),
            decimals: 18,
            blockchain: "Ethereum".to_owned(),
            derivation_path: derivation_path.to_owned(),
            curve: "secp256k1".to_owned(),
            public_key_type: "secp256k1Extended".to_owned(),
            all_info: HashMap::new(),
        };

        let stored_key = StoredKey::create_with_private_key_and_default_address(
            "mask",
            &password,
            priv_key_str,
            &coin,
        )
        .unwrap();
        assert_eq!(stored_key.get_accounts_count(), 1);
        assert_eq!(stored_key.version, VERSION);
        let account = stored_key.get_account(0).unwrap();
        assert_eq!(
            account.address,
            "0xC2D7CF95645D33006175B78989035C7c9061d3F9"
        );
        assert_eq!(account.derivation_path.to_string(), derivation_path);
        assert_eq!(account.extended_public_key, "");
    }

    #[test]
    fn test_create_with_mnemonic() {
        let mnemonic =
            "team engine square letter hero song dizzy scrub tornado fabric divert saddle";
        let password = "";
        let derivation_path = "m/44'/60'/0'/0/0";
        let coin = Coin {
            id: "60".to_owned(),
            name: "ethereum".to_owned(),
            coin_id: 60,
            symbol: "ETH".to_owned(),
            decimals: 18,
            blockchain: "Ethereum".to_owned(),
            derivation_path: derivation_path.to_owned(),
            curve: "secp256k1".to_owned(),
            public_key_type: "secp256k1Extended".to_owned(),
            all_info: HashMap::new(),
        };

        let mut stored_key = StoredKey::create_with_mnemonic(&password, &mnemonic).unwrap();
        stored_key
            .add_new_account_of_coin_and_derivation_path_by_password(
                "mask",
                coin,
                &derivation_path,
                &password,
            )
            .unwrap();
        assert_eq!(stored_key.r#type == StoredKeyType::Mnemonic, true);
        assert_eq!(stored_key.get_accounts_count(), 1);
        let decrypted = stored_key.payload.decrypt(password.as_bytes()).unwrap();
        assert_eq!(&decrypted, mnemonic.as_bytes());
        let account = stored_key.get_account(0).unwrap();
        assert_eq!(
            account.address,
            "0x494f60cb6Ac2c8F5E1393aD9FdBdF4Ad589507F7"
        );
        assert_eq!(account.derivation_path.to_string(), derivation_path);
        assert_eq!(account.coin.name, "ethereum");
        assert_eq!(account.extended_public_key, "");
    }

    #[test]
    fn test_create_with_json() {
        let _json = r#""#;
    }

    #[test]
    fn test_create_account_at_path() {
        let mnemonic =
            "suffer artefact burst review network fantasy easy century mom unique pupil boy";
        let password = "";
        let derivation_path = "m/44'/60'/0'/0/0";
        let coin = Coin {
            id: "60".to_owned(),
            name: "ethereum".to_owned(),
            coin_id: 60,
            symbol: "ETH".to_owned(),
            decimals: 18,
            blockchain: "Ethereum".to_owned(),
            derivation_path: derivation_path.to_owned(),
            curve: "secp256k1".to_owned(),
            public_key_type: "secp256k1Extended".to_owned(),
            all_info: HashMap::new(),
        };

        let mut stored_key = StoredKey::create_with_mnemonic(&password, &mnemonic).unwrap();
        let test_derivation_path1 = "m/44'/60'/0'/0/1";
        let account1 = stored_key
            .add_new_account_of_coin_and_derivation_path_by_password(
                "mask",
                coin,
                &test_derivation_path1,
                &password,
            )
            .unwrap();
        assert_eq!(account1.derivation_path.to_string(), test_derivation_path1);
    }

    #[test]
    fn test_hash() {
        let mnemonic1 =
            "suffer artefact burst review network fantasy easy century mom unique pupil boy";
        let password = "";
        let stored_key1 = StoredKey::create_with_mnemonic(&password, &mnemonic1).unwrap();
        let stored_key2 = StoredKey::create_with_mnemonic(&password, &mnemonic1).unwrap();
        assert_eq!(stored_key1.hash, stored_key2.hash);

        let (stored_key_random, _) = StoredKey::create_with_mnemonic_random(&password).unwrap();
        assert_ne!(stored_key1.hash, stored_key_random.hash);
        assert_ne!(stored_key2.hash, stored_key_random.hash);
    }
}
