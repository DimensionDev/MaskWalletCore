use secp256k1::PublicKey;
use std::str::FromStr;
use crate::Error;

#[derive(PartialEq)]
pub enum PublicKeyType {
    SECP256k1Extended
}

impl FromStr for PublicKeyType {
    type Err = ();

    fn from_str(input: &str) -> Result<PublicKeyType, Self::Err> {
        match input.to_lowercase().as_str() {
            "secp256k1extended" => Ok(Self::SECP256k1Extended),
            _  => Err(()),
        }
    }
}

struct Secp256k1Extend;

impl PublicKeyConvert for Secp256k1Extend {
    fn convert(&self, private_key: &[u8]) -> Result<Vec<u8>, Error> {
        PublicKey::from_slice(&private_key).map(|pk| pk.serialize().to_vec() ).map_err(|_| Error::InvalidPrivateKey )
    }
}

trait PublicKeyConvert {
    fn convert(&self, private_key: &[u8]) -> Result<Vec<u8>, Error>;
}

struct PublickKeyConvertter;

impl PublickKeyConvertter {
    fn convert<T: PublicKeyConvert>(g: T, private_key: &[u8]) -> Result<Vec<u8>, Error> {
        g.convert(&private_key)
    }
}

pub fn get_public_key(pub_key_type: &str, private_key: &[u8], extend_bytes: &[u8], chain_code_bytes: &[u8]) -> Result<Vec<u8>, Error> {
    let public_key_type = PublicKeyType::from_str(pub_key_type).map_err(|_| Error::NotSupportedPublicKeyType)?;

        match public_key_type {
            PublicKeyType::SECP256k1Extended => {
                PublickKeyConvertter::convert(Secp256k1Extend, private_key)
            },
            _ => Err(Error::NotSupportedPublicKeyType)
        }
}