use crate::Error;
use chain_common::api::Coin as ProtoCoinType;
use chain_common::coin::Coin;
use chain_common::entry::Entry;
use chain_common::private_key::PrivateKey;
use chain_common::public_key::PublicKey;
use chain_common::Error as ChainError;
use ethereum::entry::EthereumEntry;
use solana::entry::SolanaEntry;
use std::str::FromStr;

pub struct CoinDispatcher {}

impl CoinDispatcher {
    pub fn get_entry(coin: &Coin) -> Result<Box<dyn Entry>, Error> {
        let coin_proto_type = ProtoCoinType::from_str(&coin.name)?;
        match coin_proto_type {
            ProtoCoinType::Ethereum => Ok(Box::new(EthereumEntry {})),
            ProtoCoinType::Solana => Ok(Box::new(SolanaEntry {})),
            _ => Err(Error::ChainError(ChainError::NotSupportedCoin)),
        }
    }
}

pub fn derive_address_with_private_key(
    coin: &Coin,
    private_key: &PrivateKey,
) -> Result<String, Error> {
    let public_key = private_key.get_public_key(&coin.public_key_type)?;
    derive_address_with_public_key(&coin, &public_key)
}

pub fn derive_address_with_public_key(
    coin: &Coin,
    public_key: &PublicKey,
) -> Result<String, Error> {
    let p2pkh = coin.get_value("p2pkh").unwrap_or_default();
    let hrp = coin.get_value("hrp").unwrap_or_default();
    Ok(CoinDispatcher::get_entry(&coin)?.derive_address(
        &coin,
        &public_key,
        p2pkh.as_bytes(),
        hrp.as_bytes(),
    )?)
}
