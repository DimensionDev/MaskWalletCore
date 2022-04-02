use super::derivation_path::DerivationPath;
use crate::Error;
use chain_common::api::StoredKeyAccountInfo;
use chain_common::coin::Coin;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Account {
    pub address: String,
    pub name: String,
    pub coin: Coin,
    pub derivation_path: DerivationPath,
    pub extended_public_key: String,
}

impl Account {
    pub fn new(
        address: &str,
        name: &str,
        coin: Coin,
        derivation_path: &str,
        extended_public_key: &str,
    ) -> Result<Self, Error> {
        let derivation_path_struct = DerivationPath::new(derivation_path)?;
        Ok(Account {
            address: address.to_owned(),
            name: name.to_owned(),
            coin,
            derivation_path: derivation_path_struct,
            extended_public_key: extended_public_key.to_owned(),
        })
    }
}

impl From<&Account> for StoredKeyAccountInfo {
    fn from(account: &Account) -> Self {
        StoredKeyAccountInfo {
            address: account.address.to_owned(),
            name: account.name.to_owned(),
            derivation_path: account.derivation_path.to_string(),
            coin: account.coin.id.to_owned(),
            extended_public_key: account.extended_public_key.to_owned(),
        }
    }
}
