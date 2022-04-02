use crate::Error;

pub trait Hashable {
    fn hash(&self, input: &[u8]) -> Result<Vec<u8>, Error>;
}

pub struct Hasher;

impl Hasher {
    pub fn hash<T: Hashable>(h: T, input: &[u8]) -> Result<Vec<u8>, Error> {
        h.hash(input)
    }
}

pub struct Keccak256;
impl Hashable for Keccak256 {
    fn hash(&self, input: &[u8]) -> Result<Vec<u8>, Error> {
        use tiny_keccak::{Hasher as KeccakHasher, Keccak};
        let mut hasher = Keccak::v256();
        hasher.update(input);
        let mut output = [0u8; 32];
        hasher.finalize(&mut output);
        Ok(output.to_vec())
    }
}

/* Helper hash functions */
pub fn compute_mac(derived_key: &[u8], encrypted_text: &[u8]) -> Vec<u8> {
    use tiny_keccak::{Hasher as KeccakHasher, Keccak};
    let result = [derived_key, encrypted_text].concat();
    let mut hasher = Keccak::v256();
    hasher.update(&result);
    let mut output = [0u8; 32];
    hasher.finalize(&mut output);
    output.to_vec()
}

pub fn dsha256(input: &[u8]) -> Vec<u8> {
    use sha2::{ Digest, Sha256 };
    Sha256::digest(&Sha256::digest(input)).to_vec()
}

#[cfg(test)]
mod tests {
    use tiny_keccak::{Hasher, Keccak};

    #[test]
    fn empty_keccak() {
        let keccak = Keccak::v256();
        let mut output = [0; 32];
        let expected = b"\
            \xc5\xd2\x46\x01\x86\xf7\x23\x3c\x92\x7e\x7d\xb2\xdc\xc7\x03\xc0\
            \xe5\x00\xb6\x53\xca\x82\x27\x3b\x7b\xfa\xd8\x04\x5d\x85\xa4\x70\
        ";

        keccak.finalize(&mut output);
        assert_eq!(expected, &output);
    }

    #[test]
    fn string_keccak_256() {
        let mut keccak = Keccak::v256();
        let mut in_and_out: [u8; 32] = [0; 32];
        for i in 1..6 {
            in_and_out[i as usize - 1] = i
        }
        let expected = b"\
            \x7d\x87\xc5\xea\x75\xf7\x37\x8b\xb7\x01\xe4\x04\xc5\x06\x39\x16\
            \x1a\xf3\xef\xf6\x62\x93\xe9\xf3\x75\xb5\xf1\x7e\xb5\x04\x76\xf4\
        ";
        keccak.update(&in_and_out[0..5]);
        keccak.finalize(&mut in_and_out);
        assert_eq!(expected, &in_and_out);
    }

    use crate::hash::compute_mac;
    #[test]
    fn test_compute_mac() {
        let input1 = "hello world";
        let input2 = "!";
        let expected_mac =
            "57caa176af1ac0433c5df30e8dabcd2ec1af1e92a26eced5f719b88458777cd6".to_owned();
        let mac = compute_mac(input1.as_bytes(), input2.as_bytes());
        let hex_mac = hex::encode(mac);
        assert_eq!(hex_mac, expected_mac);

        let wrong_mac = compute_mac(input2.as_bytes(), input1.as_bytes());
        let wrong_hex_mac = hex::encode(wrong_mac);
        assert_ne!(wrong_hex_mac, expected_mac);
    }
}
