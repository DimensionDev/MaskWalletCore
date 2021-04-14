use hex::ToHex;
use super::number_util::random_iv;

pub struct AESParams {
    pub iv: String,
}

impl Default for AESParams {
    fn default() -> Self {
        let random = random_iv(16);
        let iv_hex = random.encode_hex::<String>();
        AESParams {
            iv: iv_hex
        }
    }
}