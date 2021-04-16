use super::public_key::PublicKey;
use super::private_key::PrivateKey;
use crypto::Error as CryptoError;

type Error = CryptoError;

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
    pub fn derive_address(&self, private_key: &PrivateKey) -> Result<String, Error> {
        let public_key = private_key.get_public_key(&self.public_key_type)?;
        self.derive_address_from_pub(&public_key)
    }

    pub fn derive_address_from_pub(&self, public_key: &PublicKey) -> Result<String, Error> {
        
        Ok("".to_owned())
    }
}