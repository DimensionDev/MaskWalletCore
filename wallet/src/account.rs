use serde::{ Serialize, Deserialize };
use chain_common::coin::Coin;

#[derive(Serialize, Deserialize)]
pub struct Account {
    pub address: String,
    pub coin: Coin,
    pub derivation_path: String,
}