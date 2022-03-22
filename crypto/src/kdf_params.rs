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

#[allow(dead_code)]
pub fn generate_derived_key_from<T, U>(adapter: T, password: &[u8]) -> Result<Vec<u8>, Error>
where
    U: From<T>,
    U: KdfParamsType,
{
    let excutor = U::from(adapter);
    excutor.generate_derived_key(password)
}

#[allow(dead_code)]
pub fn generate_derived_key_with<T: AsRef<U>, U>(
    adapter: T,
    password: &[u8],
) -> Result<Vec<u8>, Error>
where
    U: KdfParamsType,
{
    adapter.as_ref().generate_derived_key(password)
}
