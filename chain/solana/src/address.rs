use bitcoin::util::base58;
use chain_common::public_key::PublicKey;
use crypto::public_key::PublicKeyType;
use crypto::Error;

const ADDRESS_SIZE: usize = 32;

pub struct SolanaAddress {
    pub data: Vec<u8>,
}

impl SolanaAddress {
    pub fn is_valid(address: &str) -> bool {
        let data = match base58::from(&address) {
            Ok(data) => data,
            Err(_) => return false,
        };
        data.len() == ADDRESS_SIZE
    }

    pub fn new(public_key: &PublicKey) -> Result<Self, Error> {
        if public_key.r#type != PublicKeyType::Ed25519 {
            return Err(Error::NotSupportedPublicKeyType);
        }
        Ok(SolanaAddress {
            data: public_key.data.to_vec(),
        })
    }
}

impl ToString for SolanaAddress {
    fn to_string(&self) -> String {
        base58::encode_slice(&self.data)
    }
}

#[cfg(test)]
mod tests {
    use crate::address::SolanaAddress;
    use bitcoin::util::base58;
    use chain_common::public_key::PublicKey;
    use crypto::public_key::PublicKeyType;

    #[test]
    fn test_validate_address() {
        let test1 = "abc";
        let test2 = "2gVkYWexTHR5Hb2aLeQN3tnngvWzisFKXDUPrgMHpd";
        let test3 = "2gVkYWexTHR5Hb2aLeQN3tnngvWzisFKXDUPrgMHpdSl";

        let test4 = "2gVkYWexTHR5Hb2aLeQN3tnngvWzisFKXDUPrgMHpdST";

        assert_eq!(SolanaAddress::is_valid(&test1), false);
        assert_eq!(SolanaAddress::is_valid(&test2), false);
        assert_eq!(SolanaAddress::is_valid(&test3), false);
        assert_eq!(SolanaAddress::is_valid(&test4), true);
    }

    #[test]
    fn test_derive_from_pub_key() {
        let pub_key_str = "2gVkYWexTHR5Hb2aLeQN3tnngvWzisFKXDUPrgMHpdST";

        let pub_key_data = base58::from(&pub_key_str).unwrap();

        let public_key = PublicKey {
            r#type: PublicKeyType::Ed25519,
            data: pub_key_data.to_vec(),
        };
        let address = SolanaAddress::new(&public_key);
        assert_eq!(address.is_ok(), true);
        let address_str = address.unwrap().to_string();
        assert_eq!(address_str, "2gVkYWexTHR5Hb2aLeQN3tnngvWzisFKXDUPrgMHpdST");
    }
}
