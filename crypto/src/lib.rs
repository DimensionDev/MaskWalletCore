pub mod kdf_params;
pub mod scrypt_params;
pub mod aes_params;
pub mod aes;
pub mod hash;
pub mod curve;
pub mod public_key;

pub mod bip39;
pub mod bip32;

pub mod number_util;

#[derive(Debug, PartialOrd, PartialEq)]
pub enum Error {
    
    KdfParamsInvalid,
    
    PasswordIncorrect,
    
    DerivedKeyNotMatched,
    
    InvalidKeyIvLength,
    
    InvalidCiphertext,

    InvalidPrivateKey,

    InvalidPublicKey,

    InvalidMnemonic,

    InvalidSeed,

    NotSupportedPublicKeyType,

    NotSupportedCurve,
    
    CachedDkFeatureNotSupport,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
