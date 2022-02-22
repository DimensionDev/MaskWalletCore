use super::Error as CryptoError;
use rsa::RsaPrivateKey;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::str::FromStr;

use base64;
use biscuit::jwa::{Algorithm, SignatureAlgorithm};
use biscuit::jwk::{AlgorithmParameters, CommonParameters, RSAKeyParameters, JWK};
use biscuit::Empty;
use rsa::PublicKeyParts;

#[derive(Serialize, Deserialize)]
struct RSAMid {
    kty: String,
    n: String,
    e: String,
    d: String,
    p: String,
    q: String,
}

pub fn new_jwk(rsa_priv_key: &RsaPrivateKey) -> Result<Vec<u8>, CryptoError> {
    let rsa_mid = RSAMid {
        kty: "RSA".to_owned(),
        n: base64::encode_config(rsa_priv_key.n().to_bytes_be(), base64::URL_SAFE_NO_PAD),
        e: base64::encode_config(rsa_priv_key.e().to_bytes_be(), base64::URL_SAFE_NO_PAD),
        d: base64::encode_config(rsa_priv_key.d().to_bytes_be(), base64::URL_SAFE_NO_PAD),
        p: base64::encode_config(
            rsa_priv_key.primes()[0].to_bytes_be(),
            base64::URL_SAFE_NO_PAD,
        ),
        q: base64::encode_config(
            rsa_priv_key.primes()[1].to_bytes_be(),
            base64::URL_SAFE_NO_PAD,
        ),
    };
    let rsa_mid_str =
        serde_json::to_string(&rsa_mid).map_err(|_| CryptoError::InvalidKeyIvLength)?;

    let rsa_param: RSAKeyParameters =
        serde_json::from_str(&rsa_mid_str).map_err(|_| CryptoError::InvalidKeyIvLength)?;

    let jwk: JWK<Empty> = JWK {
        common: CommonParameters {
            algorithm: Some(Algorithm::Signature(SignatureAlgorithm::RS256)),
            ..Default::default()
        },
        algorithm: AlgorithmParameters::RSA(rsa_param),
        additional: Default::default(),
    };
    let jwk_bytes = serde_json::to_vec(&jwk).map_err(|_| CryptoError::InvalidPrivateKey)?;
    Ok(jwk_bytes)
}

pub fn jwk_from_bytes(jwk_bytes: &[u8]) -> Result<Vec<u8>, CryptoError> {
    let jwk: JWK<Empty> =
        serde_json::from_slice(&jwk_bytes).map_err(|_| CryptoError::InvalidPrivateKey)?;
    let rsa_param = match jwk.algorithm {
        AlgorithmParameters::RSA(rsa_param) => rsa_param,
        _ => return Err(CryptoError::InvalidPrivateKey),
    };
    Ok(Sha256::digest(&rsa_param.n.to_bytes_be()).to_vec())
}
