use rmp::encode::*;

use super::number_util::random_iv;
use super::Error;

enum Index {
    Version = 0,
    AuthorNetwork = 1,
    AuthorID = 2,
    AuthorPublicKeyAlgorithm = 3,
    AuthorPublicKey = 4,
    Encryption = 5,
    Data = 6,
}

pub fn encode_with_container(
    network: &str,
    authorId: &str,
    algr: u8,
    author_pub_key: &[u8],
    aes_key: &[u8],
    iv: &[u8],
    encrypted: &[u8],
) -> Result<Vec<u8>, Error> {
    let encoded_without_container = encode_v37(
        &network,
        &authorId,
        algr,
        &author_pub_key,
        &aes_key,
        &iv,
        &encrypted,
    )
    .map_err(|_| Error::InvalidCiphertext)?;
    let mut buf = Vec::new();
    write_map_len(&mut buf, 2).map_err(|_| Error::InvalidCiphertext)?;
    write_sint(&mut buf, 0).map_err(|_| Error::InvalidCiphertext)?;
    write_bin(&mut buf, &encoded_without_container).map_err(|_| Error::InvalidCiphertext)?;
    Ok(buf)
}

fn encode_v37(
    network: &str,
    authorId: &str,
    algr: u8,
    author_pub_key: &[u8],
    aes_key: &[u8],
    iv: &[u8],
    encrypted: &[u8],
) -> Result<Vec<u8>, Error> {
    let mut buf = Vec::new();
    write_map_len(&mut buf, 6).map_err(|_| Error::InvalidCiphertext)?;

    write_sint(&mut buf, Index::AuthorNetwork as i64).map_err(|_| Error::InvalidCiphertext)?;
    write_str(&mut buf, &network).map_err(|_| Error::InvalidCiphertext)?;

    write_sint(&mut buf, Index::AuthorID as i64).map_err(|_| Error::InvalidCiphertext)?;
    write_str(&mut buf, &authorId).map_err(|_| Error::InvalidCiphertext)?;

    write_sint(&mut buf, Index::AuthorPublicKeyAlgorithm as i64)
        .map_err(|_| Error::InvalidCiphertext)?;
    write_sint(&mut buf, algr as i64).map_err(|_| Error::InvalidCiphertext)?;

    write_sint(&mut buf, Index::AuthorPublicKey as i64).map_err(|_| Error::InvalidCiphertext)?;
    write_bin(&mut buf, &author_pub_key);

    write_sint(&mut buf, Index::Encryption as i64).map_err(|_| Error::InvalidCiphertext)?;
    write_array_len(&mut buf, 3).map_err(|_| Error::InvalidCiphertext)?;
    write_sint(&mut buf, 0).map_err(|_| Error::InvalidCiphertext)?;
    write_bin(&mut buf, &aes_key);
    write_bin(&mut buf, &iv);

    write_sint(&mut buf, Index::Data as i64).map_err(|_| Error::InvalidCiphertext)?;
    write_bin(&mut buf, &encrypted).map_err(|_| Error::InvalidCiphertext)?;

    Ok(buf.to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rmp::encode::*;

    const IV_SIZE: usize = 16;
    const AES_KEY_SIZE: usize = 32;

    #[test]
    fn test_encode_v37() {
        // let post_iv = random_iv(IV_SIZE);
        // let post_key_iv = random_iv(AES_KEY_SIZE);
        // let author_key = random_iv(33);
        // let content = "sample text";

        // let encrypted_message = aes_encrypt(&post_iv, &post_key_iv, &content.as_bytes()).unwrap();
        // let message = "hello world";
        // let network = "localhost";
        // let authorId = "alice";
        // let algr = 2;
        // let encode_with_no_sign = encode_with_container(
        //     &network,
        //     &authorId,
        //     algr,
        //     &author_key,
        //     &post_key_iv,
        //     &post_iv,
        //     &encrypted_message,
        // )
        // .unwrap();
        // assert_eq!(&encode_with_no_sign, "1".as_bytes());
    }
}
