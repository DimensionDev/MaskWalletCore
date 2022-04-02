use super::address::EthereumAddress;

use crypto::hash::{Hasher, Keccak256};

pub enum ChecksumType {
    Eip55,
    WanChain,
}

pub fn checksum(address: &EthereumAddress, r#type: ChecksumType) -> String {
    let address_string = hex::encode(&address.data);
    let hash =
        Hasher::hash(Keccak256, address_string.as_bytes()).expect("Fail to do keccak256 hash");
    let hash_hex = hex::encode(hash);

    let mut prefix = "0x".to_owned();

    for i in 0..std::cmp::min(address_string.len(), hash_hex.len()) {
        let a = address_string.chars().nth(i).unwrap();
        let h = hash_hex.chars().nth(i).unwrap();

        if ('0'..='9').contains(&a) {
            prefix.push(a);
        } else if ('8'..='9').contains(&h) || ('a'..='f').contains(&h) {
            match r#type {
                ChecksumType::Eip55 => prefix.push(a.to_uppercase().next().unwrap()),
                ChecksumType::WanChain => prefix.push(a.to_lowercase().next().unwrap()),
            };
        } else {
            match r#type {
                ChecksumType::Eip55 => prefix.push(a.to_lowercase().next().unwrap()),
                ChecksumType::WanChain => prefix.push(a.to_uppercase().next().unwrap()),
            };
        }
    }

    prefix
}
