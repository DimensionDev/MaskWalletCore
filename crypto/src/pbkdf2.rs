use hmac::Hmac;
use pbkdf2;
use sha2::Sha256;

pub fn derive_key(password: &[u8], salt: &[u8], iterations: u32) -> Vec<u8> {
    let mut res = vec![0u8; 32];
    pbkdf2::pbkdf2::<Hmac<Sha256>>(password, salt, iterations, &mut res);
    res
}

#[cfg(test)]
mod tests {
    use crate::pbkdf2::derive_key;

    use base64::{encode_config, URL_SAFE_NO_PAD};

    #[test]
    fn test_pbkdf2() {
        let test_salt = "brother lemon plate blame sing donate wagon gospel level play brown cave";
        let password = concat!(
            "pPBO3vuCUjpEkPAc-74-CByna75Y2vvVsj_riqxljkk",
            "qtSrDPyn8eOEZ1-Zr5yWNBqhJZTNW5A43ypKx--CLrM"
        );

        let derived_key = derive_key(password.as_bytes(), test_salt.as_bytes(), 100_000);
        let base64_url_config = URL_SAFE_NO_PAD;
        let d = encode_config(&derived_key, base64_url_config);
        assert_eq!(d, "zG3NAz9demftsEMpiCBNr9TgEWvBFy3LV8R5BMycVZ8");
    }
}
