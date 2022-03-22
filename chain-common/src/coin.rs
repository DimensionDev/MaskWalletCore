use crate::api::Coin as ProtoCoin;
use crate::Error;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    pub all_info: HashMap<String, serde_json::Value>,
}

impl Coin {
    pub fn get_value(&self, key: &str) -> Option<String> {
        self.all_info.get(key).map(|x| x.to_string())
    }

    pub fn get_xpub(&self) -> Option<String> {
        self.all_info.get("xpub").map(|x| x.to_string())
    }
}

impl PartialEq for Coin {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for Coin {}

impl std::fmt::Display for ProtoCoin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::str::FromStr for ProtoCoin {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "ethereum" => Ok(ProtoCoin::Ethereum),
            "polkadot" => Ok(ProtoCoin::Polkadot),
            "solana" => Ok(ProtoCoin::Solana),
            _ => Err(Error::NotSupportedCoin),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::api::Coin as ProtoCoin;
    #[test]
    fn test_proto_coin_into_str() {
        assert_eq!(ProtoCoin::Ethereum.to_string(), "Ethereum");
    }
}
