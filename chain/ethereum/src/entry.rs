use chain_common::entry::Entry;
use chain_common::public_key::PublicKey;
use chain_common::coin::Coin;
use crypto::Error;
use super::address::EthereumAddress;

pub struct EthereumEntry;

impl Entry for EthereumEntry {
    fn derive_address(&self, coin: &Coin, public_key: &PublicKey, _p2pkh: &[u8], _hrp: &[u8]) -> Result<String, Error> {
        let address = EthereumAddress::new(public_key, &coin.id)?;
        Ok(address.to_string())
    }
}