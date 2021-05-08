pub mod account;
mod coin_dispatcher;
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
