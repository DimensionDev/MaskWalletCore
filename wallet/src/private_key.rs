use std::str::FromStr;
use crate::Error;
use crypto::curve::Curve;
use crypto::Error as CryptoError;
use crypto::public_key::PublicKeyType;
use super::public_key::PublicKey;

// The number of bytes in a private key.
const VALID_SIZE: u8 = 32;
// The number of bytes in an extended private key.
const VALID_EXTENDED_SIZE: u8 = 3 * VALID_SIZE;

pub struct PrivateKey {
    data: Vec<u8>,
    extends_data: Vec<u8>,
    chain_code_bytes: Vec<u8>,
}

impl PrivateKey {
    fn is_valid_data(data: &[u8]) -> bool {
        // Check length.  Extended key needs 3*32 bytes.
        if data.len() as u8 != VALID_SIZE && data.len() as u8 != VALID_EXTENDED_SIZE {
            return false
        }
        // Check whether data is not all zero
        return data.iter().any(|&x| x != 0);
    }

    pub fn is_valid(data: &[u8], curve: &str) -> Result<(), Error> {
        if !Self::is_valid_data(data) {
            return Err(Error::CryptoError(CryptoError::InvalidPrivateKey));
        }
        return Curve::from_str(curve).map_err(|_| Error::NotSupportedCurve).map(|_| {});
    }

    fn new_extended(data: &[u8], ext: &[u8], chain_code: &[u8]) -> Result<PrivateKey, Error> {
        if !Self::is_valid_data(data) || !Self::is_valid_data(ext) || !Self::is_valid_data(chain_code) {
            return Err(Error::CryptoError(CryptoError::InvalidPrivateKey));
        }
        Ok(PrivateKey {
            data: data.to_vec(),
            extends_data: ext.to_vec(),
            chain_code_bytes: chain_code.to_vec(),
        })
    }

    pub fn new(data: &[u8]) -> Result<PrivateKey, Error> {
        if !Self::is_valid_data(data) {
            return Err(Error::CryptoError(CryptoError::InvalidPrivateKey));
        }
        if data.len() == VALID_EXTENDED_SIZE as usize {
            Self::new_extended(&data[0..32], &data[32..64], &data[64..96])
        } else {
            Ok(PrivateKey {
                data: data.to_vec(),
                extends_data: vec![],
                chain_code_bytes: vec![],
            })
        }
    }

    pub fn get_public_key(&self, public_key_type_str: &str) -> Result<PublicKey, Error> {
        let public_key_type = PublicKeyType::from_str(public_key_type_str).map_err(|_| Error::NotSupportedPublicKeyType)?;

        match public_key_type {
            PublicKeyType::SECP256k1Extended => Err(Error::NotSupportedPublicKeyType),
            _ => Err(Error::NotSupportedPublicKeyType)
        }
    }
}