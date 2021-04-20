use std::str::FromStr;
use crypto::bip39;
use chain_common::coin::Coin;
use chain_common::private_key::{ PrivateKey, PrivateKeyType };
use crypto::curve::Curve;
use crypto::Error as CryptoError;
use crate::Error;
use super::derivation_path::DerivationPath;

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
    fn get_key(coin: &Coin, derivation_path: &DerivationPath) -> Result<PrivateKey, Error> {
        let curve = Curve::from_str(&coin.curve)?;
        let private_key_type = PrivateKey::get_private_key_type(&curve);
        match private_key_type {
            PrivateKeyType::PrivateKeyTypeDefault32 => {
                let priv_key = PrivateKey::new(&[])?;
                Ok(priv_key)
            },
            PrivateKeyType::PrivateKeyTypeExtended96 => {
                Err(Error::CryptoError(CryptoError::InvalidPrivateKey))
            },
            PrivateKeyType::PrivateKeyTypeHD => {
                Err(Error::CryptoError(CryptoError::InvalidPrivateKey))
            }
        }
    }

    pub fn get_address_for_coin(&self, coin: &Coin) -> Result<String, Error> {
        let derivation_path = DerivationPath::new(&coin.derivation_path)?;
        let priv_key = Self::get_key(&coin, &derivation_path)?;
        Ok("".to_owned())
    }
}