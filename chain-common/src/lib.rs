use crypto::Error as CryptoError;
// mod generated;
use generated::api::{mw_response::Response, MwResponse, MwResponseError};

mod generated;
pub use generated::api;
pub use generated::ethereum;

pub mod coin;
pub mod entry;
pub mod private_key;
pub mod public_key;

#[derive(Debug, PartialEq)]
pub enum Error {
    NotSupportedCoin,
    InvalidSignInput,
    InvalidPrivateKey,
}

impl Error {
    pub fn get_code(&self) -> String {
        match self {
            Error::NotSupportedCoin => "-2001".to_owned(),
            Error::InvalidSignInput => "-2002".to_owned(),
            Error::InvalidPrivateKey => "-2003".to_owned(),
        }
    }

    pub fn get_message(&self) -> String {
        match self {
            Error::NotSupportedCoin => "Not supported coin".to_owned(),
            Error::InvalidSignInput => "Invalid sign input".to_owned(),
            Error::InvalidPrivateKey => "Invalid private key".to_owned(),
        }
    }
}

impl From<CryptoError> for MwResponseError {
    fn from(err: CryptoError) -> Self {
        Self {
            error_code: err.get_code(),
            error_msg: err.get_message(),
        }
    }
}

impl From<crypto::BIP32Error> for MwResponseError {
    fn from(err: crypto::BIP32Error) -> Self {
        Self {
            error_code: "-1".to_string(),
            error_msg: format!("{:?}", err),
        }
    }
}

impl From<MwResponseError> for MwResponse {
    fn from(err: MwResponseError) -> Self {
        Self {
            response: Some(Response::Error(err)),
        }
    }
}
