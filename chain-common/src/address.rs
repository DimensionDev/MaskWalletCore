use super::coin::Coin;
use super::public_key::PublicKey;
use crypto::Error;

pub trait Address {
    fn derive_address(coin: &Coin, public_key: &PublicKey, p2pkh: &[u8], hrp: &[u8]) -> Result<String, Error>;
}