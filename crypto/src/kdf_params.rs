pub trait KdfParams: Default {
    fn generate_derived_key(&self, password: &[u8]) -> Vec<u8>;
}