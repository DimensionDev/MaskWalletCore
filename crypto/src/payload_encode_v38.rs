use super::aes_gcm::aes_encrypt;
use super::number_util::random_iv;
use super::post_encryption::EncryptionResultE2E;
use super::Error;
use bitcoin::secp256k1::{ecdh, PublicKey, SecretKey};
use std::collections::HashMap;

impl From<bitcoin::secp256k1::Error> for Error {
    fn from(_err: bitcoin::secp256k1::Error) -> Error {
        Error::InvalidPrivateKey
    }
}

use base64::{decode_config, encode_config, STANDARD, URL_SAFE_NO_PAD};

const SHARED_KEY_ENCODED: &str = "3Bf8BJ3ZPSMUM2jg2ThODeLuRRD_-_iwQEaeLdcQXpg";
const E2E_KEY: [u8; 2] = [40, 70];
const E2E_IV: [u8; 1] = [33];

enum Index {
    AuthorPublicKey = 5,
    PublicShared = 6,
    AuthorIdentifier = 7,
}

#[allow(clippy::too_many_arguments)]
pub fn encode_v38(
    is_public: bool,
    network: &str,
    author_id: Option<&str>,
    iv: &[u8],
    key: &[u8],
    encrypted: &[u8],
    author_pub_key: Option<&[u8]>,
    local_key_data: Option<&[u8]>,
    target: HashMap<String, Vec<u8>>,
    author_private_key: Option<&[u8]>,
) -> Result<(String, Option<HashMap<String, EncryptionResultE2E>>), Error> {
    let base64_config = STANDARD;
    let (aes_key_encrypted, ecdh_result): (String, Option<HashMap<String, EncryptionResultE2E>>) =
        match is_public {
            true => (encode_aes_key_encrypted(iv, key)?, None),
            false => {
                let local_key = local_key_data.ok_or(Error::InvalidLocalKey)?;
                let post_key_encoded = encode_post_key(key);
                let owners_aes_key_encrypted =
                    encrypt_by_local_key(&post_key_encoded, iv, local_key)?;
                let author_private_key_data = author_private_key.ok_or(Error::InvalidPrivateKey)?;
                let ecdh_result =
                    add_receiver(author_private_key_data, &target, &post_key_encoded)?;
                let owners_aes_key_encrypted_string =
                    encode_config(&owners_aes_key_encrypted, base64_config);
                (owners_aes_key_encrypted_string, Some(ecdh_result))
            }
        };

    let encoded_iv = encode_config(&iv, base64_config);
    let encoded_encrypted = encode_config(&encrypted, base64_config);
    let signature = "_";
    let encoded_fields = encode_fields(
        is_public,
        &aes_key_encrypted,
        &encoded_iv,
        &encoded_encrypted,
        signature,
        network,
        author_id,
        author_pub_key,
    )?;
    Ok((encoded_fields, ecdh_result))
}

fn encode_aes_key_encrypted(iv: &[u8], key: &[u8]) -> Result<String, Error> {
    let base64_url_config = URL_SAFE_NO_PAD;
    let encoded_aes_key = encode_config(&key, base64_url_config);
    let ab = format!(
        r#"{{"alg":"A256GCM","ext":true,"k":"{}","key_ops":["decrypt","encrypt"],"kty":"oct"}}"#,
        &encoded_aes_key
    );
    let ab_bytes = ab.as_bytes();
    let shared_key_bytes = decode_config(&SHARED_KEY_ENCODED, base64_url_config)
        .map_err(|_| Error::InvalidCiphertext)?;
    let encrypted_key = aes_encrypt(iv, &shared_key_bytes, ab_bytes)?;
    let base64_config = STANDARD;
    let encoded_key = encode_config(&encrypted_key, base64_config);
    Ok(encoded_key)
}

fn encrypt_by_local_key(
    encoded_post_key: &[u8],
    post_iv: &[u8],
    local_key_data: &[u8],
) -> Result<Vec<u8>, Error> {
    aes_encrypt(post_iv, local_key_data, encoded_post_key)
}

#[allow(clippy::too_many_arguments)]
fn encode_fields(
    is_public: bool,
    aes_key_encrypted: &str,
    encoded_iv: &str,
    encoded_encrypted: &str,
    signature: &str,
    network: &str,
    author_id: Option<&str>,
    author_pub_key: Option<&[u8]>,
) -> Result<String, Error> {
    let mut fields: [&str; 8] = [
        "\u{1F3BC}4/4",
        aes_key_encrypted,
        encoded_iv,
        encoded_encrypted,
        signature,
        "",
        "",
        "",
    ];

    let public_key_str = match author_pub_key {
        Some(key_data) => {
            let base64_config = STANDARD;
            let public_key =
                PublicKey::from_slice(key_data).map_err(|_| Error::InvalidPrivateKey)?;
            let compressed_key = public_key.serialize();
            encode_config(&compressed_key, base64_config)
        }
        None => "".to_string(),
    };
    fields[Index::AuthorPublicKey as usize] = &public_key_str;

    match is_public {
        true => {
            fields[Index::PublicShared as usize] = "1";
        }
        false => {
            fields[Index::PublicShared as usize] = "0";
        }
    }

    let identity = match author_id {
        Some(author_id) => {
            let profile_identifier = format!("{}/{}", network, author_id);
            let base64_config = STANDARD;
            encode_config(&profile_identifier, base64_config)
        }
        _ => "".to_string(),
    };
    fields[Index::AuthorIdentifier as usize] = &identity;

    let joined_fields = fields.join("|");
    let result = format!("{}:||", joined_fields);
    Ok(result)
}

fn encode_post_key(post_key: &[u8]) -> Vec<u8> {
    let base64_url_config = URL_SAFE_NO_PAD;
    let encoded_post_key = encode_config(&post_key, base64_url_config);
    let result = format!(
        r#"{{"alg":"A256GCM","ext":true,"k":"{}","key_ops":["decrypt","encrypt"],"kty":"oct"}}"#,
        &encoded_post_key
    );
    result.as_bytes().to_vec()
}

fn add_receiver(
    author_private_key: &[u8],
    target: &HashMap<String, Vec<u8>>,
    encoded_post_key: &[u8],
) -> Result<HashMap<String, EncryptionResultE2E>, Error> {
    let mut ecdh_result = HashMap::new();
    for (profile_id, receiver_public_key) in target.iter() {
        let iv_to_be_published = random_iv(16);
        let (aes, iv) = derive_ecdh_and_extra_steps(
            receiver_public_key,
            author_private_key,
            &iv_to_be_published,
        )?;
        let encrypted_post_key = aes_encrypt(&iv, &aes, encoded_post_key)?;
        let result = EncryptionResultE2E {
            target: profile_id.to_string(),
            iv_to_be_published: Some(iv_to_be_published),
            encrypted_post_key,
        };
        ecdh_result.insert(profile_id.to_string(), result);
    }
    Ok(ecdh_result)
}

fn derive_ecdh_and_extra_steps(
    public_key: &[u8],
    author_private_key: &[u8],
    iv: &[u8],
) -> Result<(Vec<u8>, [u8; 16]), Error> {
    use sha2::{Digest, Sha256};
    let derive_result = derive_aes_by_ecdh(public_key, author_private_key)?;
    let mut _a = Vec::new();
    _a.extend(&derive_result);
    _a.extend(iv);

    let mut next_key_material_raw = Vec::new();
    next_key_material_raw.extend(&_a);
    next_key_material_raw.extend(iv);
    next_key_material_raw.extend(E2E_KEY);
    let next_aes_key_material = Sha256::digest(&next_key_material_raw);

    let mut iv_pre_raw = Vec::new();
    iv_pre_raw.extend(&_a);
    iv_pre_raw.extend(iv);
    iv_pre_raw.extend(E2E_IV);
    let iv_pre = Sha256::digest(&iv_pre_raw);

    let mut next_iv: [u8; 16] = [0; 16];
    for i in 0..16 {
        next_iv[i] = iv_pre[i] ^ iv_pre[16 + i];
    }
    Ok((next_aes_key_material.to_vec(), next_iv))
}

fn derive_aes_by_ecdh(public_key: &[u8], private_key: &[u8]) -> Result<Vec<u8>, Error> {
    let pub_key = PublicKey::from_slice(public_key)?;
    let sec_key = SecretKey::from_slice(private_key)?;
    let shared_secret = ecdh::SharedSecret::new(&pub_key, &sec_key);
    Ok(shared_secret.as_ref().to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::aes_gcm::aes_decrypt;
    use crate::number_util::random_iv;
    use sha2::{Digest, Sha256};
    #[test]
    fn test_encode_aes_key() {
        let iv = [
            44, 67, 220, 0, 135, 88, 111, 139, 0, 72, 96, 128, 156, 163, 95, 183,
        ];
        let key_iv = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];

        let encoded_key = encode_aes_key_encrypted(&iv, &key_iv).unwrap();
        assert_eq!(encoded_key, "7jLN2yGxMEVM28cIjVlJJ1PBSh6qt3qgUoDL579dssh4EQoxImWZfezILlxTMtoPEFzIN8T369jz2Pai2IzrI9coSAr+V46S91/4Bh2QnlSsWc6B+IZIc/hIWhFKBUeU+5bq/SvBYSVpE5/+C4sIk8beyHIl");
    }

    #[test]
    fn test_encode_fields() {
        let aes_key_encrypted = "8NXbnHHTwNaQlnihC4ov7JiAXIMfjmpP6LZG9SCpsBTGgscJuET25HO0DfkXOmjtepWV5NAGzRn5iFJENjTtIeMmnAaDl7ijSmsIfcS6Gp9wQZZ2yUaAj4S1rN6zCx6uZQNPaVH2kywLfQVZJ+pxNflXmKYgNcw53yG/XKgI7ksqCnwWqiqQyYYS";
        let encoded_iv = "Q43qdWbDoDbWBca2+LB6lA==";
        let encoded_encrypted = "h6WdGLrQ+H2fMJrXVFwKFw+IiQ==";
        let signature = "_";
        let network = "twitter.com";
        let author_id = "yuan_brad";
        let public_key_data = [
            2, 210, 107, 119, 140, 57, 180, 37, 245, 126, 86, 79, 41, 128, 107, 64, 99, 141, 222,
            6, 87, 249, 95, 130, 198, 99, 1, 113, 41, 91, 239, 152, 212,
        ];

        let encoded_fields = encode_fields(
            true,
            aes_key_encrypted,
            encoded_iv,
            encoded_encrypted,
            signature,
            network,
            Some(author_id),
            Some(&public_key_data),
        )
        .unwrap();
        assert_eq!(encoded_fields, "🎼4/4|8NXbnHHTwNaQlnihC4ov7JiAXIMfjmpP6LZG9SCpsBTGgscJuET25HO0DfkXOmjtepWV5NAGzRn5iFJENjTtIeMmnAaDl7ijSmsIfcS6Gp9wQZZ2yUaAj4S1rN6zCx6uZQNPaVH2kywLfQVZJ+pxNflXmKYgNcw53yG/XKgI7ksqCnwWqiqQyYYS|Q43qdWbDoDbWBca2+LB6lA==|h6WdGLrQ+H2fMJrXVFwKFw+IiQ==|_|AtJrd4w5tCX1flZPKYBrQGON3gZX+V+CxmMBcSlb75jU|1|dHdpdHRlci5jb20veXVhbl9icmFk:||");
    }

    #[test]
    fn test_ecdh_derive() {
        // Check whether the `derive_aes_by_ecdh` method could derive a valid aes key
        let test_message = "hello world";
        let test_iv = random_iv(16);
        let private_key = vec![
            164, 220, 24, 245, 162, 159, 141, 176, 18, 151, 248, 162, 174, 140, 138, 146, 6, 126,
            21, 156, 237, 185, 200, 177, 167, 250, 42, 150, 246, 13, 30, 134,
        ];
        let public_key = vec![
            2, 170, 10, 30, 27, 232, 4, 43, 63, 50, 63, 249, 34, 255, 147, 179, 179, 85, 203, 103,
            115, 52, 111, 166, 140, 56, 20, 223, 54, 25, 143, 49, 28,
        ];

        assert!(PublicKey::from_slice(&public_key).is_ok());
        assert!(SecretKey::from_slice(&private_key).is_ok());
        let shared_secret = derive_aes_by_ecdh(&public_key, &private_key).unwrap();
        let encrypted = aes_encrypt(&test_iv, &shared_secret, test_message.as_bytes()).unwrap();
        let decrypted = aes_decrypt(&test_iv, &shared_secret, &encrypted).unwrap();
        assert_eq!(decrypted, test_message.as_bytes());
    }

    #[test]
    fn test_derive_ecdh_and_extra_steps() {
        let derived_key_raw = [
            62, 155, 237, 84, 13, 3, 137, 47, 239, 227, 65, 3, 107, 135, 75, 66, 118, 27, 77, 132,
            91, 79, 223, 58, 248, 249, 95, 193, 42, 88, 199, 12,
        ];
        let iv = [
            148, 238, 119, 124, 75, 191, 117, 180, 14, 0, 77, 65, 63, 213, 1, 227,
        ];
        let mut _a = Vec::new();
        _a.extend(&derived_key_raw);
        _a.extend(iv);

        assert_eq!(
            &_a,
            &[
                62, 155, 237, 84, 13, 3, 137, 47, 239, 227, 65, 3, 107, 135, 75, 66, 118, 27, 77,
                132, 91, 79, 223, 58, 248, 249, 95, 193, 42, 88, 199, 12, 148, 238, 119, 124, 75,
                191, 117, 180, 14, 0, 77, 65, 63, 213, 1, 227
            ]
        );

        let mut next_key_material_raw = Vec::new();
        next_key_material_raw.extend(&_a);
        next_key_material_raw.extend(iv);
        next_key_material_raw.extend(E2E_KEY);
        let next_aes_key_material = Sha256::digest(&next_key_material_raw);
        assert_eq!(
            &next_aes_key_material.to_vec(),
            &[
                74, 12, 205, 110, 243, 104, 194, 172, 14, 90, 45, 147, 214, 168, 127, 97, 242, 39,
                56, 126, 197, 0, 228, 66, 97, 6, 86, 132, 38, 76, 166, 24
            ]
        );

        let mut iv_pre_raw = Vec::new();
        iv_pre_raw.extend(&_a);
        iv_pre_raw.extend(iv);
        iv_pre_raw.extend(E2E_IV);
        let iv_pre = Sha256::digest(&iv_pre_raw);

        assert_eq!(
            &iv_pre.to_vec(),
            &[
                194, 208, 143, 170, 171, 60, 198, 137, 133, 142, 75, 252, 168, 127, 65, 229, 41,
                208, 41, 99, 233, 19, 118, 190, 203, 252, 150, 137, 221, 215, 144, 68
            ]
        );

        let mut next_iv: [u8; 16] = [0; 16];
        for i in 0..16 {
            next_iv[i] = iv_pre[i] ^ iv_pre[16 + i];
        }

        assert_eq!(
            &next_iv,
            &[235, 0, 166, 201, 66, 47, 176, 55, 78, 114, 221, 117, 117, 168, 209, 161]
        );
    }

    #[test]
    fn test_encode_v38() {
        let base64_url_config = URL_SAFE_NO_PAD;
        let is_public = false;
        let network = "twitter.com";
        let author_id = "yuan_brad";
        let iv = [
            8, 224, 216, 3, 117, 23, 198, 40, 218, 134, 149, 179, 52, 216, 88, 91,
        ];
        let aes_key_encoded = "MERv1-yBnsotcyzNG5zHv6WFlfIkGeosp2-UA1U_1Io";
        let aes_key = decode_config(&aes_key_encoded, base64_url_config).unwrap();
        let encrypted_message = [
            178, 39, 43, 146, 217, 20, 125, 160, 36, 78, 54, 45, 100, 113, 253, 43, 49, 165, 202,
            106, 185, 155,
        ];

        let author_pub_key_x_str = "LQ37fyhD6ug-2a9xmlez8bD3_eNTQnZ2O_8lRcWNSI4";
        let author_pub_key_y_str = "yoWbbZIyR-8dwLivurXT4fwD3QqP4sZ329jan3fp4I0";
        let author_pub_key_x_1 = decode_config(&author_pub_key_x_str, base64_url_config).unwrap();
        let author_pub_key_y_1 = decode_config(&author_pub_key_y_str, base64_url_config).unwrap();
        let author_public_key = [[0x04].to_vec(), author_pub_key_x_1, author_pub_key_y_1].concat();

        let target_pub_key_x_str = "j3RDjs8gfSBG2kpn5oX67e7CioZxRM1k1uyx7UzHpVU";
        let target_pub_key_y_str = "Q68JL9-pStMOzi3BlM8N8tAkiIY4PZqO7tvDk19sTm0";
        let target_pub_key_x_1 = decode_config(&target_pub_key_x_str, base64_url_config).unwrap();
        let target_pub_key_y_1 = decode_config(&target_pub_key_y_str, base64_url_config).unwrap();
        let target_public_key = [[0x04].to_vec(), target_pub_key_x_1, target_pub_key_y_1].concat();
        assert_eq!(target_public_key.len(), 65);

        let local_key_str = "JzGZnwVX9RKdkAKsrWmNMnzixUZA8I7vaaa2T_tEIT0";
        let local_key = decode_config(&local_key_str, base64_url_config).unwrap();
        let author_private_key_str = "xx96FEmD0_syCDgTu9vZW7doi8dFDwKe59P-a_N2jTg";

        let author_private_key_data =
            decode_config(&author_private_key_str, base64_url_config).unwrap();

        let sec_key = SecretKey::from_slice(&author_private_key_data).unwrap();
        let pub_key = PublicKey::from_slice(&target_public_key).unwrap();
        let shared_secret = ecdh::SharedSecret::new(&pub_key, &sec_key);
        println!("{:?}", &shared_secret);

        let mut target: HashMap<String, Vec<u8>> = HashMap::new();
        target.insert("author_id".to_string(), target_public_key);

        let (output, e2e_result) = encode_v38(
            is_public,
            network,
            Some(author_id),
            &iv,
            &aes_key,
            &encrypted_message,
            Some(&author_public_key),
            Some(&local_key),
            target,
            Some(&author_private_key_data),
        )
        .unwrap();
        println!("{:?}", e2e_result);
        assert_eq!(output, "🎼4/4|Bwpu5LcIkJkW2IWz1FJSXjso2l312ydbACk0owMXFXC2VUci0I7dK7smPEW/iAXU0v0b6pttFOdPsavNUJl+CSkjHaeKY4pBGdRPVLVX9wTFvha7233bTAh7H8MaOQKAcjMTTPSpiIfXV6z+adQ4ub/GBz13JEEcq1tBWGe14e6KJM0BAlavKA8W|CODYA3UXxijahpWzNNhYWw==|sicrktkUfaAkTjYtZHH9KzGlymq5mw==|_|Ay0N+38oQ+roPtmvcZpXs/Gw9/3jU0J2djv/JUXFjUiO|0|dHdpdHRlci5jb20veXVhbl9icmFk:||");
    }
}
