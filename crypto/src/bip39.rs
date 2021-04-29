use crate::Error;
use bip39::Mnemonic as CryptoMnemonic;
use std::str::FromStr;

pub struct Mnemonic {
    pub words: String,
    pub seed: Vec<u8>,
    pub entropy: Vec<u8>,
}

impl Mnemonic {
    pub fn generate(word_count: u32, passphrase: &str) -> Result<Mnemonic, Error> {
        let mnemonic =
            CryptoMnemonic::generate(word_count as usize).map_err(|_| Error::InvalidMnemonic)?;
        let seed = mnemonic.to_seed_normalized(passphrase).to_vec();
        let (arr, len) = mnemonic.to_entropy_array();
        let entropy = arr[0..len].to_vec();
        Ok(Mnemonic {
            words: mnemonic.to_string(),
            seed,
            entropy,
        })
    }

    pub fn new(mnemonic: &str, passphrase: &str) -> Result<Mnemonic, Error> {
        let mnemonic = CryptoMnemonic::from_str(mnemonic).map_err(|_| Error::InvalidMnemonic)?;
        let seed = mnemonic.to_seed_normalized(passphrase).to_vec();
        let (arr, len) = mnemonic.to_entropy_array();
        let entropy = arr[0..len].to_vec();
        Ok(Mnemonic {
            words: mnemonic.to_string(),
            seed,
            entropy,
        })
    }

    pub fn is_valid(mnemonic: &str) -> bool {
        CryptoMnemonic::parse_normalized(mnemonic).is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_generate_new_mnenomic() {
        let mnemonic = Mnemonic::generate(12, "").unwrap();
        assert_eq!(Mnemonic::is_valid(&mnemonic.words), true);

        let mnemonic = Mnemonic::generate(18, "").unwrap();
        assert_eq!(Mnemonic::is_valid(&mnemonic.words), true);

        let mnemonic = Mnemonic::generate(24, "").unwrap();
        assert_eq!(Mnemonic::is_valid(&mnemonic.words), true);

        assert_eq!(Mnemonic::generate(25, "").is_err(), true);
        assert_eq!(Mnemonic::generate(11, "").is_err(), true);
    }
}
