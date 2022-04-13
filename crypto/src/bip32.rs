use super::curve::Curve;
use crate::Error;
use bitcoin::secp256k1::Secp256k1;
use bitcoin::util::bip32::{ChildNumber, DerivationPath, ExtendedPrivKey, ExtendedPubKey};
use bitcoin::Network;
use ed25519_dalek_bip32;
use std::convert::TryInto;
use std::str::FromStr;

pub struct HdNode {
    pub depth: u8,
    pub child_num: u32,
    pub chain_code: [u8; 32],
    pub private_key_bytes: [u8; 32],
    pub private_key_extension: [u8; 32],
    pub public_key_bytes: [u8; 33],
    pub curve: Curve,
}

impl HdNode {
    pub fn new_from_extended_private_key(extended_priv_key: ExtendedPrivKey, curve: Curve) -> Self {
        let child_num = match extended_priv_key.child_number {
            ChildNumber::Normal { index } => index,
            ChildNumber::Hardened { index } => index,
        };
        let private_key_bytes: [u8; 32] =
            extended_priv_key.private_key.to_bytes().try_into().unwrap();
        HdNode {
            depth: extended_priv_key.depth,
            child_num,
            chain_code: extended_priv_key.chain_code.to_bytes(),
            private_key_bytes,
            private_key_extension: [0u8; 32],
            public_key_bytes: [0u8; 33],
            curve,
        }
    }

    pub fn get_node(seed: &[u8], path: &str, curve: Curve) -> Result<HdNode, Error> {
        let extended_master_key =
            ExtendedPrivKey::new_master(Network::Bitcoin, seed).map_err(|_| Error::InvalidSeed)?;
        let derivation_path =
            DerivationPath::from_str(path).map_err(|_| Error::InvalidDerivationpath)?;

        match curve {
            Curve::Ed25519 => {
                let path = ed25519_dalek_bip32::DerivationPath::from_str(path)
                    .map_err(|_| Error::InvalidDerivationpath)?;
                let extended_key = ed25519_dalek_bip32::ExtendedSecretKey::from_seed(seed)
                    .and_then(|extended| extended.derive(&path))
                    .map_err(|_| Error::InvalidSeed)?;
                let child_num = match path.path().last() {
                    Some(num) => num.to_u32(),
                    None => 0,
                };

                Ok(HdNode {
                    depth: extended_key.depth,
                    child_num,
                    chain_code: extended_key.chain_code,
                    private_key_bytes: extended_key.secret_key.to_bytes(),
                    private_key_extension: [0; 32],
                    public_key_bytes: [0u8; 33],
                    curve,
                })
            }
            _ => {
                let extended_private_key = extended_master_key
                    .derive_priv(&Secp256k1::new(), &derivation_path)
                    .map_err(|_| Error::InvalidSeed)?;

                Ok(HdNode::new_from_extended_private_key(
                    extended_private_key,
                    curve,
                ))
            }
        }
    }
}

pub fn get_extended_public_key(seed: &[u8], path: &str) -> Result<String, Error> {
    let extended_master_key =
        ExtendedPrivKey::new_master(Network::Bitcoin, seed).map_err(|_| Error::InvalidSeed)?;
    let derivation_path =
        DerivationPath::from_str(path).map_err(|_| Error::InvalidDerivationpath)?;
    let extended_private_key = extended_master_key
        .derive_priv(&Secp256k1::new(), &derivation_path)
        .map_err(|_| Error::InvalidSeed)?;
    let extended_public_key =
        ExtendedPubKey::from_private(&Secp256k1::new(), &extended_private_key);
    std::str::from_utf8(&extended_public_key.encode()[..])
        .map(|x| x.to_owned())
        .map_err(|_| Error::InvalidSeed)
}

#[cfg(test)]
mod tests {
    use crate::bip32::HdNode;
    use crate::curve::Curve;
    use bitcoin::secp256k1::Secp256k1;
    use bitcoin::util::bip32::{DerivationPath, ExtendedPrivKey};
    use bitcoin::Network;
    use hex;
    use std::str::FromStr;
    #[test]
    fn test_derive_from_seed() {
        let seed = "000102030405060708090a0b0c0d0e0f";
        let seed_bytes = hex::decode(seed).unwrap();

        let extended_private_key =
            ExtendedPrivKey::new_master(Network::Bitcoin, &seed_bytes).unwrap();
        assert_eq!(extended_private_key.to_string(), "xprv9s21ZrQH143K3QTDL4LXw2F7HEK3wJUD2nW2nRk4stbPy6cq3jPPqjiChkVvvNKmPGJxWUtg6LnF5kejMRNNU3TGtRBeJgk33yuGBxrMPHi");

        let derivation_path = "m/0'/1/2'/2";
        let curve = Curve::Secp256k1;
        let node = HdNode::get_node(&seed_bytes, derivation_path, curve).unwrap();
        assert_eq!(node.depth, 4);
        assert_eq!(node.child_num, 2);

        let extended_master_key =
            ExtendedPrivKey::new_master(Network::Bitcoin, &seed_bytes).unwrap();
        let derivation_path = DerivationPath::from_str(derivation_path).unwrap();
        let extended_private_key = extended_master_key
            .derive_priv(&Secp256k1::new(), &derivation_path)
            .unwrap();
        assert_eq!(extended_private_key.to_string(), "xprvA2JDeKCSNNZky6uBCviVfJSKyQ1mDYahRjijr5idH2WwLsEd4Hsb2Tyh8RfQMuPh7f7RtyzTtdrbdqqsunu5Mm3wDvUAKRHSC34sJ7in334");
        assert_eq!(
            node.private_key_bytes[..],
            extended_private_key.private_key.to_bytes()
        );
    }
}
