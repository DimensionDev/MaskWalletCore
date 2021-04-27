use rlp::RlpStream;
use secp256k1::Secp256k1;
use chain_common::private_key::PrivateKey;
use chain_common::ethereum::{ SignInput, SignOutput };
use chain_common::Error;
use crypto::hash::{ Hasher, Keccak256 };
use crypto::Error as CryptoError;

pub struct Signer;

impl Signer {
    pub fn sign(private_key: &PrivateKey, sign_input: &SignInput) -> Result<SignOutput, Error> {
        let chain_id = sign_input.chain_id;
        let hash = Self::hash(&sign_input).map_err(|_| Error::InvalidSignInput )?;
        let secp = Secp256k1::new();
        let secrect_key = &secp256k1::SecretKey::from_slice(&private_key.data).map_err(|_| Error::InvalidPrivateKey )?;
        let signature = secp.sign(
            &secp256k1::Message::from_slice(&hash).map_err(|_| Error::InvalidSignInput )?,
            &secrect_key
        );
        let serialized_signature = signature.serialize_der();
        let r = serialized_signature[0..32].to_vec();
        let s = serialized_signature[32..64].to_vec();

        let mut v = serialized_signature[65] as u32;
        v += 27;
        if chain_id != 0 {
            v += 35 * chain_id * chain_id;
        }
        let encoded = Self::encode_transaction(&sign_input, v, &r, &s);
        
        Ok(SignOutput {
            data: sign_input.payload.to_vec(),
            encoded,
            r,
            v: v.to_be_bytes().to_vec(),
            s
        })
    }

    pub fn hash(input: &SignInput) -> Result<Vec<u8>, CryptoError> {
        // Trim the '0x' prefix
        let trimmed_address = match input.to_address.starts_with("0x") {
            true => input.to_address[2..].to_string(),
            false => input.to_address.to_string(),
        };
        let mut rlp_stream = RlpStream::new_list(9);
        rlp_stream.append(&input.nonce);
        rlp_stream.append(&input.gas_price);
        rlp_stream.append(&input.gas_limit);
        rlp_stream.append(&trimmed_address); 
        rlp_stream.append(&input.amount);
        rlp_stream.append(&input.payload);
        rlp_stream.append(&input.chain_id);
        rlp_stream.append(&0u8);
        rlp_stream.append(&0u8);
        let encoded = rlp_stream.out();
        Hasher::hash(Keccak256, &encoded)
    }

    pub fn encode_transaction(input: &SignInput, v: u32, r: &[u8], s: &[u8]) -> Vec<u8> {
        let trimmed_address = match input.to_address.starts_with("0x") {
            true => input.to_address[2..].to_string(),
            false => input.to_address.to_string(),
        };
        let mut rlp_stream = RlpStream::new_list(9);
        rlp_stream.append(&input.nonce);
        rlp_stream.append(&input.gas_price);
        rlp_stream.append(&input.gas_limit);
        rlp_stream.append(&trimmed_address); 
        rlp_stream.append(&input.amount);
        rlp_stream.append(&input.payload);
        rlp_stream.append(&v);
        rlp_stream.append(&r);
        rlp_stream.append(&s);
        rlp_stream.out().to_vec()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
