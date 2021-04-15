pub trait KdfParamsType {
    fn generate_derived_key(&self, password: &[u8]) -> Vec<u8>;
}

pub enum KdfParams {
    ScryptParam(Box<dyn KdfParamsType>),
}

impl KdfParamsType for KdfParams {
    fn generate_derived_key(&self, password: &[u8]) -> Vec<u8> {
        match self {
            Self::ScryptParam(algo) => algo.generate_derived_key(password)
        }
    }
}