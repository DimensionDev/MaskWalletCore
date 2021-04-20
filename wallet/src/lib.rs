pub mod stored_key;
pub mod account;
mod derivation_path;
pub mod encryption_params;
mod coin_dispatcher;

use std::fmt::Debug;
use crypto::Error as CryptoError;

#[derive(Debug, PartialEq)]
pub enum Error {
    CryptoError(CryptoError),
    InvalidDerivationpath,
    IndexOutOfBounds,
}

impl From<CryptoError> for Error {
    fn from(err: CryptoError) -> Error {
        Error::CryptoError(err)
    }
}