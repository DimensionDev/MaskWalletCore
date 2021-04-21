use std::collections::HashMap;
use serde::{ Serialize, Deserialize };
use serde_json;

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

    pub all_info: HashMap<String, serde_json::Value>,
}

impl Coin {
    pub fn get_xpub(&self) -> Option<String> {
        self.all_info.get("xpub").map(|x| x.to_string() )
    }
}