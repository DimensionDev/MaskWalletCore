use super::address::EthereumAddress;
use super::transaction::Transaction;
use chain_common::ethereum::{SignInput, SignOutput};
use chain_common::private_key::PrivateKey;
use chain_common::Error;
use crypto::hash::{Hasher, Keccak256};
use crypto::Error as CryptoError;
use ethereum_types::U256;
use rlp::RlpStream;
use secp256k1::Secp256k1;
use std::convert::TryFrom;

pub struct Signer;

impl Signer {
    pub fn sign(private_key: &PrivateKey, sign_input: &SignInput) -> Result<SignOutput, Error> {
        if !EthereumAddress::is_valid(&sign_input.to_address) {
            return Err(Error::InvalidSignInput);
        }
        let chain_id = sign_input.chain_id;
        let transaction = Transaction::try_from(sign_input)?;
        let hash = Self::hash(&transaction).map_err(|_| Error::InvalidSignInput)?;
        let secp = Secp256k1::new();
        let secrect_key = &secp256k1::SecretKey::from_slice(&private_key.data)
            .map_err(|_| Error::InvalidPrivateKey)?;
        let (v_id, signature) = secp
            .sign_recoverable(
                &secp256k1::Message::from_slice(&hash).map_err(|_| Error::InvalidSignInput)?,
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
        if chain_id_i32 != 0 {
            v += 35 + chain_id_i32 * 2;
        }
        let encoded = Self::encode_transaction(&transaction, v as u64, &r, &s);
        Ok(SignOutput {
            data: transaction.payload,
            encoded,
            r,
            v: v as u32,
            s,
        })
    }

    pub fn hash(transaction: &Transaction) -> Result<Vec<u8>, CryptoError> {
        // let receiver_bytes = hex::decode(&transaction.receiver).unwrap();
        let mut rlp_stream = RlpStream::new_list(9);
        rlp_stream.append(&transaction.nonce);
        rlp_stream.append(&transaction.gas_price);
        rlp_stream.append(&transaction.gas_limit);
        if let Some(ref t) = transaction.receiver {
            rlp_stream.append(t);
        } else {
            rlp_stream.append(&vec![]);
        }
        rlp_stream.append(&transaction.amount);
        rlp_stream.append(&transaction.payload);
        rlp_stream.append(&(transaction.chain_id as u64));
        rlp_stream.append(&U256::zero());
        rlp_stream.append(&U256::zero());
        let encoded = rlp_stream.out();
        Hasher::hash(Keccak256, &encoded)
    }

    pub fn encode_transaction(transaction: &Transaction, v: u64, r: &[u8], s: &[u8]) -> Vec<u8> {
        let mut rlp_stream = RlpStream::new_list(9);
        rlp_stream.append(&transaction.nonce);
        rlp_stream.append(&transaction.gas_price);
        rlp_stream.append(&transaction.gas_limit);
        if let Some(ref t) = transaction.receiver {
            rlp_stream.append(t);
        } else {
            rlp_stream.append(&vec![]);
        }
        rlp_stream.append(&transaction.amount);
        rlp_stream.append(&transaction.payload);
        rlp_stream.append(&v);
        rlp_stream.append(&r);
        rlp_stream.append(&s);
        rlp_stream.out().to_vec()
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
            amount: "0xde0b6b3a7640000".to_owned(),
            payload: "".as_bytes().to_vec(),
            to_address: "0x3535353535353535353535353535353535353535".to_owned(),
        };
        let transaction = Transaction::try_from(&input).unwrap();
        assert_eq!(transaction.amount.to_string(), "1000000000000000000");
        assert_eq!(transaction.nonce.to_string(), "9");
        assert_eq!(transaction.gas_limit.to_string(), "21000");
        assert_eq!(transaction.gas_price.to_string(), "20000000000");

        let hashed = Signer::hash(&transaction).unwrap();
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
            amount: "0x1bc16d674ec80000".to_owned(),
            payload: payload_bytes.to_vec(),
            to_address: "0x5322b34c88ed0691971bf52a7047448f0f4efc84".to_owned(),
        };
        let transaction = Transaction::try_from(&input).unwrap();
        assert_eq!(transaction.amount.to_string(), "2000000000000000000");
        assert_eq!(transaction.nonce.to_string(), "0");
        assert_eq!(transaction.gas_limit.to_string(), "78009");
        assert_eq!(transaction.gas_price.to_string(), "42000000000");

        let private_key = PrivateKey::from_str(
            "608dcb1742bb3fb7aec002074e3420e4fab7d00cced79ccdac53ed5b27138151",
        )
        .unwrap();
        let output = Signer::sign(&private_key, &input).unwrap();
        assert_eq!(output.v, 37);
        let hex_r = hex::encode(output.r);
        let hex_s = hex::encode(output.s);
        assert_eq!(
            hex_r,
            "724c62ad4fbf47346b02de06e603e013f26f26b56fdc0be7ba3d6273401d98ce"
        );
        assert_eq!(
            hex_s,
            "032131cae15da7ddcda66963e8bef51ca0d9962bfef0547d3f02597a4a58c931"
        );
    fn test_sign_short_address() {
        let input = SignInput {
            chain_id: 1,
            nonce: "0x9".to_owned(),
            gas_limit: "0x5208".to_owned(),
            gas_price: "0x4a817c800".to_owned(),
            amount: "0xde0b6b3a7640000".to_owned(),
            payload: "".to_owned(),
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
}
