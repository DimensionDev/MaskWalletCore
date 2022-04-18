use scrypt::{scrypt, Params};
use serde::{Deserialize, Serialize};

use super::kdf_params::KdfParamsType;
use super::number_util::random_iv;
use crate::Error;

const CREDENTIAL_LEN: usize = 64usize;

#[derive(Serialize, Deserialize, Clone)]
pub struct ScryptParams {
    pub n: u32,
    pub p: u32,
    pub r: u32,
    pub dklen: usize,
    pub salt: String,
}

impl Default for ScryptParams {
    fn default() -> ScryptParams {
        ScryptParams {
            n: 4096,
            p: 1,
            r: 8,
            dklen: 32,
            salt: hex::encode(random_iv(32)),
        }
    }
}

impl KdfParamsType for ScryptParams {
    fn generate_derived_key(&self, password: &[u8]) -> Result<Vec<u8>, Error> {
        let log_n = (self.n as f64).log2().round();
        let params = Params::new(log_n as u8, self.r, self.p).or(Err(Error::KdfParamsInvalid))?;

        let mut output: [u8; CREDENTIAL_LEN] = [0; CREDENTIAL_LEN];
        let salt_bytes = hex::decode(&self.salt).or(Err(Error::KdfParamsInvalid))?;
        scrypt(password, &salt_bytes, &params, &mut output).or(Err(Error::PasswordIncorrect))?;
        Ok(output[0..self.dklen].to_vec())
    }
}

#[cfg(test)]
mod tests {
    use scrypt::{scrypt, Params};
    #[test]
    fn test_scrypt() {
        let log_n = 4096_f64.log2().round();
        let params = Params::new(log_n as u8, 8, 1).unwrap();
        let mut output: [u8; 64] = [0; 64];
        let password = "mask";
        scrypt(password.as_bytes(), "".as_bytes(), &params, &mut output).unwrap();
    }
}
