use bip39::Mnemonic as CryptoMnemonic;
use crate::Error;

pub struct Mnemonic {
    pub words: String,
    pub seed: Vec<u8>,
    pub entropy: Vec<u8>,
}

pub fn check_mnemonic(mnemonic: &str) -> bool {
    CryptoMnemonic::parse_normalized(mnemonic).is_ok()
}

pub fn generate(word_count: u32, passphrase: &str) -> Result<Mnemonic, Error> {
    let mnemonic = CryptoMnemonic::generate(word_count as usize).map_err(|_| Error::InvalidMnemonic)?;
    let seed = mnemonic.to_seed_normalized(passphrase).to_vec();
    let (arr, len) = mnemonic.to_entropy_array();
    let entropy = arr[0..len].to_vec();
    Ok(Mnemonic {
        words: mnemonic.to_string(),
        seed,
        entropy
    })
}

