use serde::{ Serialize, Deserialize };
use super::scrypt_params::ScryptParams;
use crate::Error;

pub trait KdfParamsType {
    fn generate_derived_key(&self, password: &[u8]) -> Result<Vec<u8>, Error>;
}

#[derive(Serialize, Deserialize)]
pub enum KdfParams {
    ScryptParam(ScryptParams),
}

impl KdfParamsType for KdfParams {
    fn generate_derived_key(&self, password: &[u8]) -> Result<Vec<u8>, Error> {
        match self {
            Self::ScryptParam(algo) => algo.generate_derived_key(password)
        }
    }
}