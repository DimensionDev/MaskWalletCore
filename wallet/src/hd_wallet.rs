use std::str::FromStr;
use crypto::curve::Curve;
use crypto::Error as CryptoError;
use crypto::bip39::Mnemonic;
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
    pub fn new(word_count: u32, passphrase: &str) -> Result<HdWallet, Error> {
        let mnemonic = Mnemonic::generate(word_count, passphrase)?;
        Ok(HdWallet {
            seed: mnemonic.seed,
            mnemonic: mnemonic.words,
            passphrase: passphrase.to_owned(),
            entropy: mnemonic.entropy
        })
    }

    pub fn new_with_mnemonic(mnemonic: &str, passphrase: &str) -> Result<HdWallet, Error> {
        let mnemonic = Mnemonic::new(&mnemonic, &passphrase)?;
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
    use chain_common::coin::Coin;
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
        let word_count = 12;
        let wallet = HdWallet::new(word_count, "").unwrap();
        assert_eq!(wallet.mnemonic.split(' ').collect::<Vec<&str>>().len(), word_count as usize);
        assert_eq!(HdWallet::is_valid(&wallet.mnemonic), true);

        let word_count = 18;
        let wallet = HdWallet::new(word_count, "").unwrap();
        assert_eq!(wallet.mnemonic.split(' ').collect::<Vec<&str>>().len(), word_count as usize);
        assert_eq!(HdWallet::is_valid(&wallet.mnemonic), true);

        let word_count = 24;
        let wallet = HdWallet::new(word_count, "").unwrap();
        assert_eq!(wallet.mnemonic.split(' ').collect::<Vec<&str>>().len(), word_count as usize);
        assert_eq!(HdWallet::is_valid(&wallet.mnemonic), true);
    }

    #[test]
    fn test_get_address_for_coin() {
        let wallet = HdWallet::new(12, "").unwrap();
        let derivation_path = "m/44'/60'/0'/0/0";
        let coin = Coin {
            id: "60".to_owned(),
            name: "ethereum".to_owned(),
            coin_id: 60,
            symbol: "ETH".to_owned(),
            decimals: 18,
            blockchain: "Ethereum".to_owned(),
            derivation_path: derivation_path.to_owned(),
            curve: "secp256k1".to_owned(),
            public_key_type: "secp256k1Extended".to_owned(),
        };
        let address1 = wallet.get_address_for_coin(&coin).unwrap();
        let address2 = wallet.get_address_for_coin(&coin).unwrap();
        assert_eq!(address1, address2);
    }

}