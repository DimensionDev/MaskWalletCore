use super::address::EthereumAddress;
use super::transaction::{Eip1559Transaction, LegacyTransaction, Transaction};
use chain_common::ethereum::{SignInput, SignOutput};
use chain_common::private_key::PrivateKey;
use chain_common::Error;
use ethereum_types::U256;
use secp256k1::Secp256k1;
use std::convert::TryFrom;

pub struct Signer;

impl Signer {
    pub fn sign(private_key: &PrivateKey, sign_input: &SignInput) -> Result<SignOutput, Error> {
        if !EthereumAddress::is_valid(&sign_input.to_address) {
            return Err(Error::InvalidSignInput);
        }
        let chain_id = sign_input.chain_id;
        let secp = Secp256k1::signing_only();
        let secrect_key = &secp256k1::SecretKey::from_slice(&private_key.data)
            .map_err(|_| Error::InvalidPrivateKey)?;
        let gas_price =
            U256::from_str_radix(&sign_input.gas_price, 16).map_err(|_| Error::InvalidSignInput)?;
        match gas_price == U256::from(0) {
            false => {
                let transaction = LegacyTransaction::try_from(sign_input)?;
                let hash = transaction
                    .hash(chain_id)
                    .map_err(|_| Error::InvalidSignInput)?;
                let (v_id, signature) = secp
                    .sign_recoverable(
                        &secp256k1::Message::from_slice(&hash)
                            .map_err(|_| Error::InvalidSignInput)?,
                        &secrect_key,
                    )
                    .serialize_compact();
                let mut r = signature[0..32].to_vec();
                let mut s = signature[32..64].to_vec();
                while r[0] == 0 {
                    r.remove(0);
                }
                while s[0] == 0 {
                    s.remove(0);
                }
                let mut v = v_id.to_i32();
                let chain_id_i32 = chain_id as i32;
                if transaction.uses_replay_protection() {
                    // Embed chainID in V param, for replay protection, legacy (EIP155)
                    if chain_id_i32 != 0 {
                        v += 35 + chain_id_i32 * 2;
                    } else {
                        v += 27;
                    }
                }
                let encoded = transaction.encode_transaction(v as u64, &r, &s, chain_id);
                Ok(SignOutput {
                    data: transaction.base.payload,
                    encoded,
                    r,
                    v: v as u32,
                    s,
                })
            }
            true => {
                let transaction = Eip1559Transaction::try_from(sign_input)?;
                let hash = transaction
                    .hash(chain_id)
                    .map_err(|_| Error::InvalidSignInput)?;
                let (v_id, signature) = secp
                    .sign_recoverable(
                        &secp256k1::Message::from_slice(&hash)
                            .map_err(|_| Error::InvalidSignInput)?,
                        &secrect_key,
                    )
                    .serialize_compact();
                let mut r = signature[0..32].to_vec();
                let mut s = signature[32..64].to_vec();
                while r[0] == 0 {
                    r.remove(0);
                }
                while s[0] == 0 {
                    s.remove(0);
                }
                let v = v_id.to_i32();
                let encoded = transaction.encode_transaction(v as u64, &r, &s, chain_id);
                Ok(SignOutput {
                    data: transaction.base.payload,
                    encoded,
                    r,
                    v: v as u32,
                    s,
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chain_common::ethereum::SignInput;
    use chain_common::private_key::PrivateKey;
    use hex;
    use std::convert::TryFrom;
    use std::str::FromStr;
    #[test]
    fn test_sign_hash() {
        let input = SignInput {
            chain_id: 1,
            nonce: "0x9".to_owned(),
            gas_limit: "0x5208".to_owned(),
            gas_price: "0x4a817c800".to_owned(),
            max_inclusion_fee_per_gas: "0x0".to_owned(),
            max_fee_per_gas: "0x0".to_owned(),
            amount: "0xde0b6b3a7640000".to_owned(),
            payload: "".as_bytes().to_vec(),
            to_address: "0x3535353535353535353535353535353535353535".to_owned(),
        };
        let transaction = LegacyTransaction::try_from(&input).unwrap();
        assert_eq!(transaction.base.amount.to_string(), "1000000000000000000");
        assert_eq!(transaction.base.nonce.to_string(), "9");
        assert_eq!(transaction.base.gas_limit.to_string(), "21000");
        assert_eq!(transaction.gas_price.to_string(), "20000000000");

        let hashed = transaction.hash(input.chain_id).unwrap();
        let hashed_hex = hex::encode(hashed);
        assert_eq!(
            hashed_hex,
            "daf5a779ae972f972197303d7b574746c7ef83eadac0f2791ad23db92e4c8e53"
        );
        let private_key = PrivateKey::from_str(
            "4646464646464646464646464646464646464646464646464646464646464646",
        )
        .unwrap();
        let output = Signer::sign(&private_key, &input).unwrap();
        assert_eq!(output.v, 37);

        assert_eq!(
            output.encoded,
            [
                248, 108, 9, 133, 4, 168, 23, 200, 0, 130, 82, 8, 148, 53, 53, 53, 53, 53, 53, 53,
                53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 53, 136, 13, 224, 182, 179, 167,
                100, 0, 0, 128, 37, 160, 40, 239, 97, 52, 11, 217, 57, 188, 33, 149, 254, 83, 117,
                103, 134, 96, 3, 225, 161, 93, 60, 113, 255, 99, 225, 89, 6, 32, 170, 99, 98, 118,
                160, 103, 203, 233, 216, 153, 127, 118, 26, 236, 183, 3, 48, 75, 56, 0, 204, 245,
                85, 201, 243, 220, 100, 33, 75, 41, 127, 177, 150, 106, 59, 109, 131
            ]
            .to_vec()
        );
    }

    #[test]
    fn test_sign2() {
        let payload_bytes = hex::decode("6b175474e89094c44da98b954eedeac495271d0f").unwrap();
        let input = SignInput {
            chain_id: 1,
            nonce: "0x0".to_owned(),
            gas_limit: "0x130b9".to_owned(),
            gas_price: "0x9c7652400".to_owned(),
            max_inclusion_fee_per_gas: "0x0".to_owned(),
            max_fee_per_gas: "0x0".to_owned(),
            amount: "0x1bc16d674ec80000".to_owned(),
            payload: payload_bytes.to_vec(),
            to_address: "0x5322b34c88ed0691971bf52a7047448f0f4efc84".to_owned(),
        };
        let transaction = LegacyTransaction::try_from(&input).unwrap();
        assert_eq!(transaction.base.amount.to_string(), "2000000000000000000");
        assert_eq!(transaction.base.nonce.to_string(), "0");
        assert_eq!(transaction.base.gas_limit.to_string(), "78009");
        assert_eq!(transaction.gas_price.to_string(), "42000000000");

        let private_key = PrivateKey::from_str(
            "608dcb1742bb3fb7aec002074e3420e4fab7d00cced79ccdac53ed5b27138151",
        )
        .unwrap();
        let output = Signer::sign(&private_key, &input).unwrap();
        assert_eq!(output.v, 37);
    }

    #[test]
    fn test_sign_short_address() {
        let input = SignInput {
            chain_id: 1,
            nonce: "0x9".to_owned(),
            gas_limit: "0x5208".to_owned(),
            gas_price: "0x4a817c800".to_owned(),
            max_inclusion_fee_per_gas: "0x0".to_owned(),
            max_fee_per_gas: "0x0".to_owned(),
            amount: "0xde0b6b3a7640000".to_owned(),
            payload: "".as_bytes().to_vec(),
            to_address: "0x146aed09cd9dea7a64de689c5d3ef73d2ee5ca".to_owned(), // short addr
        };
        let private_key = PrivateKey::from_str(
            "4646464646464646464646464646464646464646464646464646464646464646",
        )
        .unwrap();
        let sign_error = Signer::sign(&private_key, &input);
        assert_eq!(sign_error.is_err(), true);
        assert_eq!(sign_error.err().unwrap(), Error::InvalidSignInput);
    }

    #[test]
    fn test_sign_eip1559() {
        let input = SignInput {
            chain_id: 3,
            nonce: "0x6".to_owned(),
            gas_limit: "0x526C".to_owned(),
            gas_price: "0x0".to_owned(),
            max_inclusion_fee_per_gas: "0x77359400".to_owned(),
            max_fee_per_gas: "0xB2D05E00".to_owned(),
            amount: "0x1EE0C29F50CB1".to_owned(),
            payload: "".as_bytes().to_vec(),
            to_address: "0xB9F5771C27664bF2282D98E09D7F50cEc7cB01a7".to_owned(),
        };
        let private_key = PrivateKey::from_str(
            "4f96ed80e9a7555a6f74b3d658afdd9c756b0a40d4ca30c42c2039eb449bb904",
        )
        .unwrap();
        let sign_result = Signer::sign(&private_key, &input).unwrap();
        assert_eq!(sign_result.v, 0);
        assert_eq!(hex::encode(sign_result.encoded), "02f8710306847735940084b2d05e0082526c94b9f5771c27664bf2282d98e09d7f50cec7cb01a78701ee0c29f50cb180c080a092c336138f7d0231fe9422bb30ee9ef10bf222761fe9e04442e3a11e88880c64a06487026011dae03dc281bc21c7d7ede5c2226d197befb813a4ecad686b559e58");
    }
}
