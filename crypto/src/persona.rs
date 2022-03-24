use std::str::FromStr;

use super::Error;
use bip39::Mnemonic;
use bitcoin::{
    hashes::{
        hmac::{Hmac, HmacEngine},
        sha512::Hash as SHA512Hash,
        // sha256::Hash as SHA256Hash,
        Hash, HashEngine,
    },
    network::constants::Network,
    util::bip32::{DerivationPath, ExtendedPrivKey},
};

use secp256k1::Secp256k1;

#[derive(Default)]
pub struct JWK<'a> {
    pub crv: &'a str,
    pub identifier: &'a str,
}

pub trait EncryptEngine {
    fn encrypt_content(&self, mnemonic_str: &str, password: &str, path: &str)
        -> Result<JWK, Error>;
    fn mnemonic_and_seed(
        &self,
        mnemonic_str: &str,
        password: &str,
    ) -> Result<(Vec<u8>, Mnemonic), Error> {
        let mnemonic = Mnemonic::from_str(&mnemonic_str.to_lowercase())?;
        let seed = mnemonic.to_seed_normalized(password).to_vec();

        Ok((seed, mnemonic))
    }

    fn hmac512(key: &str, seed: &[u8]) -> (Vec<u8>, Vec<u8>) {
        let mut engine = HmacEngine::<SHA512Hash>::new(key.as_bytes());
        engine.input(&seed);
        let hash = Hmac::<SHA512Hash>::from_engine(engine);
        let il = hash[0..32]
            .into_iter()
            .map(|x| x.clone())
            .collect::<Vec<_>>();
        let ir: Vec<u8> = hash[32..].into_iter().map(|x| x.clone()).collect();
        (il, ir)
    }
}

#[derive(Default)]
pub struct EncryptKey<T: EncryptEngine> {
    engine: T,
}

impl<T> EncryptKey<T>
where
    T: EncryptEngine,
{
    pub fn new(engine: T) -> EncryptKey<T>
    where
        T: EncryptEngine,
    {
        EncryptKey { engine: engine }
    }
}

impl<T: EncryptEngine> EncryptKey<T> {
    pub fn generate_jwk(
        &self,
        mnemonic_str: &str,
        password: &str,
        path: &str,
    ) -> Result<JWK, Error> {
        self.engine.encrypt_content(mnemonic_str, password, path)
    }
}

pub(crate) mod engine {
    use super::Secp256k1 as secp256k1;
    use super::*;
    #[derive(Default)]
    pub struct Secp256k1;

    impl EncryptEngine for Secp256k1 {
        fn encrypt_content(
            &self,
            mnemonic_str: &str,
            password: &str,
            path: &str,
        ) -> Result<JWK, Error> {
            let (seed, _mnemonic) = self.mnemonic_and_seed(mnemonic_str, password)?;
            let sk = ExtendedPrivKey::new_master(Network::Bitcoin, &seed)
                .map_err(|_| Error::InvalidCiphertext)?;
            let secp = secp256k1::new();
            let _derived_key = sk
                .derive_priv(&secp, &DerivationPath::from_str(path).unwrap())
                .map_err(|_| Error::InvalidDerivationpath)?;

            Ok(JWK::default())
        }
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod test {
    use bitcoin::util::psbt::serialize::Serialize;

    use super::*;

    #[test]
    fn secp256k1_test() {
        for suit in vec![PeronaTestSuit::bulk_suit(), PeronaTestSuit::doss_suit()] {
            let engine = engine::Secp256k1;
            let (seed, _mnemonic) = engine.mnemonic_and_seed(suit.mnemonic_str, "").unwrap();
            // let (il, ir) = engine::Secp256k1::hmac512("Bitcoin seed", &seed);

            let sk = ExtendedPrivKey::new_master(Network::Bitcoin, &seed).unwrap();

            let secp = Secp256k1::new();
            let derived_key = sk
                .derive_priv(&secp, &DerivationPath::from_str(suit.path_str).unwrap())
                .unwrap();

            let sk_pub = sk.private_key.public_key(&secp);

            let ser_compressed_pub = sk_pub.serialize();

            // master_key test
            assert_eq!(seed, suit.preferred_seed);
            assert_eq!(
                suit.master_key.private_key == sk.private_key.to_bytes(),
                true
            );
            assert_eq!(suit.master_key.chaincode == sk.chain_code.as_bytes(), true);
            assert_eq!(suit.master_key.public_key == ser_compressed_pub, true);

            // derive_key test
            assert_eq!(
                suit.derived_key.private_key == derived_key.private_key.to_bytes(),
                true
            );
            assert_eq!(
                suit.derived_key.chaincode == derived_key.chain_code.as_bytes(),
                true
            );
        }
    }

    #[test]
    fn test_path_slice() {
        let path = "m/44'/60'/0'/0/0";
        let paths = path.split("/").collect::<Vec<&str>>();
        assert_eq!(paths, vec!["m", "44'", "60'", "0'", "0", "0"]);
    }

    struct PeronaTestSuit<'a> {
        mnemonic_str: &'a str,
        path_str: &'a str,
        preferred_seed: Vec<u8>,
        master_key: Key,
        derived_key: Key,
    }

    #[derive(Debug, PartialEq, Eq)]
    struct Key {
        algorithm: String,
        chaincode: Vec<u8>,
        depth: i32,
        index: i32,
        key_identifier: Vec<u8>,
        partent_fingerprint: Vec<u8>,
        private_key: Vec<u8>,
        public_key: Vec<u8>,
    }

    impl<'a> PeronaTestSuit<'a> {
        fn bulk_suit() -> Self {
            Self {
                mnemonic_str:
                    "bulk vault region flip fury kitchen bread swallow bridge treat piano behave",
                path_str: "m/44'/60'/0'/0/0",
                preferred_seed: vec![
                    189, 42, 254, 109, 157, 245, 140, 114, 23, 88, 102, 33, 221, 89, 167, 234, 127,
                    1, 193, 102, 218, 177, 28, 187, 87, 96, 99, 38, 60, 15, 60, 207, 90, 211, 26,
                    76, 28, 142, 222, 10, 44, 127, 239, 246, 175, 228, 104, 13, 239, 144, 175, 142,
                    197, 220, 165, 228, 91, 175, 135, 52, 30, 78, 225, 36,
                ],

                master_key: Key {
                    algorithm: "secp256k1".to_owned(),
                    chaincode: vec![
                        164, 181, 123, 67, 78, 53, 244, 139, 52, 192, 94, 182, 218, 96, 217, 31,
                        24, 223, 40, 124, 160, 137, 77, 216, 37, 246, 223, 217, 161, 216, 178, 164,
                    ],
                    depth: 0,
                    index: 0,
                    key_identifier: vec![
                        191, 242, 229, 114, 111, 9, 6, 6, 160, 159, 21, 43, 13, 81, 20, 95, 38, 17,
                        39, 24,
                    ],
                    partent_fingerprint: vec![0, 0, 0, 0],
                    private_key: vec![
                        126, 193, 228, 81, 223, 250, 61, 186, 226, 128, 121, 253, 26, 62, 154, 82,
                        134, 223, 181, 235, 15, 47, 185, 238, 219, 125, 187, 8, 214, 132, 216, 165,
                    ],
                    public_key: vec![
                        3, 184, 219, 205, 44, 234, 225, 53, 23, 105, 163, 66, 132, 11, 168, 187,
                        213, 114, 35, 195, 24, 58, 222, 243, 37, 202, 51, 192, 125, 208, 195, 150,
                        209,
                    ],
                },

                derived_key: Key {
                    algorithm: "secp256k1".to_owned(),
                    chaincode: vec![
                        189, 111, 110, 248, 19, 75, 7, 113, 184, 224, 71, 241, 55, 139, 80, 142,
                        102, 116, 7, 81, 114, 14, 81, 241, 105, 54, 79, 238, 57, 14, 12, 47,
                    ],
                    depth: 5,
                    index: 0,
                    key_identifier: vec![
                        219, 29, 21, 98, 11, 20, 12, 22, 13, 64, 99, 32, 186, 53, 10, 46, 127, 8,
                        224, 251,
                    ],
                    partent_fingerprint: vec![177, 242, 18, 157],
                    private_key: vec![
                        67, 5, 185, 251, 248, 196, 69, 201, 233, 115, 129, 18, 107, 220, 233, 163,
                        128, 2, 50, 8, 56, 112, 130, 139, 178, 188, 227, 176, 207, 174, 229, 227,
                    ],
                    public_key: vec![
                        3, 158, 113, 89, 203, 198, 176, 199, 158, 37, 236, 253, 97, 61, 72, 222,
                        50, 238, 189, 19, 41, 137, 174, 196, 232, 149, 185, 214, 118, 96, 165, 229,
                        113,
                    ],
                },
            }
        }
    }

    impl<'a> PeronaTestSuit<'a> {
        fn doss_suit() -> Self {
            Self {
                mnemonic_str:
                    "dose grass fossil bike avocado owner high autumn interest mom memory claim",
                path_str: "m/44'/60'/0'/0/0",
                preferred_seed: vec![
                    211, 61, 100, 111, 253, 245, 200, 143, 15, 179, 159, 200, 12, 19, 148, 147, 98,
                    118, 141, 222, 174, 200, 93, 188, 26, 93, 48, 33, 36, 77, 157, 55, 149, 209, 6,
                    119, 44, 11, 209, 42, 232, 162, 27, 157, 70, 23, 12, 124, 236, 92, 231, 177,
                    150, 67, 65, 122, 158, 60, 48, 139, 14, 1, 240, 8,
                ],

                master_key: Key {
                    algorithm: "secp256k1".to_owned(),
                    chaincode: vec![
                        75, 61, 83, 161, 214, 137, 104, 126, 172, 51, 212, 95, 173, 39, 89, 141,
                        217, 157, 30, 215, 4, 164, 240, 232, 183, 199, 152, 190, 202, 76, 2, 57,
                    ],
                    depth: 0,
                    index: 0,
                    key_identifier: vec![
                        76, 27, 90, 203, 62, 24, 224, 143, 64, 38, 19, 163, 136, 136, 182, 131,
                        144, 239, 44, 120,
                    ],
                    partent_fingerprint: vec![0, 0, 0, 0],
                    private_key: vec![
                        159, 227, 189, 245, 178, 68, 163, 121, 5, 61, 207, 245, 51, 152, 231, 195,
                        239, 88, 236, 126, 182, 137, 155, 34, 222, 111, 209, 2, 164, 114, 55, 26,
                    ],
                    public_key: vec![
                        2, 11, 111, 15, 210, 211, 70, 97, 239, 148, 73, 125, 37, 8, 53, 40, 255,
                        55, 34, 122, 193, 148, 194, 128, 225, 20, 45, 30, 240, 135, 243, 191, 13,
                    ],
                },
                derived_key: Key {
                    algorithm: "secp256k1".to_owned(),
                    chaincode: vec![
                        110, 106, 1, 194, 8, 4, 105, 104, 194, 52, 102, 253, 126, 199, 1, 231, 45,
                        175, 5, 194, 70, 53, 180, 106, 107, 124, 229, 213, 135, 233, 45, 135,
                    ],
                    depth: 5,
                    index: 0,
                    key_identifier: vec![
                        213, 249, 118, 248, 138, 255, 172, 4, 83, 178, 102, 88, 113, 16, 49, 84,
                        56, 229, 116, 155,
                    ],
                    partent_fingerprint: vec![177, 242, 18, 157],
                    private_key: vec![
                        164, 220, 24, 245, 162, 159, 141, 176, 18, 151, 248, 162, 174, 140, 138,
                        146, 6, 126, 21, 156, 237, 185, 200, 177, 167, 250, 42, 150, 246, 13, 30,
                        134,
                    ],
                    public_key: vec![
                        2, 170, 10, 30, 27, 232, 4, 43, 63, 50, 63, 249, 34, 255, 147, 179, 179,
                        85, 203, 103, 115, 52, 111, 166, 140, 56, 20, 223, 54, 25, 143, 49, 28,
                    ],
                },
            }
        }
    }
}
