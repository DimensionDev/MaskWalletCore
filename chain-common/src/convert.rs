use std::convert::{From, TryFrom};
use std::str::FromStr;

use crate::generated::api::{
    mw_response::Response, persona_generation_param::Curve, MwResponse, MwResponseError,
};
use crypto::Error as CryptoError;

impl From<CryptoError> for MwResponseError {
    fn from(err: CryptoError) -> Self {
        Self {
            error_code: err.get_code(),
            error_msg: err.get_message(),
        }
    }
}

impl From<crypto::jwk::BIP32Error> for MwResponseError {
    fn from(err: crypto::jwk::BIP32Error) -> Self {
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
            "secp256k1" | "Secp256k1" | "0" => Ok(Curve::Secp256k1),
            "Ed25519" | "ed25519" | "1" => Ok(Curve::Ed25519),
            _ => Err(MwResponseError {
                error_code: "-1".to_owned(),
                error_msg: format!("unsupport curve type: {:}", s),
            }),
        }
    }
}

impl TryFrom<i32> for Curve {
    type Error = MwResponseError;

    fn try_from(value: i32) -> Result<Curve, MwResponseError> {
        match value {
            0 => Ok(Curve::Secp256k1),
            1 => Ok(Curve::Ed25519),
            _ => Err(MwResponseError {
                error_code: "-1".to_owned(),
                error_msg: format!("unsupport curve type: {:}", value),
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

impl TryFrom<Option<i32>> for Curve {
    type Error = MwResponseError;

    fn try_from(s: Option<i32>) -> Result<Self, Self::Error> {
        match s {
            Some(s) => Curve::try_from(s),
            None => Err(MwResponseError {
                error_code: "-1".to_owned(),
                error_msg: format!("empty curve type"),
            }),
        }
    }
}
