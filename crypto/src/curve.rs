use crate::Error;
use std::str::FromStr;

use std::convert::TryFrom;

#[derive(Clone, Debug)]
pub enum Curve {
    Secp256k1,
    Ed25519,
    Ed25519hd,
    Ed25519Blake2bNano,
    Curve25519,
    Nist256p1,
    Ed25519Extended,
}

impl Default for Curve {
    fn default() -> Self {
        Curve::Secp256k1
    }
}

impl FromStr for Curve {
    type Err = Error;

    fn from_str(input: &str) -> Result<Curve, Error> {
        match input.to_lowercase().as_str() {
            "secp256k1" => Ok(Self::Secp256k1),
            "ed25519" => Ok(Self::Ed25519),
            "ed25519-hd" => Ok(Self::Ed25519hd),
            "ed25519-blake2b-nano" => Ok(Self::Ed25519Blake2bNano),
            "curve25519" => Ok(Self::Curve25519),
            "nist256p1" => Ok(Self::Nist256p1),
            "ed25519-cardano-seed" => Ok(Self::Ed25519Extended),
            _ => Err(Error::NotSupportedCurve),
        }
    }
}

impl TryFrom<&str> for Curve {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Error> {
        Self::from_str(s)
    }
}
