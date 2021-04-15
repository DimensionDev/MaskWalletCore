use scrypt::{ scrypt, Params };

use super::kdf_params::KdfParamsType;

const CREDENTIAL_LEN: usize = 64usize;

pub struct ScryptParameters {
    n: u32,
    p: u32,
    r: u32,
    dklen: usize,
    salt: String,
}

impl Default for ScryptParameters {
    fn default() -> ScryptParameters {
        ScryptParameters {
            n: 262144,
            p: 1,
            r: 8,
            dklen: 32,
            salt: "".to_string(),
        }
    }
}

impl KdfParamsType for ScryptParameters {
    fn generate_derived_key(&self, password: &[u8]) -> Vec<u8> {
        let log_n = (self.n as f64).log2().round();
        let params = Params::new(log_n as u8, self.r, self.p).expect("invalid scrypt params");

        let mut output: [u8; CREDENTIAL_LEN] = [0; CREDENTIAL_LEN];
        scrypt(password, self.salt.as_bytes(), &params, &mut output).expect("can not execute scrypt");
        return output[0..self.dklen].to_vec();
    }
}