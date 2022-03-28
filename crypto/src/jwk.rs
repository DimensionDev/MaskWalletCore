use super::{curve::Curve, Error};

use base64_url::encode_to_string;
use bip39::Mnemonic;
pub use bitcoin::util::bip32::{DerivationPath, Error as BIP32Error};
use bitcoin::{network::constants::Network, util::bip32::ExtendedPrivKey};
use secp256k1::Secp256k1;

use std::{convert::Into, str::FromStr};

#[derive(Default)]
pub struct JWK {
    pub crv: String,
    pub identifier: Option<String>,
    pub ext: bool,
    pub x: String,
    pub y: String,
    pub key_ops: Vec<String>,
    pub kty: String,
    pub d: Option<String>,
}

impl JWK {
    pub fn derive_on(
        mnemonic: &str,
        password: &str,
        path: &str,
        curve: Curve,
    ) -> Result<JWK, Error> {
        match curve {
            Curve::Ed25519 => {
                let seed = Self::derive_seed(mnemonic, password)?;
                let path = ed25519_dalek_bip32::DerivationPath::from_str(path)
                    .map_err(|_| Error::InvalidDerivationpath)?;

                // path limit
                if !path.path().into_iter().fold(true, |acc, child| {
                    acc && matches!(child, ed25519_dalek_bip32::ChildIndex::Hardened(_))
                }) {
                    return Err(Error::InvalidDerivationpath);
                }

                let derived_key = ed25519_dalek_bip32::ExtendedSecretKey::from_seed(&seed)
                    .and_then(|extended| extended.derive(&path))
                    .map_err(|_| Error::InvalidSeed)?;

                let _sk = derived_key.secret_key.as_bytes();
                let _pk = derived_key.chain_code;
                // let identifier = String::from_utf8(pub_key.into());
                // .map_err(|_| Error::InvalidPublicKey)?
                // .replace("/", "|");

                Ok(JWK {
                    crv: "ed25519".to_string(),
                    identifier: Option::Some(format!("ec_key:ed25519/{:}", "")),
                    ext: true,
                    x: "".into(),
                    y: "".into(),
                    key_ops: vec!["deriveKey".to_string(), "deriveBits".to_string()],
                    kty: "EC".to_string(),
                    d: Option::Some("".into()),
                })
            }

            Curve::Secp256k1 => {
                let seed = Self::derive_seed(mnemonic, password)?;

                let sk = ExtendedPrivKey::new_master(Network::Bitcoin, &seed)
                    .map_err(|_| Error::InvalidCiphertext)?;
                let secp = Secp256k1::new();
                let path =
                    DerivationPath::from_str(path).map_err(|_| Error::InvalidDerivationpath)?;

                let derived_key = sk
                    .derive_priv(&secp, &path)
                    .map_err(|_| Error::InvalidDerivationpath)?;

                let _finger_print = derived_key.fingerprint(&secp);

                let sk_pub = derived_key.private_key.public_key(&secp);
                let mut d = String::new();
                encode_to_string(&derived_key.private_key.to_bytes(), &mut d);

                let ser_uncompressed_pub = sk_pub.key.serialize_uncompressed();
                let identifier = String::from_utf8(sk_pub.key.serialize().into())
                    .map_err(|_| Error::InvalidPublicKey)?
                    .replace("/", "|");
                let pub_x = &ser_uncompressed_pub[1..33];
                let pub_y = &ser_uncompressed_pub[33..];

                let mut pubx_string = String::new();
                encode_to_string(pub_x, &mut pubx_string);

                let mut puby_string = String::new();
                encode_to_string(pub_y, &mut puby_string);

                Ok(JWK {
                    crv: "K-256".to_string(),
                    identifier: Option::Some(format!("ec_key:secp256k1/{:}", identifier)),
                    ext: true,
                    x: pubx_string,
                    y: puby_string,
                    key_ops: vec!["deriveKey".to_string(), "deriveBits".to_string()],
                    kty: "EC".to_string(),
                    d: Option::Some(d),
                })
            }

            _ => Err(Error::NotSupportedCurve),
        }
    }

    fn derive_seed(mnemonic: &str, password: &str) -> Result<Vec<u8>, Error> {
        let mnemonic = Mnemonic::parse_normalized(&mnemonic.to_lowercase())?;
        let seed = mnemonic.to_seed_normalized(password).to_vec();

        Ok(seed)
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod test {
    use super::*;

    #[test]
    fn secp256k1_derive_test() {
        fn results_for(suit: Sec256k1) -> Result<(), Error> {
            let jwk = JWK::derive_on(suit.mnemonic_str, "", suit.path_str, Curve::Secp256k1)?;

            assert_eq!(suit.pub_x, jwk.x);
            assert_eq!(suit.pub_y, jwk.y);
            assert_eq!(suit.compressed_point, jwk.identifier.unwrap());

            Ok(())
        }
        for suit in vec![Sec256k1::bulk_suit(), Sec256k1::doss_suit()] {
            let _ = results_for(suit);
        }
    }

    #[test]
    fn ed25519_derive_test() {
        let suit = Sec256k1::bulk_suit();
        let _path = "m/44'/60'/0'/0'/0'";
        // hex::encode(suit.mnemonic_str.as);
        let _jwk = JWK::derive_on(suit.mnemonic_str, "", suit.path_str, Curve::Ed25519);
        // .and_then(|extended| extended.derive(&path));
        // let child_num = match path.path().last() {
        //     Some(num) => num.to_u32(),
        //     None => 0,
        // };
        // let pub_key = extended_key.public_key().to_bytes();

        println!("1313");
    }

    #[test]
    fn test_path_slice() {
        let path = "m/44'/60'/0'/0/0";
        let paths = path.split("/").collect::<Vec<&str>>();
        assert_eq!(paths, vec!["m", "44'", "60'", "0'", "0", "0"]);
    }

    struct Sec256k1<'a> {
        mnemonic_str: &'a str,
        path_str: &'a str,
        preferred_seed: Vec<u8>,
        master_key: Key,
        derived_key: Key,
        pub_x: &'a str,
        pub_y: &'a str,
        compressed_point: &'a str,
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

    impl<'a> Sec256k1<'a> {
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
                pub_x: "nnFZy8awx54l7P1hPUjeMu69EymJrsTolbnWdmCl5XE",
                pub_y: "5LymtUZeFBMO-SdJqmIq1FVX1cyfXgXuVsmOPkBdkAs",
                compressed_point: "ec_key:secp256k1/A55xWcvGsMeeJez9YT1I3jLuvRMpia7E6JW51nZgpeVx",
            }
        }
    }

    impl<'a> Sec256k1<'a> {
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
                pub_x: "qgoeG-gEKz8yP_ki_5Ozs1XLZ3M0b6aMOBTfNhmPMRw",
                pub_y: "mMGy9l21691y6i7PYMmTO5M11K4pSVc_w58gBDKXhDY",
                compressed_point: "ec_key:secp256k1/AqoKHhvoBCs/Mj/5Iv+Ts7NVy2dzNG+mjDgU3zYZjzEc",
            }
        }
    }
}
