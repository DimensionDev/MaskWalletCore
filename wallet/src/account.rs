use serde::{ Serialize, Deserialize };
use chain_common::coin::Coin;
use super::derivation_path::DerivationPath;
use crate::Error;

#[derive(Serialize, Deserialize, Clone)]
pub struct Account {
    pub address: String,
    pub coin: Coin,
    pub derivation_path: DerivationPath,
    pub extended_public_key: String,
}

impl Account {
    pub fn new(address: &str, coin: Coin, derivation_path: &str, extended_public_key: &str) -> Result<Self, Error> {
        let derivation_path_struct = DerivationPath::new(&derivation_path)?;
        Ok(Account {
            address: address.to_owned(),
            coin,
            derivation_path: derivation_path_struct,
            extended_public_key: extended_public_key.to_owned(),
        })
    }
}