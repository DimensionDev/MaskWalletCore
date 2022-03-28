use std::str::FromStr;

use bip39::Language;
pub use bip39::{Error as BIP39Error, Mnemonic as CryptoMnemonic};

use super::number_util::random_iv;
use crate::Error;

const SUPPORT_MNEMONIC_WORDS_COUNT: [u32; 3] = [12, 18, 24];

pub struct Mnemonic {
    pub words: String,
    pub seed: Vec<u8>,
    pub entropy: Vec<u8>,
}

impl Mnemonic {
    pub fn generate_mnemonic_string(word_count: u32) -> Result<String, Error> {
        if !SUPPORT_MNEMONIC_WORDS_COUNT.contains(&word_count) {
            return Err(Error::InvalidMnemonic);
        }
        let entropy_bytes = (word_count / 3) * 4;
        let entropy = random_iv(entropy_bytes as usize);
        let mnemonic = CryptoMnemonic::from_entropy_in(Language::English, &entropy)?;
        Ok(mnemonic.to_string())
    }

    pub fn generate(word_count: u32, password: &str) -> Result<Mnemonic, Error> {
        if !SUPPORT_MNEMONIC_WORDS_COUNT.contains(&word_count) {
            return Err(Error::InvalidMnemonic);
        }
        let entropy_bytes = (word_count / 3) * 4;
        let entropy = random_iv(entropy_bytes as usize);
        let mnemonic = CryptoMnemonic::from_entropy_in(Language::English, &entropy)?;

        let seed = mnemonic.to_seed_normalized(password).to_vec();
        let (arr, len) = mnemonic.to_entropy_array();
        let entropy = arr[0..len].to_vec();
        Ok(Mnemonic {
            words: mnemonic.to_string(),
            seed,
            entropy,
        })
    }

    pub fn new(mnemonic: &str, password: &str) -> Result<Mnemonic, Error> {
        let mnemonic = CryptoMnemonic::from_str(&mnemonic.to_lowercase())?;
        let seed = mnemonic.to_seed_normalized(password).to_vec();
        let (arr, len) = mnemonic.to_entropy_array();
        let entropy = arr[0..len].to_vec();
        Ok(Mnemonic {
            words: mnemonic.to_string(),
            seed,
            entropy,
        })
    }

    pub fn is_valid(mnemonic: &str) -> bool {
        CryptoMnemonic::parse_normalized(&mnemonic.to_lowercase()).is_ok()
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

    #[test]
    fn test_init_with_mneomnic_string() {
        let test_mnemonic_eng =
            "Depth bachelor empower life rapid album medal royal gorilla grace impact build";
        let test_mnemonic_jpn = "ぐこう　いわい　けんすう　そまつ　にっさん　あわてる　たたかう　のこぎり　じてん　しねん　ずほう　えんしゅう";
        let test_mnemonic_chn = "育 内 充 敢 炭 说 旗 伦 茶 硫 亮 农";
        let is_valid_eng = Mnemonic::is_valid(&test_mnemonic_eng);
        let is_valid_jpn = Mnemonic::is_valid(&test_mnemonic_jpn);
        let is_valid_chn = Mnemonic::is_valid(&test_mnemonic_chn);
        assert_eq!(is_valid_eng, true);
        assert_eq!(is_valid_jpn, true);
        assert_eq!(is_valid_chn, true);

        let mnemonic_eng = Mnemonic::new(&test_mnemonic_eng, "password1").unwrap();
        assert_eq!(mnemonic_eng.words, test_mnemonic_eng.to_lowercase());
    }
}
