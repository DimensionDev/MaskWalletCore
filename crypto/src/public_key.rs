use secp256k1::PublicKey;
use std::str::FromStr;
use crate::Error;

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

pub fn get_public_key_data(private_key_data: &[u8]) -> Result<Vec<u8>, Error> {
    PublicKey::from_slice(&private_key_data).map(|pk| pk.serialize().to_vec() ).map_err(|_| Error::InvalidPrivateKey )
}