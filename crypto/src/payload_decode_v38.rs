use base64::{decode_config, STANDARD};

use super::payload_encode_v38::Index;

#[derive(Debug)]
enum ParseError {
    InvalidField,
    InvalidAuthorPubKey,
    InvalidIdentity,
    InvalidPostIdentifier,
}

struct EncrtyptionParam {
    is_public: bool,
    aes_key_encrypted: String,
    encoded_iv: String,
    encoded_encrypted: String,
    signature: String,
    network: Option<String>,
    author_id: Option<String>,
    author_pub_key: Option<Vec<u8>>,
}

fn decode_post_e2e_v38(
    post_identifier: &str,
    local_key_data: Option<&[u8]>,
    author_private_key: Option<&[u8]>,
    post_content: &str,
) -> Result<(), ParseError> {
    let parsed_fields = parse_fields(post_content)?;
    let parsed_enctypted_info = parse_and_decode_fields_to_encrypt_info(parsed_fields)?;

    Ok(())
}

fn parse_post_identifier(formated: &str) -> Result<(String, String), ParseError> {
    let prefix = "post_iv:";

    match formated.starts_with(prefix) {
        false => Err(ParseError::InvalidPostIdentifier),
        true => {
            let mut network_and_encode_iv = formated
                .split(prefix)
                .find(|x| x.contains('/'))
                .map(|x| x.split('/'))
                .ok_or(ParseError::InvalidField)?;

            let network = network_and_encode_iv
                .next()
                .ok_or(ParseError::InvalidPostIdentifier)?;
            let encode_iv = network_and_encode_iv
                .next()
                .ok_or(ParseError::InvalidPostIdentifier)?;

            Ok((network.to_owned(), encode_iv.to_owned()))
        }
    }
}

fn parse_fields(given_content: &str) -> Result<Vec<&str>, ParseError> {
    let prefix = "\u{1F3BC}4/4";
    if !given_content.starts_with(prefix) {
        return Err(ParseError::InvalidField);
    }

    if !given_content.ends_with(":||") {
        return Err(ParseError::InvalidField);
    }

    let compace_fields = given_content.split(":||").next().unwrap_or("");
    if compace_fields.is_empty() {
        return Err(ParseError::InvalidField);
    }

    let fields = compace_fields.split('|').collect::<Vec<&str>>();
    if fields.len() != 8 {
        return Err(ParseError::InvalidField);
    }

    Ok(fields)
}

fn parse_and_decode_fields_to_encrypt_info(
    fields: Vec<&str>,
) -> Result<EncrtyptionParam, ParseError> {
    if fields[0] != "\u{1F3BC}4/4" {
        return Err(ParseError::InvalidField);
    }

    let is_public = fields[Index::PublicShared as usize];
    if !["0", "1"].contains(&is_public) {
        return Err(ParseError::InvalidField);
    }

    let aes_key_encrypted = fields[1].to_owned();

    let identity = fields[Index::AuthorIdentifier as usize];

    let (network, author_id) = match identity.is_empty() {
        true => (Option::None, Option::None),
        false => {
            let decoded_identity =
                decode_config(identity, STANDARD).map_err(|_| ParseError::InvalidIdentity)?;
            let string =
                String::from_utf8(decoded_identity).map_err(|_| ParseError::InvalidIdentity)?;
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
            let key =
                decode_config(pub_key, STANDARD).map_err(|_| ParseError::InvalidAuthorPubKey)?;
            Option::Some(key)
        }
    };

    Ok(EncrtyptionParam {
        is_public: is_public == "1",
        aes_key_encrypted,
        encoded_iv: fields[2].to_owned(),
        encoded_encrypted: fields[3].to_owned(),
        signature: fields[4].to_owned(),
        network,
        author_id,
        author_pub_key: pub_key_serialized,
    })
}

#[cfg(test)]
mod tests {
    use super::super::Error;
    use super::*;
    use crate::payload_encode_v38::*;
    use base64::URL_SAFE_NO_PAD;

    // use

    const IV_SIZE: usize = 16;
    const AES_KEY_SIZE: usize = 32;

    const PUB_KEY_SIZE: usize = 33;

    #[test]
    fn test_decode_v37() -> Result<(), Error> {
        todo!()
    }

    #[test]
    fn test_decode_post_iv() {
        todo!()
    }

    #[test]
    fn test_get_post_identifier_and_post_iv() {
        let post_identifier = "post_iv:eth/randomiv";
        let (network, iv) =
            parse_post_identifier(post_identifier).expect("Invalid post identifier");
        assert_eq!(network, "eth");
        assert_eq!(iv, "randomiv");
    }
    #[test]
    fn test_get_encrypted_message() {
        todo!()
    }

    #[test]
    fn test_decode_fields() {
        let public_fields =
            parse_fields("\u{1F3BC}4/4|13add|1333dad|1318dadss_dad|juudad|12adad|1|dad:||");
        let privte_fields =
            parse_fields("\u{1F3BC}4/4|13add|1333dad|1318dadss_dad|juudad|01131|0|dad:||");
        let faild_public_fields =
            parse_fields("11dd|13add|1333dad|1318dadss_dad|juudad|1qe1dda|1q|dad:||");
        let faild_fields =
            parse_fields("\u{1F3BC}4/4|13add|1333dad|1318dadss_dad|juudad|1qe1dda|1q|dad:||");

        assert!(public_fields.is_ok());
        assert!(privte_fields.is_ok());
        assert!(faild_public_fields.is_err());
        assert!(faild_fields.is_ok());
    }

    #[test]
    fn test_base64_decode_encoded_encryped() {
        let base64_url_config = URL_SAFE_NO_PAD;
        let encrypted_result = "🎼4/4|dPsavNUJl+CSkjHaeKY4pBGdRPVLVX9wTFvha7233bTAh7H8MaOQKAcjMTTPSpiIfXV6z+adQ4ub/GBz13JEEcq1tBWGe14e6KJM0BAlavKA8W|CODYA3UXxijahpWzNNhYWw==|sicrktkUfaAkTjYtZHH9KzGlymq5mw==|_|Ay0N+38oQ+roPtmvcZpXs/Gw9/3jU0J2djv/JUXFjUiO|0|dHdpdHRlci5jb20veXVhbl9icmFk:||";
        let parsed_field = parse_fields(encrypted_result).expect("parse failed");
        let encryped_info = parse_and_decode_fields_to_encrypt_info(parsed_field)
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
            encryped_info.author_pub_key.unwrap()[1..],
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
}
