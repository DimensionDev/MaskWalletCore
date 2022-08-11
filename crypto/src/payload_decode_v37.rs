use std::str::from_utf8;

use super::payload_encode_v37::Index;
use super::Error;
use rmp::decode::*;

struct DecodedData {
    network: String,
    author_id: String,
    algorithm: i64,
    author_pub_key: Vec<u8>,
    aes_key: Vec<u8>,
    iv: Vec<u8>,
    encrypted_content: Vec<u8>,
}

fn decode_with_container(encrypted: &[u8]) -> Result<DecodedData, Error> {
    let mut content = Bytes::new(encrypted);
    let map_len = read_map_len(&mut content).map_err(|_| Error::InvalidCiphertext)?;
    if map_len != 2 {
        return Err(Error::InvalidCiphertext);
    }

    let flag: i64 = read_int(&mut content).map_err(|_| Error::InvalidCiphertext)?;
    if flag != 0 {
        return Err(Error::InvalidCiphertext);
    }
    let _ = read_bin_len(&mut content).map_err(|_| Error::InvalidCiphertext)?;

    let data_map_len = read_map_len(&mut content).map_err(|_| Error::InvalidCiphertext)?;
    if data_map_len != 6 {
        return Err(Error::InvalidCiphertext);
    }

    // network
    let author_network =
        decode_str(&mut content, Index::AuthorNetwork).map_err(|_| Error::InvalidCiphertext)?;

    // author_id
    let author_decoded_id =
        decode_str(&mut content, Index::AuthorID).map_err(|_| Error::InvalidCiphertext)?;

    // algorithm
    let author_pub_key_algorithm: i64 = decode_int64(&mut content, Index::AuthorPublicKeyAlgorithm)
        .map_err(|_| Error::InvalidCiphertext)?;

    // author_pub_key
    let author_pub_key = decode_bin(&mut content, Option::Some(Index::AuthorPublicKey))
        .map_err(|_| Error::InvalidCiphertext)?;

    // iv and aes_key
    let encryption_index: i64 = read_int(&mut content).map_err(|_| Error::InvalidCiphertext)?;
    match encryption_index.try_into()? {
        Index::Encryption => {}
        _ => return Err(Error::InvalidCiphertext),
    }
    let array_len = read_array_len(&mut content).map_err(|_| Error::InvalidCiphertext)?;
    if array_len != 3 {
        return Err(Error::InvalidCiphertext);
    }
    let flag: i64 = read_int(&mut content).map_err(|_| Error::InvalidCiphertext)?;
    if flag != 0 {
        return Err(Error::InvalidCiphertext);
    }
    let aes_key = decode_bin(&mut content, Option::None).map_err(|_| Error::InvalidCiphertext)?;
    let decoded_iv =
        decode_bin(&mut content, Option::None).map_err(|_| Error::InvalidCiphertext)?;

    // encrypted text
    let decoded_data = decode_bin(&mut content, Option::Some(Index::Data))
        .map_err(|_| Error::InvalidCiphertext)?;

    Ok(DecodedData {
        network: author_network,
        author_id: author_decoded_id,
        algorithm: author_pub_key_algorithm,
        author_pub_key,
        aes_key,
        iv: decoded_iv,
        encrypted_content: decoded_data,
    })
}

fn decode_str(bytes: &mut Bytes, index: Index) -> Result<String, Error> {
    let index_value: i64 = rmp::decode::read_int(bytes).map_err(|_| Error::InvalidCiphertext)?;
    let index_value: Index = index_value.try_into()?;

    if index_value != index {
        return Err(Error::InvalidCiphertext);
    }

    let str_len = read_str_len(bytes).map_err(|_| Error::InvalidCiphertext)?;
    let mut str_buf = [0u8; 1].repeat(str_len as usize);
    let _ = bytes
        .read_exact_buf(&mut str_buf)
        .map_err(|_| Error::InvalidCiphertext)?;

    match from_utf8(&str_buf) {
        Ok(decoded) => Ok(decoded.to_owned()),
        _ => Err(Error::InvalidCiphertext),
    }
}

fn decode_int64(bytes: &mut Bytes, index: Index) -> Result<i64, Error> {
    let decoded_index: i64 = rmp::decode::read_int(bytes).map_err(|_| Error::InvalidCiphertext)?;
    let decoded_index: Index = decoded_index.try_into()?;
    if decoded_index != index {
        return Err(Error::InvalidCiphertext);
    }

    match read_int(bytes) {
        Ok(decoded) => Ok(decoded),
        _ => Err(Error::InvalidCiphertext),
    }
}

fn decode_bin(bytes: &mut Bytes, index: Option<Index>) -> Result<Vec<u8>, Error> {
    if let Some(index) = index {
        let data_index: i64 = read_int(bytes).map_err(|_| Error::InvalidCiphertext)?;
        let data_index: Index = data_index.try_into()?;
        if data_index != index {
            return Err(Error::InvalidCiphertext);
        }
    }

    let data_len = read_bin_len(bytes).map_err(|_| Error::InvalidCiphertext)?;
    let mut decoded_data = [0u8; 1].repeat(data_len as usize);
    bytes
        .read_exact_buf(&mut decoded_data)
        .map_err(|_| Error::InvalidCiphertext)?;

    Ok(decoded_data.to_vec())
}

#[cfg(test)]
mod tests {
    use super::super::number_util::random_iv;
    use super::super::payload_encode_v37::*;
    use super::super::Error;
    use super::*;

    const IV_SIZE: usize = 16;
    const AES_KEY_SIZE: usize = 32;

    #[test]
    fn test_decode_v37() -> Result<(), Error> {
        let post_iv = random_iv(IV_SIZE);
        let post_key_iv = random_iv(AES_KEY_SIZE);
        let author_key = random_iv(33);
        let text_content = random_iv(32);
        let network = "eht";
        let author_id = "1331444";
        let algr = 2;

        let encrypted = encode_with_container(
            network,
            author_id,
            algr,
            &post_key_iv,
            &author_key,
            &post_iv,
            &text_content,
        )
        .unwrap();

        let result = decode_with_container(&encrypted).map_err(|_| Error::InvalidCiphertext)?;

        assert_eq!(result.network, network);
        assert_eq!(result.author_id, author_id);
        assert_eq!(result.algorithm, algr as i64);
        assert_eq!(result.author_pub_key, post_key_iv);
        assert_eq!(result.aes_key, author_key);
        assert_eq!(result.iv, post_iv);
        assert_eq!(result.encrypted_content, text_content);

        Ok(())
    }
}
