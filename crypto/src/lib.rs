pub mod aes;
pub mod aes_params;
pub mod curve;
pub mod hash;
pub mod kdf_params;
pub mod key_store_json;
pub mod public_key;
pub mod scrypt_params;

pub mod bip32;
pub mod bip39;

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

    InvalidDerivationpath,

    InvalidKeyStoreJSON,

    NotSupportedPublicKeyType,

    NotSupportedCurve,

    NotSupportedCipher,

    CachedDkFeatureNotSupport,
}
