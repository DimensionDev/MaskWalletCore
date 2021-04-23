use hex::ToHex;
use serde::{ Serialize, Deserialize };
use super::number_util::random_iv;

#[derive(Serialize, Deserialize, Clone)]
pub struct AesParams {
    pub iv: String,
}

impl Default for AesParams {
    fn default() -> Self {
        let random = random_iv(16);
        let iv_hex = random.encode_hex::<String>();
        AesParams {
            iv: iv_hex
        }
    }
}