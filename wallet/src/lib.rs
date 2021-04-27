pub mod stored_key;
pub mod account;
pub mod hd_wallet;
pub mod encryption_params;
mod derivation_path;
mod coin_dispatcher;

use std::fmt::Debug;
use crypto::Error as CryptoError;
use chain_common::Error as ChainError;

#[derive(Debug, PartialEq)]
pub enum Error {
    CryptoError(CryptoError),
    ChainError(ChainError),
    IndexOutOfBounds,
    InvalidAccountRequested,
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