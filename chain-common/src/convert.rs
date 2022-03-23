use std::convert::{From, TryFrom};
use std::str::FromStr;

use crate::generated::api::{curve::Curve, mw_response::Response, MwResponse, MwResponseError};
use crypto::Error as CryptoError;

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

impl From<Result<Response, MwResponseError>> for MwResponse {
    fn from(result: Result<Response, MwResponseError>) -> Self {
        match result {
            Ok(resp) => MwResponse {
                response: Some(resp),
            },
            Err(error) => MwResponse {
                response: Some(Response::Error(error)),
            },
        }
    }
}

impl FromStr for Curve {
    type Err = MwResponseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "secp256k1" | "Secp256k1" => Ok(Curve::Secp256k1(s.to_owned())),
            "Ed25519" | "ed25519" => Ok(Curve::Ed25519(s.to_owned())),
            _ => Err(MwResponseError {
                error_code: "-1".to_owned(),
                error_msg: format!("unsupport curve type: {:}", s),
            }),
        }
    }
}

impl TryFrom<&str> for Curve {
    type Error = MwResponseError;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Curve::from_str(s)
    }
}
