use chain_common::entry::Entry;
use chain_common::coin::Coin;
use chain_common::public_key::PublicKey;
use chain_common::private_key::PrivateKey;
use crate::Error;
use ethereum::entry::EthereumEntry;

pub fn get_dispatcher(coin: &Coin) -> Box<dyn Entry> {
    match coin.name.as_str() {
        "ethereum" => Box::new(EthereumEntry{}),
        _ => Box::new(EthereumEntry{})
    }
}

pub fn derive_address_with_private_key(coin: &Coin, private_key: &PrivateKey) -> Result<String, Error> {
    let public_key = private_key.get_public_key(&coin.public_key_type)?;
    derive_address_with_public_key(&coin, &public_key)
}

pub fn derive_address_with_public_key(coin: &Coin, public_key: &PublicKey) -> Result<String, Error> {
    let p2pkh = coin.get_value("p2pkh").unwrap_or_default();
    let hrp = coin.get_value("hrp").unwrap_or_default();
    Ok(get_dispatcher(&coin).derive_address(&coin, &public_key, p2pkh.as_bytes(), hrp.as_bytes())?)
}