use serde::{ Serialize, Deserialize };

#[derive(Clone, Serialize, Deserialize)]
pub struct Coin {
    
    pub id: String,
    
    pub name: String,
    
    pub coin_id: i32,
    
    pub symbol: String,
    
    pub decimals: i32,
    
    pub blockchain: String,
    
    pub derivation_path: String,
    
    pub curve: String,
    
    pub public_key_type: String,
}