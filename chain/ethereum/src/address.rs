use chain_common::address::Address;
use chain_common::coin::Coin;
use chain_common::public_key::PublicKey;
use crypto::Error;
use crypto::public_key::PublicKeyType;
use super::address_checksum;

pub struct EthereumAddress;

impl Address for EthereumAddress {
    fn derive_address(coin: &Coin, public_key: &PublicKey, p2pkh: &[u8], hrp: &[u8]) -> Result<String, Error> {
        if public_key.r#type != PublicKeyType::SECP256k1Extended {
            return Err(Error::NotSupportedPublicKeyType);
        }
        Ok("".to_owned())
    }
}