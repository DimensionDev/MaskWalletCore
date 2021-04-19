pub mod ctr {
    use aes_ctr::Aes128Ctr;
    use aes_ctr::cipher::{
        generic_array::GenericArray,
        stream::{
            NewStreamCipher, SyncStreamCipher
        }
    };

    use crate::Error;

    type CryptoResult<T> = Result<T, Error>;

    pub fn encrypt(data: &[u8], key: &[u8], iv: &[u8]) -> CryptoResult<Vec<u8>> {
        if key.len() != 16 || iv.len() != 16 {
            return Err(Error::InvalidKeyIvLength);
        }
        let key = GenericArray::from_slice(key);
        let iv = GenericArray::from_slice(iv);
        let mut cipher = Aes128Ctr::new(&key, &iv);
        let mut data_copy = vec![0; data.len()];
        data_copy.copy_from_slice(data);
        cipher.apply_keystream(&mut data_copy);
        Ok(data_copy)
    }

    pub fn decrypt(data: &[u8], key: &[u8], iv: &[u8]) -> CryptoResult<Vec<u8>> {
        if key.len() != 16 || iv.len() != 16 {
            return Err(Error::InvalidKeyIvLength);
        }
        let key = GenericArray::from_slice(key);
        let iv = GenericArray::from_slice(iv);
        let mut cipher = Aes128Ctr::new(key, iv);
        let mut data_copy = vec![0; data.len()];
        data_copy.copy_from_slice(data);
        cipher.apply_keystream(&mut data_copy);
        Ok(data_copy)
    }
}

#[cfg(test)]
mod tests {

    use hex::{ ToHex };

    #[test]
    fn ctr_encrypt_test() {
        use crate::aes::ctr::encrypt;

        let data = "MaskWallet".as_bytes();
        let key = hex::decode("01020304010203040102030401020304").unwrap();
        let iv = hex::decode("01020304010203040102030401020304").unwrap();
        let ret = encrypt(&data, &key, &iv).expect("encrypt nopadding data");
        let ret_hex = ret.encode_hex::<String>();

        assert_eq!("f89074571af13f467cd4", ret_hex);

        let wrong_len_key = hex::decode("010203040102030401020304").unwrap();
        let ret = encrypt(&data, &wrong_len_key, &iv);
        assert!(ret.is_err());

        let wrong_len_iv = hex::decode("010203040102030401020304").unwrap();
        let ret = encrypt(&data, &key, &wrong_len_iv);
        assert!(ret.is_err());
    }

    #[test]
    fn decrypted_data_test() {
        use crate::aes::ctr::decrypt;

        let data = "MaskWallet".as_bytes();
        let encrypted_data = hex::decode("f89074571af13f467cd4").unwrap();
        let key = hex::decode("01020304010203040102030401020304").unwrap();
        let iv = hex::decode("01020304010203040102030401020304").unwrap();
        let ret = decrypt(&encrypted_data, &key, &iv).expect("decrypted data error");

        assert_eq!(
            "MaskWallet",
            String::from_utf8(ret).expect("decrypted failed")
        );

        let wrong_len_key = hex::decode("010203040102030401020304").unwrap();
        let ret = decrypt(&data, &wrong_len_key, &iv);
        assert!(ret.is_err());

        let wrong_len_iv = hex::decode("010203040102030401020304").unwrap();
        let ret = decrypt(&data, &key, &wrong_len_iv);
        assert!(ret.is_err());
    }
}