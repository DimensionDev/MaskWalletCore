use super::address_checksum::{checksum, ChecksumType};
use chain_common::public_key::PublicKey;
use crypto::hash::Keccak256;
use crypto::public_key::PublicKeyType;
use crypto::Error;
use std::string::ToString;

const ADDRESS_SIZE: usize = 20;

pub struct EthereumAddress {
    pub coin_id: String,
    pub data: Vec<u8>,
}

impl EthereumAddress {
    pub fn is_valid(address: &str) -> bool {
        if address.is_empty() {
            return false;
        }
        if !address.starts_with("0x") {
            return false;
        }
        if address.len() != 42 {
            return false;
        }
        let data = match hex::decode(&address[2..]) {
            Ok(data) => data,
            Err(_) => return false,
        };
        data.len() == ADDRESS_SIZE
    }

    pub fn new(public_key: &PublicKey, coin_id: &str) -> Result<Self, Error> {
        if public_key.r#type != PublicKeyType::Secp256k1Extended {
            return Err(Error::NotSupportedPublicKeyType);
        }
        let hash = public_key.hash(&[], Keccak256, true)?;
        let begin = hash.len() - ADDRESS_SIZE;
        Ok(EthereumAddress {
            coin_id: coin_id.to_owned(),
            data: hash[begin..].to_vec(),
        })
    }
}

impl ToString for EthereumAddress {
    fn to_string(&self) -> String {
        let checksum_type: ChecksumType = match self.coin_id.to_lowercase().as_str() {
            "ethereum" => ChecksumType::Eip55,
            "wanchain" => ChecksumType::WanChain,
            _ => ChecksumType::Eip55,
        };
        checksum(&self, checksum_type)
    }
}

#[cfg(test)]
mod tests {
    use crate::address::EthereumAddress;
    use chain_common::public_key::PublicKey;
    use crypto::public_key::PublicKeyType;

    #[test]
    fn test_validate_address() {
        let test1 = "abc";
        let test2 = "aaeb60f3e94c9b9a09f33669435e7ef1beaed";
        let test3 = "fB6916095ca1df60bB79Ce92cE3Ea74c37c5d359";

        let test4 = "0x5aAeb6053F3E94C9b9A09f33669435E7Ef1BeAed";

        assert_eq!(EthereumAddress::is_valid(&test1), false);
        assert_eq!(EthereumAddress::is_valid(&test2), false);
        assert_eq!(EthereumAddress::is_valid(&test3), false);
        assert_eq!(EthereumAddress::is_valid(&test4), true);
    }

    #[test]
    fn test_derive_from_pub_key() {
        let pub_key_str = "0499c6f51ad6f98c9c583f8e92bb7758ab2ca9a04110c0a1126ec43e5453d196c166b489a4b7c491e7688e6ebea3a71fc3a1a48d60f98d5ce84c93b65e423fde91";

        let pub_key_data = hex::decode(pub_key_str).unwrap();

        let public_key = PublicKey {
            r#type: PublicKeyType::Secp256k1Extended,
            data: pub_key_data.to_vec(),
        };
        let address = EthereumAddress::new(&public_key, "ethereum");
        assert_eq!(address.is_ok(), true);
        let address_str = address.unwrap().to_string();
        assert_eq!(address_str, "0xAc1ec44E4f0ca7D172B7803f6836De87Fb72b309");
    }
}
