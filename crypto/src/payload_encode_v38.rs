use super::aes_gcm::aes_encrypt;
use super::post_encryption::Target;
use super::Error;
use bitcoin::secp256k1::PublicKey;

use base64::{decode_config, encode_config, STANDARD_NO_PAD, URL_SAFE_NO_PAD};

const SHARED_KEY_ENCODED: &str = "3Bf8BJ3ZPSMUM2jg2ThODeLuRRD_-_iwQEaeLdcQXpg";

enum Index {
    AuthorPublicKey = 5,
    PublicShared = 6,
    AuthorIdentifier = 7,
}

pub fn encode_v38(
    target: Target,
    network: Option<&str>,
    author_id: Option<&str>,
    iv: &[u8],
    key: &[u8],
    encrypted: &[u8],
    author_pub_key: Option<&[u8]>,
) -> Result<String, Error> {
    let aes_key_encrypted = encode_aes_key_encrypted(&target, iv, key)?;

    let base64_config = STANDARD_NO_PAD;
    let encoded_iv = encode_config(&iv, base64_config);
    let encoded_encrypted = encode_config(&encrypted, base64_config);
    let signature = "_";
    let encoded_fields = encode_fields(
        target,
        &aes_key_encrypted,
        &encoded_iv,
        &encoded_encrypted,
        signature,
        network,
        author_id,
        author_pub_key,
    )?;
    Ok(encoded_fields)
}

fn encode_aes_key_encrypted(target: &Target, iv: &[u8], key: &[u8]) -> Result<String, Error> {
    match target {
        Target::Public => {
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
            let base64_config = STANDARD_NO_PAD;
            let encoded_key = encode_config(&encrypted_key, base64_config);
            Ok(encoded_key)
        }
    }
}

fn encode_fields(
    target: Target,
    aes_key_encrypted: &str,
    encoded_iv: &str,
    encoded_encrypted: &str,
    signature: &str,
    network: Option<&str>,
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
            let base64_config = STANDARD_NO_PAD;
            let public_key =
                PublicKey::from_slice(key_data).map_err(|_| Error::InvalidPrivateKey)?;
            let compressed_key = public_key.serialize();
            encode_config(&compressed_key, base64_config)
        }
        None => "".to_string(),
    };
    fields[Index::AuthorPublicKey as usize] = &public_key_str;

    match target {
        Target::Public => {
            fields[Index::PublicShared as usize] = "1";
        }
    }

    let identity = match (network, author_id) {
        (Some(network), Some(author_id)) => {
            let profile_identifier = format!("{}/{}", network, author_id);
            let base64_config = STANDARD_NO_PAD;
            encode_config(&profile_identifier, base64_config)
        }
        _ => "".to_string(),
    };
    fields[Index::AuthorIdentifier as usize] = &identity;

    let joined_fields = fields.join("|");
    let result = format!("{}:||", joined_fields);
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_encode_aes_key() {
        let iv = [
            44, 67, 220, 0, 135, 88, 111, 139, 0, 72, 96, 128, 156, 163, 95, 183,
        ];
        let key_iv = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];

        let encoded_key = encode_aes_key_encrypted(&Target::Public, &iv, &key_iv).unwrap();
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
            Target::Public,
            aes_key_encrypted,
            encoded_iv,
            encoded_encrypted,
            signature,
            Some(network),
            Some(author_id),
            Some(&public_key_data),
        )
        .unwrap();
        assert_eq!(encoded_fields, "🎼4/4|8NXbnHHTwNaQlnihC4ov7JiAXIMfjmpP6LZG9SCpsBTGgscJuET25HO0DfkXOmjtepWV5NAGzRn5iFJENjTtIeMmnAaDl7ijSmsIfcS6Gp9wQZZ2yUaAj4S1rN6zCx6uZQNPaVH2kywLfQVZJ+pxNflXmKYgNcw53yG/XKgI7ksqCnwWqiqQyYYS|Q43qdWbDoDbWBca2+LB6lA==|h6WdGLrQ+H2fMJrXVFwKFw+IiQ==|_|AtJrd4w5tCX1flZPKYBrQGON3gZX+V+CxmMBcSlb75jU|1|dHdpdHRlci5jb20veXVhbl9icmFk:||");
    }
}
