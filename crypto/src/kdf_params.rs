use super::scrypt_params::ScryptParams;
use crate::Error;
use serde::{Deserialize, Serialize};

pub trait KdfParamsType {
    fn generate_derived_key(&self, password: &[u8]) -> Result<Vec<u8>, Error>;
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum KdfParams {
    ScryptParam(ScryptParams),
}

impl KdfParamsType for KdfParams {
    fn generate_derived_key(&self, password: &[u8]) -> Result<Vec<u8>, Error> {
        match self {
            Self::ScryptParam(algo) => algo.generate_derived_key(password),
        }
    }
}
