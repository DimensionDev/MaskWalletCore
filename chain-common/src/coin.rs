use serde::{ Serialize, Deserialize };

use super::public_key::PublicKey;
use super::private_key::PrivateKey;
use super::entry::Entry;
use crypto::Error as CryptoError;

type Error = CryptoError;

#[derive(Serialize, Deserialize)]
pub struct Coin {
    
    pub id: String,
    
    pub name: String,
    
    pub coin_id: i32,
    
    pub symbol: String,
    
    pub decimal: i32,
    
    pub blockchain: String,
    
    pub derivation_path: String,
    
    pub curve: String,
    
    pub public_key_type: String,
}