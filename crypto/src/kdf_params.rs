use crate::Error;

pub trait KdfParamsType {
    fn generate_derived_key(&self, password: &[u8]) -> Result<Vec<u8>, Error>;
}

pub enum KdfParams {
    ScryptParam(Box<dyn KdfParamsType>),
}

impl KdfParamsType for KdfParams {
    fn generate_derived_key(&self, password: &[u8]) -> Result<Vec<u8>, Error> {
        match self {
            Self::ScryptParam(algo) => algo.generate_derived_key(password)
        }
    }
}