use std::str::FromStr;
use crypto::curve::Curve;
use crypto::Error as CryptoError;
use crypto::bip39;
use crypto::bip32;
use chain_common::coin::Coin;
use chain_common::private_key::{ PrivateKey, PrivateKeyType };
use crate::Error;
use super::derivation_path::DerivationPath;
use super::coin_dispatcher::get_dispatcher;

pub struct HdWallet {
    seed: Vec<u8>,
    pub mnemonic: String,
    passphrase: String,
    entropy: Vec<u8>
}

impl HdWallet {
    pub fn is_valid(mnemonic: &str) -> bool {
        bip39::check_mnemonic(mnemonic)
    }

    pub fn new(strength: u32, passphrase: &str) -> Result<HdWallet, Error> {
        if strength % 32 != 0 || strength < 128 || strength > 256 {
            return Err(Error::InvalidWalletStrength);
        }
        let mnemonic = bip39::generate(strength / 8, passphrase)?;
        Ok(HdWallet {
            seed: mnemonic.seed,
            mnemonic: mnemonic.words,
            passphrase: passphrase.to_owned(),
            entropy: mnemonic.entropy
        })
    }
}

impl HdWallet {
    fn get_key(&self, coin: &Coin, derivation_path: &DerivationPath) -> Result<PrivateKey, Error> {
        let curve = Curve::from_str(&coin.curve)?;
        let private_key_type = PrivateKey::get_private_key_type(&curve);
        let node = bip32::get_node(&self.seed, &derivation_path.to_string(), curve)?;
        match private_key_type {
            PrivateKeyType::PrivateKeyTypeDefault32 => {
                Ok(PrivateKey::new(&node.private_key_bytes)?)
            },
            PrivateKeyType::PrivateKeyTypeExtended96 |
            PrivateKeyType::PrivateKeyTypeHd => {
                Err(Error::CryptoError(CryptoError::InvalidPrivateKey))
            },
        }
    }

    pub fn get_address_for_coin(&self, coin: &Coin) -> Result<String, Error> {
        let derivation_path = DerivationPath::new(&coin.derivation_path)?;
        let priv_key = self.get_key(&coin, &derivation_path)?;
        let public_key = priv_key.get_public_key(&coin.public_key_type)?;
        Ok(get_dispatcher(&coin).derive_address(&coin, &public_key, &[], &[])?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_mnemonic_is_valid() {
        let mnemonic = "team engine square letter hero song dizzy scrub tornado fabric divert saddle";
        let invalid_mnemonic = "team engine square letter hero song dizzy scrub tornado fabric divert";
        assert_eq!(HdWallet::is_valid(&mnemonic), true);
        assert_eq!(HdWallet::is_valid(&invalid_mnemonic), false);
    }
    #[test]
    fn test_create_new_hd_wallet() {
        let wallet = HdWallet::new(128, "").unwrap();
    }

}