use crypto::curve::Curve;

// The number of bytes in a private key.
const VALID_SIZE: u8 = 32;
// The number of bytes in an extended private key.
const VALID_EXTENDED_SIZE: u8 = 3 * VALID_SIZE;

pub struct PrivateKey {

}

impl PrivateKey {
    fn is_valid_data(data: &[u8]) -> bool {
        // Check length.  Extended key needs 3*32 bytes.
        if data.len() as u8 != VALID_SIZE && data.len() as u8 != VALID_EXTENDED_SIZE {
            return false
        }
        // Check whether data is not all zero
        return data.iter().any(|&x| x != 0);
    }

    pub fn is_valid(data: &[u8], curve: Curve) -> bool {
        if !Self::is_valid_data(data) {
            return false;
        }
        match curve {
            Curve::SECP256k1 => {
                return true
            },
            _ => return false
        }
    }
}