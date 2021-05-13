pub mod account;
pub mod coin_dispatcher;
mod derivation_path;
pub mod encryption_params;
pub mod hd_wallet;
pub mod stored_key;

use chain_common::Error as ChainError;
use crypto::Error as CryptoError;
use std::fmt::Debug;

#[derive(Debug, PartialEq)]
pub enum Error {
    CryptoError(CryptoError),
    ChainError(ChainError),
    IndexOutOfBounds,
    RequestNotSupportedOnPrivateKeyTypeStoredKey,
    RequstedAccountNotFound,
    AccountAlreadyExist,
    JsonSerializationError,
}

impl From<CryptoError> for Error {
    fn from(err: CryptoError) -> Error {
        Error::CryptoError(err)
    }
}

impl From<ChainError> for Error {
    fn from(err: ChainError) -> Error {
        Error::ChainError(err)
    }
}

impl Error {
    pub fn get_code(&self) -> String {
        match self {
            Error::JsonSerializationError => "-1001".to_owned(),
            Error::IndexOutOfBounds => "-1002".to_owned(),
            Error::RequestNotSupportedOnPrivateKeyTypeStoredKey => "-1003".to_owned(),
            Error::RequstedAccountNotFound => "-1004".to_owned(),
            Error::AccountAlreadyExist => "-1005".to_owned(),
            Error::CryptoError(crypto_error) => crypto_error.get_code(),
            Error::ChainError(chain_error) => chain_error.get_code(),
        }
    }

    pub fn get_message(&self) -> String {
        match self {
            Error::JsonSerializationError => "Fail to serialize json".to_owned(),
            Error::IndexOutOfBounds => "Requested index is out of bounds".to_owned(),
            Error::RequestNotSupportedOnPrivateKeyTypeStoredKey => {
                "Request is not supported on a private key type StoredKey".to_owned()
            }
            Error::RequstedAccountNotFound => {
                "Account of requested address is not found".to_owned()
            }
            Error::AccountAlreadyExist => "The requested account already exists".to_owned(),
            Error::CryptoError(crypto_error) => crypto_error.get_message(),
            Error::ChainError(chain_error) => chain_error.get_message(),
        }
    }
}
