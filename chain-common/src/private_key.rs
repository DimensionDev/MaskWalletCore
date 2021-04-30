use super::public_key::PublicKey;
use crypto::curve::Curve;
use crypto::public_key::PublicKeyType;
use crypto::Error as CryptoError;
use std::str::FromStr;
use std::string::ToString;

// The number of bytes in a private key.
const VALID_SIZE: u8 = 32;
// The number of bytes in an extended private key.
const VALID_EXTENDED_SIZE: u8 = 3 * VALID_SIZE;

pub enum PrivateKeyType {
    PrivateKeyTypeDefault32 = 0,  // 32-byte private key
    PrivateKeyTypeExtended96 = 1, // 3*32-byte extended private key
    PrivateKeyTypeHd = 2,         // 32-byte private key
}

pub struct PrivateKey {
    pub data: Vec<u8>,
    pub extends_data: Vec<u8>,
    pub chain_code_bytes: Vec<u8>,
}

impl PrivateKey {
    pub fn get_private_key_type(curve: &Curve) -> PrivateKeyType {
        match curve {
            Curve::Ed25519Extended => PrivateKeyType::PrivateKeyTypeExtended96,
            Curve::Ed25519hd => PrivateKeyType::PrivateKeyTypeHd,
            _ => PrivateKeyType::PrivateKeyTypeDefault32,
        }
    }

    fn is_valid_data(data: &[u8]) -> bool {
        // Check length.  Extended key needs 3*32 bytes.
        if data.len() as u8 != VALID_SIZE && data.len() as u8 != VALID_EXTENDED_SIZE {
            return false;
        }
        // Check whether data is not all zero
        return data.iter().any(|&x| x != 0);
    }

    pub fn is_valid(data: &[u8], curve: &str) -> Result<(), CryptoError> {
        if !Self::is_valid_data(data) {
            return Err(CryptoError::InvalidPrivateKey);
        }
        Curve::from_str(curve)
            .map_err(|_| CryptoError::NotSupportedCurve)
            .map(|_| {})
    }

    fn new_extended(data: &[u8], ext: &[u8], chain_code: &[u8]) -> Result<PrivateKey, CryptoError> {
        if !Self::is_valid_data(data)
            || !Self::is_valid_data(ext)
            || !Self::is_valid_data(chain_code)
        {
            return Err(CryptoError::InvalidPrivateKey);
        }
        Ok(PrivateKey {
            data: data.to_vec(),
            extends_data: ext.to_vec(),
            chain_code_bytes: chain_code.to_vec(),
        })
    }

    pub fn new(data: &[u8]) -> Result<PrivateKey, CryptoError> {
        if !Self::is_valid_data(data) {
            return Err(CryptoError::InvalidPrivateKey);
        }
        if data.len() == VALID_EXTENDED_SIZE as usize {
            Self::new_extended(&data[0..32], &data[32..64], &data[64..96])
        } else {
            Ok(PrivateKey {
                data: data.to_vec(),
                extends_data: vec![],
                chain_code_bytes: vec![],
            })
        }
    }

    pub fn get_public_key(&self, public_key_type_str: &str) -> Result<PublicKey, CryptoError> {
        let public_key_type = PublicKeyType::from_str(public_key_type_str)
            .map_err(|_| CryptoError::NotSupportedPublicKeyType)?;
        let pub_key_data = crypto::public_key::get_public_key(
            public_key_type_str,
            &self.data,
            &self.extends_data,
            &self.chain_code_bytes,
        )?;
        PublicKey::new(public_key_type, &pub_key_data)
    }
}

impl FromStr for PrivateKey {
    type Err = CryptoError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = hex::decode(s.as_bytes()).map_err(|_| CryptoError::InvalidPrivateKey)?;
        Self::new(&bytes)
    }
}

impl ToString for PrivateKey {
    fn to_string(&self) -> String {
        hex::encode(&self.data)
    }
}

#[cfg(test)]
mod tests {
    use crate::private_key::PrivateKey;
    use hex;

    #[test]
    fn test_get_public_key_secp256k1extended() {
        let priv_key_str = "18dd1dcd752466afa3d1fac1424333c6461c3a0f1d6702e9c45bc9254ec74e5f";
        let priv_key_data = hex::decode(priv_key_str).unwrap();
        let priv_key = PrivateKey::new(&priv_key_data).unwrap();
        let pub_key = priv_key.get_public_key("secp256k1extended").unwrap();

        let pub_key_hex = hex::encode(&pub_key.data);
        assert_eq!(pub_key_hex, "04bdfb71e2d953406c45279ac434667a6a1ea9fae608af91e7f6bfb0792011df760895a528e8b83622886039b4803b6182d708fb40a16919bddaef84493ef1d4cf");

        let priv_key_str2 = "afeefca74d9a325cf1d6b6911d61a65c32afa8e02bd5e78e2e4ac2910bab45f5";
        let priv_key_data2 = hex::decode(priv_key_str2).unwrap();
        let priv_key2 = PrivateKey::new(&priv_key_data2).unwrap();
        let pub_key2 = priv_key2.get_public_key("secp256k1extended").unwrap();

        let pub_key_hex2 = hex::encode(&pub_key2.data);
        assert_eq!(pub_key_hex2, "0499c6f51ad6f98c9c583f8e92bb7758ab2ca9a04110c0a1126ec43e5453d196c166b489a4b7c491e7688e6ebea3a71fc3a1a48d60f98d5ce84c93b65e423fde91");
    }
}
