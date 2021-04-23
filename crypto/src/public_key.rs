use secp256k1::{ Secp256k1, SecretKey, PublicKey};
use std::str::FromStr;
use crate::Error;

#[derive(PartialEq)]
pub enum PublicKeyType {
    Secp256k1,
    Secp256k1Extended
}

impl FromStr for PublicKeyType {
    type Err = ();

    fn from_str(input: &str) -> Result<PublicKeyType, Self::Err> {
        match input.to_lowercase().as_str() {
            "secp256k1" => Ok(Self::Secp256k1),
            "secp256k1extended" => Ok(Self::Secp256k1Extended),
            _  => Err(()),
        }
    }
}

struct Secp256k1Converter;

impl PublicKeyConvert for Secp256k1Converter {
    fn convert(&self, private_key: &[u8]) -> Result<Vec<u8>, Error> {
        let secrect_key = SecretKey::from_slice(private_key).map_err(|_| Error::InvalidPrivateKey)?;
        let pub_key = PublicKey::from_secret_key(&Secp256k1::new(), &secrect_key);
        Ok(pub_key.serialize().to_vec())
    }
}

struct Secp256k1ExtendConverter;

impl PublicKeyConvert for Secp256k1ExtendConverter {
    fn convert(&self, private_key: &[u8]) -> Result<Vec<u8>, Error> {
        let secrect_key = SecretKey::from_slice(private_key).map_err(|_| Error::InvalidPrivateKey)?;
        let pub_key = PublicKey::from_secret_key(&Secp256k1::new(), &secrect_key);
        Ok(pub_key.serialize_uncompressed().to_vec())
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

pub fn get_public_key(pub_key_type: &str, private_key: &[u8], _extend_bytes: &[u8], _chain_code_bytes: &[u8]) -> Result<Vec<u8>, Error> {
    let public_key_type = PublicKeyType::from_str(pub_key_type).map_err(|_| Error::NotSupportedPublicKeyType)?;

        match public_key_type {
            PublicKeyType::Secp256k1 => {
                PublickKeyConvertter::convert(Secp256k1Converter, private_key)
            },
            PublicKeyType::Secp256k1Extended => {
                PublickKeyConvertter::convert(Secp256k1ExtendConverter, private_key)
            },
        }
}