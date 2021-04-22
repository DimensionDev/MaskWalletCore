pub mod stored_key;
pub mod account;
pub mod hd_wallet;
pub mod encryption_params;
mod derivation_path;
mod coin_dispatcher;

use std::fmt::Debug;
use crypto::Error as CryptoError;

#[derive(Debug, PartialEq)]
pub enum Error {
    CryptoError(CryptoError),
    IndexOutOfBounds,
    InvalidAccountRequested,
}

impl From<CryptoError> for Error {
    fn from(err: CryptoError) -> Error {
        Error::CryptoError(err)
    }
}