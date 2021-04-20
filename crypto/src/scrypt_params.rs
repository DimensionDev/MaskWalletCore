use serde::{ Serialize, Deserialize };
use scrypt::{ scrypt, Params };

use super::kdf_params::KdfParamsType;
use crate::Error;

const CREDENTIAL_LEN: usize = 64usize;

#[derive(Serialize, Deserialize)]
pub struct ScryptParams {
    n: u32,
    p: u32,
    r: u32,
    dklen: usize,
    salt: String,
}

impl Default for ScryptParams {
    fn default() -> ScryptParams {
        ScryptParams {
            n: 4096,
            p: 1,
            r: 8,
            dklen: 32,
            salt: "".to_string(),
        }
    }
}

impl KdfParamsType for ScryptParams {
    fn generate_derived_key(&self, password: &[u8]) -> Result<Vec<u8>, Error> {
        let log_n = (self.n as f64).log2().round();
        let params = Params::new(log_n as u8, self.r, self.p).or(Err(Error::KdfParamsInvalid))?;

        let mut output: [u8; CREDENTIAL_LEN] = [0; CREDENTIAL_LEN];
        scrypt(password, self.salt.as_bytes(), &params, &mut output).or(Err(Error::PasswordIncorrect))?;
        Ok(output[0..self.dklen].to_vec())
    }
}

#[cfg(test)]
mod tests {
    use crate::scrypt_params::ScryptParams;
    use scrypt::{ scrypt, Params };
    #[test]
    fn it_works() {
        let log_n = (4096 as f64).log2().round();
        let params = Params::new(log_n as u8, 8, 1).unwrap();
        let mut output: [u8; 64] = [0; 64];
        let password = "mask";
        scrypt(password.as_bytes(), "".as_bytes(), &params, &mut output).unwrap();
    }
}