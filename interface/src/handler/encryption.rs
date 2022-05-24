use crypto::post_encryption::{encrypt, Target, Version};

use chain_common::api::{mw_response::Response, MwResponse, PostEncrypedResp, PostEncryptionParam};

pub fn encode(param: PostEncryptionParam) -> MwResponse {
    let version = match param.version {
        0 => Version::V37,
        1 => Version::V38,
        _ => Version::V38,
    };
    let algr = param.author_public_key_algr.map(|f| f as u8);
    let result = encrypt(
        version,
        Target::Public,
        param.network.as_deref(),
        param.author_user_id.as_deref(),
        algr,
        param.author_public_key_data.as_deref(),
        param.content.as_bytes(),
    );

    match result {
        Ok(encrypted_message) => {
            // TODO: finish implementation
            let content = PostEncrypedResp {
                content: encrypted_message,
            };
            Response::RespPostEncryption(content).into()
        }

        Err(err) => err.into(),
    }
}
