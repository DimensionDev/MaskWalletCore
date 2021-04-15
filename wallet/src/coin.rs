use super::public_key::PublicKey;

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

impl Coin {
    pub fn derive_address(&self, private_key: &[u8]) {

    }
}