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

pub mod jwk;
pub mod pbkdf2;

#[derive(Debug, PartialOrd, PartialEq)]
pub enum Error {
    KdfParamsInvalid,

    PasswordIncorrect,

    InvalidKeyIvLength,

    InvalidCiphertext,

    InvalidPrivateKey,

    InvalidPublicKey,

    InvalidMnemonic,

    InvalidSeed,

    InvalidDerivationpath,

    InvalidKeyStoreJson,

    NotSupportedPublicKeyType,

    NotSupportedCurve,

    NotSupportedCipher,
}

impl Error {
    pub fn get_code(&self) -> String {
        match self {
            Error::KdfParamsInvalid => "-3001".to_owned(),
            Error::PasswordIncorrect => "-3002".to_owned(),
            Error::InvalidKeyIvLength => "-3003".to_owned(),
            Error::InvalidCiphertext => "-3004".to_owned(),
            Error::InvalidPrivateKey => "-3005".to_owned(),
            Error::InvalidPublicKey => "-3006".to_owned(),
            Error::InvalidMnemonic => "-3007".to_owned(),
            Error::InvalidSeed => "-3008".to_owned(),
            Error::InvalidDerivationpath => "-3009".to_owned(),
            Error::InvalidKeyStoreJson => "-3010".to_owned(),
            Error::NotSupportedPublicKeyType => "-3011".to_owned(),
            Error::NotSupportedCurve => "-3012".to_owned(),
            Error::NotSupportedCipher => "-3013".to_owned(),
        }
    }

    pub fn get_message(&self) -> String {
        match self {
            Error::KdfParamsInvalid => "Invalid kdf parameter".to_owned(),
            Error::PasswordIncorrect => "Password incorrect".to_owned(),
            Error::InvalidKeyIvLength => "Invalid iv length".to_owned(),
            Error::InvalidCiphertext => "Invalid cipher text".to_owned(),
            Error::InvalidPrivateKey => "Invalid private key".to_owned(),
            Error::InvalidPublicKey => "Invalid public key".to_owned(),
            Error::InvalidMnemonic => "Invalid mnemonic".to_owned(),
            Error::InvalidSeed => "Invalid seed".to_owned(),
            Error::InvalidDerivationpath => "Invalid derivation path".to_owned(),
            Error::InvalidKeyStoreJson => "Invalid key store json".to_owned(),
            Error::NotSupportedPublicKeyType => "Not supported public key type".to_owned(),
            Error::NotSupportedCurve => "Not supported curve".to_owned(),
            Error::NotSupportedCipher => "Not supported cipher type".to_owned(),
        }
    }
}

impl From<bip39::BIP39Error> for Error {
    fn from(_err: bip39::BIP39Error) -> Error {
        Error::InvalidMnemonic
    }
}
