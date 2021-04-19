use super::coin::Coin;
use super::public_key::PublicKey;
use crypto::Error;

pub trait Entry {
    fn derive_address(&self, coin: &Coin, public_key: &PublicKey, p2pkh: &[u8], hrp: &[u8]) -> Result<String, Error>;
}