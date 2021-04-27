use std::collections::HashMap;
use super::coin::Coin;
use super::public_key::PublicKey;
use super::private_key::PrivateKey;
use crypto::Error;

pub trait Entry {
    fn derive_address(&self, coin: &Coin, public_key: &PublicKey, p2pkh: &[u8], hrp: &[u8]) -> Result<String, Error>;
    // fn sign(&self, coin: &Coin, private_key: &PrivateKey, input: HashMap<String, String>);
}