use std::collections::HashMap;

use crypto::post_encryption::{encrypt, Version};

use chain_common::api::{
    mw_response::Response, E2eEncryptionResult, MwResponse, PostEncryptedResp, PostEncryptionParam,
};

pub fn encode(param: PostEncryptionParam) -> MwResponse {
    let version = match param.version {
        0 => Version::V37,
        1 => Version::V38,
        _ => Version::V38,
    };
    let algr = param.author_public_key_algr.map(|f| f as u8);
    let local_key = param.param.as_ref().map(|x| x.local_key_data.clone());
    let author_private_key = param.param.as_ref().map(|x| x.author_private_key.clone());
    let target = param.param.map_or(HashMap::new(), |x| x.target);
    let result = encrypt(
        version,
        param.is_plublic,
        param.network.as_deref(),
        param.author_user_id.as_deref(),
        algr,
        param.author_public_key_data.as_deref(),
        param.content.as_bytes(),
        local_key.as_deref(),
        target,
        author_private_key.as_deref(),
    );

    match result {
        Ok((encrypted_message, ecdh_result)) => {
            // TODO: finish implementation
            let content = PostEncryptedResp {
                content: encrypted_message,
                results: ecdh_result
                    .unwrap_or_default()
                    .into_iter()
                    .map(|(k, v)| {
                        (
                            k,
                            E2eEncryptionResult {
                                iv: v.iv_to_be_published,
                                encrypted_post_key_data: v.encrypted_post_key,
                                ephemeral_public_key_data: None,
                            },
                        )
                    })
                    .collect(),
            };
            Response::RespPostEncryption(content).into()
        }

        Err(err) => err.into(),
    }
}
