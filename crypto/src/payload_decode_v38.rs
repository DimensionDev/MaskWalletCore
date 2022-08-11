use base64::{decode_config, STANDARD, URL_SAFE_NO_PAD};
use serde::{Deserialize, Serialize};

use crate::aes_gcm::aes_decrypt;
use crate::encryption_constants::{IV_SIZE, SHARED_KEY_ENCODED};
use crate::payload_encode_v38::Index;

#[allow(dead_code)]
#[derive(Debug)]
pub enum ParseError {
    Fields,
    AuthorPubKey,
    Identity,
    PostIdentifier,
    AesKey,
    PostKey,
    PostMessage,
    LocalKeyIsNil,
    IvIsInvalid,
}

#[allow(dead_code)]
#[derive(Debug)]
struct EncrtyptionParam {
    is_public: bool,
    aes_key_encrypted: String,
    encoded_iv: String,
    encoded_encrypted: String,
    signature: String,
    network: Option<String>,
    author_id: Option<String>,
    author_serialized_pub_key: Option<Vec<u8>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct JWKFormatObject {
    alg: String,
    ext: bool,
    k: String,
    key_ops: Vec<String>,
    kty: String,
}

pub fn decode_payload_v38(
    post_identifier: Option<&str>,
    local_key_data: Option<&[u8]>,
    post_content: &str,
) -> Result<String, ParseError> {
    let parsed_enctypted_info = parse_and_decode_fields_to_encrypt_info(post_content)?;
    let encrypted_text = decode_config(&parsed_enctypted_info.encoded_encrypted, STANDARD)
        .map_err(|_| ParseError::Fields)?;

    let decoded_iv = decode_config(parsed_enctypted_info.encoded_iv, STANDARD)
        .map_err(|_| ParseError::Fields)?;
    if decoded_iv.len() != IV_SIZE {
        return Err(ParseError::IvIsInvalid);
    }

    let network_and_post_iv = parse_post_identifier(post_identifier)?;
    match network_and_post_iv {
        None => {}
        Some((_, post_iv_from_identifier)) => {
            if decoded_iv != post_iv_from_identifier {
                return Err(ParseError::Fields);
            }
        }
    }

    // use decoded_iv and post_key to decrypt text content
    let post_key = match parsed_enctypted_info.is_public {
        false => decode_aes_key_encrypted_with_local_key(
            &parsed_enctypted_info.aes_key_encrypted,
            local_key_data,
            &decoded_iv,
        )?,
        true => decode_aes_key_encrypted(&decoded_iv, &parsed_enctypted_info.aes_key_encrypted)
            .map_err(|_| ParseError::PostKey)?,
    };

    let base64_encrypted_text =
        String::from_utf8(encrypted_text).map_err(|_| ParseError::Fields)?;
    let encrypted_text_buf =
        decode_config(&base64_encrypted_text, STANDARD).map_err(|_| ParseError::Fields)?;

    let decoded_message = aes_decrypt(&decoded_iv, &post_key, &encrypted_text_buf)
        .map_err(|_| ParseError::PostMessage)?;

    String::from_utf8(decoded_message).map_err(|_| ParseError::PostMessage)
}

fn decode_aes_key_encrypted(iv: &[u8], encrypted_ase_key: &str) -> Result<Vec<u8>, ParseError> {
    let base64_url_config = URL_SAFE_NO_PAD;
    let base64_config = STANDARD;

    let base64_decoded_ase_key =
        decode_config(encrypted_ase_key, base64_config).map_err(|_| ParseError::AesKey)?;
    let shared_key_bytes =
        decode_config(&SHARED_KEY_ENCODED, base64_url_config).map_err(|_| ParseError::AesKey)?;
    let decrypted_key = aes_decrypt(iv, &shared_key_bytes, &base64_decoded_ase_key)
        .map_err(|_| ParseError::AesKey)?;

    let ab = serde_json::from_slice::<JWKFormatObject>(&decrypted_key);
    match ab {
        Ok(decrypted_object) => {
            let encoded_aes_key = decrypted_object.k;
            let decoded_aes_key = decode_config(encoded_aes_key, base64_url_config)
                .map_err(|_| ParseError::PostKey)?;
            Ok(decoded_aes_key)
        }
        Err(_) => Err(ParseError::PostKey),
    }
}

fn decode_aes_key_encrypted_with_local_key(
    aes_key_encrypted: &str,
    local_key_data: Option<&[u8]>,
    iv: &[u8],
) -> Result<Vec<u8>, ParseError> {
    let owners_aes_key_decoded =
        decode_config(aes_key_encrypted, STANDARD).map_err(|_| ParseError::PostKey)?;
    let local_key_data = local_key_data.ok_or(ParseError::LocalKeyIsNil)?;
    let encoded_post_key = aes_decrypt(iv, local_key_data, &owners_aes_key_decoded)
        .map_err(|_| ParseError::PostKey)?;
    let jwk_object = serde_json::from_slice::<JWKFormatObject>(&encoded_post_key)
        .map_err(|_| ParseError::PostKey)?;

    decode_config(jwk_object.k, URL_SAFE_NO_PAD).map_err(|_| ParseError::PostKey)
}

fn parse_post_identifier(formatted: Option<&str>) -> Result<Option<(String, Vec<u8>)>, ParseError> {
    let prefix = "post_iv:";

    match formatted {
        None => Ok(None),
        Some(formatted) => match formatted.starts_with(prefix) {
            false => Err(ParseError::PostIdentifier),
            true => {
                let mut network_and_encode_iv = formatted
                    .split(prefix)
                    .find(|x| x.contains('/'))
                    .map(|x| x.split('/'))
                    .ok_or(ParseError::Fields)?;

                let network = network_and_encode_iv
                    .next()
                    .ok_or(ParseError::PostIdentifier)?;
                let post_iv = network_and_encode_iv
                    .next()
                    .ok_or(ParseError::PostIdentifier)?
                    .replace("|", "/");

                let post_iv =
                    decode_config(&post_iv, STANDARD).map_err(|_| ParseError::PostIdentifier)?;

                Ok(Option::Some((network.to_owned(), post_iv)))
            }
        },
    }
}

fn parse_post_fields(given_content: &str) -> Result<Vec<&str>, ParseError> {
    let prefix = "\u{1F3BC}4/4";
    if !given_content.starts_with(prefix) {
        return Err(ParseError::Fields);
    }

    if !given_content.ends_with(":||") {
        return Err(ParseError::Fields);
    }

    let compace_fields = given_content.split(":||").next().unwrap_or("");
    if compace_fields.is_empty() {
        return Err(ParseError::Fields);
    }

    let fields = compace_fields.split('|').collect::<Vec<&str>>();
    if fields.len() as usize != 8 {
        return Err(ParseError::Fields);
    }

    let is_public = fields[Index::PublicShared as usize];
    if !["0", "1"].contains(&is_public) {
        return Err(ParseError::Fields);
    }

    Ok(fields)
}

fn parse_and_decode_fields_to_encrypt_info(
    given_content: &str,
) -> Result<EncrtyptionParam, ParseError> {
    let fields = parse_post_fields(given_content)?;

    let aes_key_encrypted = fields[1].to_owned();

    let identity = fields[Index::AuthorIdentifier as usize];

    let (network, author_id) = match identity.is_empty() {
        true => (Option::None, Option::None),
        false => {
            let decoded_identity =
                decode_config(identity, STANDARD).map_err(|_| ParseError::Identity)?;
            let string = String::from_utf8(decoded_identity).map_err(|_| ParseError::Identity)?;
            let mut splits = string.split('/');

            let network = splits.next();
            let author_id = splits.next();

            (
                network.map(|x| x.to_owned()),
                author_id.map(|x| x.to_owned()),
            )
        }
    };

    let pub_key = fields[Index::AuthorPublicKey as usize];
    let pub_key_serialized = match pub_key.is_empty() {
        true => Option::None,
        false => {
            let key = decode_config(pub_key, STANDARD).map_err(|_| ParseError::AuthorPubKey)?;
            Option::Some(key)
        }
    };

    let is_public = fields[Index::PublicShared as usize];

    Ok(EncrtyptionParam {
        is_public: is_public == "1",
        aes_key_encrypted,
        encoded_iv: fields[2].to_owned(),
        encoded_encrypted: fields[3].to_owned(),
        signature: fields[4].to_owned(),
        network,
        author_id,
        author_serialized_pub_key: pub_key_serialized,
    })
}

#[cfg(test)]
mod tests {
    use base64::encode_config;

    use super::*;

    use crate::number_util::random_iv;
    use crate::payload_encode_v38::{encode_aes_key_encrypted, eocode_post_key_and_aes_key};
    use crate::encryption_constants::AES_KEY_SIZE;

    const PUB_KEY_SIZE: usize = 33;

    #[test]
    fn test_get_post_identifier_and_post_iv() {
        let post_iv = random_iv(IV_SIZE);
        let iv_str = encode_config(&post_iv, STANDARD);
        let post_identifier = format!("post_iv:eth/{}", iv_str);
        let decoded = parse_post_identifier(Some(&post_identifier))
            .expect("Invalid post identifier")
            .unwrap();
        let (network, iv) = decoded;
        assert_eq!(network, "eth");
        assert_eq!(&iv, &post_iv)
    }

    #[test]
    fn test_decode_aes_key_encrypted_public() {
        let iv = random_iv(IV_SIZE);
        let key = random_iv(AES_KEY_SIZE);
        let encoded_aes_key = encode_aes_key_encrypted(&iv, &key).unwrap();

        let decoded_key = decode_aes_key_encrypted(&iv, &encoded_aes_key);
        // let ad = Vec::from_iter(key.into_iter());
        assert!(decoded_key.is_ok());
        assert_eq!(decoded_key.unwrap(), key);
    }

    #[test]
    fn test_decode_aes_key_encrypted() {
        let iv = random_iv(IV_SIZE);
        let key = random_iv(AES_KEY_SIZE);
        let local_key_data = random_iv(AES_KEY_SIZE);
        let (_, owners_aes_key_encrypted_string) =
            eocode_post_key_and_aes_key(Some(&local_key_data), &iv, &key).unwrap();

        let result = decode_aes_key_encrypted_with_local_key(
            &owners_aes_key_encrypted_string,
            Some(&local_key_data),
            &iv,
        );

        match result {
            Err(err) => panic!("decoding failed {err:?}"),
            Ok(result) => {
                assert_eq!(result, key);
            }
        }
    }

    #[test]
    fn test_decode_fields() {
        let public_fields =
            parse_post_fields("\u{1F3BC}4/4|13add|1333dad|1318dadss_dad|juudad|12adad|1|dad:||");
        let privte_fields =
            parse_post_fields("\u{1F3BC}4/4|13add|1333dad|1318dadss_dad|juudad|01131|0|dad:||");
        let faild_public_fields =
            parse_post_fields("11dd|13add|1333dad|1318dadss_dad|juudad|1qe1dda|1q|dad:||");
        let faild_fields =
            parse_post_fields("\u{1F3BC}4/4|13add|1333dad|1318dadss_dad|juudad|1qe1dda|1q|dad:||");

        assert!(public_fields.is_ok());
        assert!(privte_fields.is_ok());
        assert!(faild_public_fields.is_err());
        assert!(faild_fields.is_err());
    }

    #[test]
    fn test_decode_base64_encoded_encryped() {
        let base64_url_config = URL_SAFE_NO_PAD;
        let encrypted_result = "🎼4/4|dPsavNUJl+CSkjHaeKY4pBGdRPVLVX9wTFvha7233bTAh7H8MaOQKAcjMTTPSpiIfXV6z+adQ4ub/GBz13JEEcq1tBWGe14e6KJM0BAlavKA8W|CODYA3UXxijahpWzNNhYWw==|sicrktkUfaAkTjYtZHH9KzGlymq5mw==|_|Ay0N+38oQ+roPtmvcZpXs/Gw9/3jU0J2djv/JUXFjUiO|0|dHdpdHRlci5jb20veXVhbl9icmFk:||";
        let encryped_info = parse_and_decode_fields_to_encrypt_info(encrypted_result)
            .expect("parse_fields_to_encrypt_info failed");

        let network = "twitter.com";
        let author_id = "yuan_brad";
        let is_public = false;
        let iv = [
            8, 224, 216, 3, 117, 23, 198, 40, 218, 134, 149, 179, 52, 216, 88, 91,
        ];

        assert_eq!(encryped_info.is_public, is_public);
        assert_eq!(encryped_info.author_id.as_ref().unwrap(), author_id);
        assert_eq!(encryped_info.network.as_deref().unwrap(), network);
        assert_eq!(encryped_info.signature, "_");

        let decoded_iv =
            decode_config(&encryped_info.encoded_iv, STANDARD).expect("iv decode failed");
        assert_eq!(decoded_iv, iv);

        let author_pub_key_x_str = "LQ37fyhD6ug-2a9xmlez8bD3_eNTQnZ2O_8lRcWNSI4";
        let author_pub_key_y_str = "yoWbbZIyR-8dwLivurXT4fwD3QqP4sZ329jan3fp4I0";
        let author_pub_key_x_1 = decode_config(&author_pub_key_x_str, base64_url_config).unwrap();
        let author_pub_key_y_1 = decode_config(&author_pub_key_y_str, base64_url_config).unwrap();
        let author_public_key = [[0x04].to_vec(), author_pub_key_x_1, author_pub_key_y_1].concat();

        // skip flag u8
        assert_eq!(
            encryped_info.author_serialized_pub_key.unwrap()[1..],
            author_public_key[1..PUB_KEY_SIZE]
        );

        let encrypted_message = [
            178, 39, 43, 146, 217, 20, 125, 160, 36, 78, 54, 45, 100, 113, 253, 43, 49, 165, 202,
            106, 185, 155,
        ];

        assert_eq!(
            decode_config(encryped_info.encoded_encrypted, STANDARD)
                .expect("decode encrypted_message failed"),
            encrypted_message
        );

        assert_eq!(encryped_info.aes_key_encrypted, "dPsavNUJl+CSkjHaeKY4pBGdRPVLVX9wTFvha7233bTAh7H8MaOQKAcjMTTPSpiIfXV6z+adQ4ub/GBz13JEEcq1tBWGe14e6KJM0BAlavKA8W");
    }

    #[test]
    fn test_decode_proccess() {
        let ownersAESKeyEncrypted: [u8; 138] = [
            144, 174, 122, 212, 72, 176, 255, 251, 188, 174, 28, 233, 204, 151, 80, 93, 36, 109,
            165, 126, 119, 100, 64, 93, 55, 94, 37, 149, 89, 113, 222, 62, 245, 197, 232, 114, 255,
            100, 89, 118, 11, 128, 59, 223, 17, 148, 137, 1, 215, 188, 44, 12, 211, 86, 246, 197,
            59, 183, 149, 125, 172, 106, 225, 119, 222, 221, 214, 135, 53, 104, 255, 122, 113, 71,
            62, 77, 164, 13, 18, 104, 245, 100, 241, 139, 50, 193, 151, 200, 82, 143, 94, 68, 56,
            246, 42, 75, 223, 44, 208, 251, 9, 195, 249, 131, 84, 175, 78, 237, 23, 26, 34, 33,
            115, 248, 238, 80, 144, 64, 120, 103, 23, 251, 177, 78, 222, 231, 250, 67, 214, 25,
            214, 99, 187, 76, 2, 197, 220, 30,
        ];

        let content_text = "🎼4/4|kK561Eiw//u8rhzpzJdQXSRtpX53ZEBdN14llVlx3j71xehy/2RZdguAO98RlIkB17wsDNNW9sU7t5V9rGrhd97d1oc1aP96cUc+TaQNEmj1ZPGLMsGXyFKPXkQ49ipL3yzQ+wnD+YNUr07tFxoiIXP47lCQQHhnF/uxTt7n+kPWGdZju0wCxdwe|tAnANz4j+aOOGkSAMTXbrA==|F8rREysE4XdmNXqvD5GKrbHA2k9IOR9s8dNW2QAA2MPOVw==|_|Asbei5hqEcr1HoHONBdPQugbBvw/EYZej6O+IKZeF8m2|0|dHdpdHRlci5jb20vZm94X3dlMTA=:||";
        let fields = parse_and_decode_fields_to_encrypt_info(content_text).unwrap();
        println!("{:?}", fields);
    }
}
