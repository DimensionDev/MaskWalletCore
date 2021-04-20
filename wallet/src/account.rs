use serde::{ Serialize, Deserialize };
use chain_common::coin::Coin;
use super::derivation_path::DerivationPath;

#[derive(Serialize, Deserialize)]
pub struct Account {
    pub address: String,
    pub coin: Coin,
    pub derivation_path: DerivationPath,
    pub extended_public_key: String,
}