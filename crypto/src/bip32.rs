use std::str::FromStr;
use std::convert::TryInto;
use bitcoin::Network;
use bitcoin::util::bip32::{ ExtendedPrivKey, DerivationPath, ChildNumber };
use secp256k1::Secp256k1;
use super::curve::Curve;
use crate::Error;

pub struct HdNode {
    pub depth: u8,
    pub child_num: u32,
    pub chain_code: [u8; 32],
    pub private_key_bytes: [u8; 32],
    pub private_key_extension: [u8; 32],
    pub public_key_bytes: [u8; 33],
    pub curve: Curve
}

impl HdNode {
    pub fn new_from_extended_private_key(extended_priv_key: ExtendedPrivKey, curve: Curve) -> Self {
        let child_num = match extended_priv_key.child_number {
            ChildNumber::Normal { index } => index,
            ChildNumber::Hardened { index } => index,
        };
        let private_key_bytes: [u8; 32] = extended_priv_key.private_key.to_bytes().try_into().unwrap();
        HdNode {
            depth: extended_priv_key.depth,
            child_num,
            chain_code: extended_priv_key.chain_code.to_bytes(),
            private_key_bytes,
            private_key_extension: [0u8; 32],
            public_key_bytes: [0u8; 33],
            curve
        }
    }
}

pub fn get_master_node(seed: &[u8], curve: Curve) -> Result<HdNode, Error> {
    let extended_private_key = ExtendedPrivKey::new_master(Network::Bitcoin, &seed).map_err(|_| Error::InvalidSeed)?;
    Ok(HdNode::new_from_extended_private_key(extended_private_key, curve))
}

pub fn get_node(seed: &[u8], path: &str, curve: Curve) -> Result<HdNode, Error> {
    let extended_master_key = ExtendedPrivKey::new_master(Network::Bitcoin, &seed).map_err(|_| Error::InvalidSeed)?;
    let derivation_path = DerivationPath::from_str(path).map_err(|_| Error::InvalidDerivationpath)?;
    let extended_private_key = extended_master_key.derive_priv(&Secp256k1::new(), &derivation_path).map_err(|_| Error::InvalidSeed)?;
    Ok(HdNode::new_from_extended_private_key(extended_private_key, curve))
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use hex;
    use bitcoin::util::bip32::{ ExtendedPrivKey, DerivationPath };
    use bitcoin::Network;
    use secp256k1::Secp256k1;
    use crate::bip32::{ get_master_node, get_node };
    use crate::curve::Curve;
    #[test]
    fn test_derive_from_seed() {
        let seed = "000102030405060708090a0b0c0d0e0f";
        let curve = Curve::Secp256k1;
        let seed_bytes = hex::decode(seed).unwrap();
        let master_node = get_master_node(&seed_bytes, curve).unwrap();
        assert_eq!(master_node.depth, 0);
        assert_eq!(master_node.child_num, 0);
        
        let extended_private_key = ExtendedPrivKey::new_master(Network::Bitcoin, &seed_bytes).unwrap();
        assert_eq!(extended_private_key.to_string(), "xprv9s21ZrQH143K3QTDL4LXw2F7HEK3wJUD2nW2nRk4stbPy6cq3jPPqjiChkVvvNKmPGJxWUtg6LnF5kejMRNNU3TGtRBeJgk33yuGBxrMPHi");
        assert_eq!(master_node.private_key_bytes[..], extended_private_key.private_key.to_bytes());
        
        let derivation_path = "m/0'/1/2'/2";
        let curve = Curve::Secp256k1;
        let node = get_node(&seed_bytes, &derivation_path, curve).unwrap();
        assert_eq!(node.depth, 4);
        assert_eq!(node.child_num, 2);

        let extended_master_key = ExtendedPrivKey::new_master(Network::Bitcoin, &seed_bytes).unwrap();
        let derivation_path = DerivationPath::from_str(derivation_path).unwrap();
        let extended_private_key = extended_master_key.derive_priv(&Secp256k1::new(), &derivation_path).unwrap();
        assert_eq!(extended_private_key.to_string(), "xprvA2JDeKCSNNZky6uBCviVfJSKyQ1mDYahRjijr5idH2WwLsEd4Hsb2Tyh8RfQMuPh7f7RtyzTtdrbdqqsunu5Mm3wDvUAKRHSC34sJ7in334");
        assert_eq!(node.private_key_bytes[..], extended_private_key.private_key.to_bytes());
    }
}

