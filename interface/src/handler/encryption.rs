use crypto::post_encryption::{encrypt, Version};
use std::collections::HashMap;

use chain_common::api::{
    mw_response::Response, E2eEncryptionResult, MwResponse, PostEncrypedResp, PostEncryptionParam,
};

pub fn encode(param: PostEncryptionParam) -> MwResponse {
    let version = match param.version {
        0 => Version::V37,
        1 => Version::V38,
        _ => Version::V38,
    };
    let algr = param.author_public_key_algr.map(|f| f as u8);
    let result = encrypt(
        version,
        param.is_plublic,
        param.network.as_deref(),
        param.author_user_id.as_deref(),
        algr,
        param.author_public_key_data.as_deref(),
        param.content.as_bytes(),
        param.local_key_data.as_deref(),
        param.target,
        param
            .author_private_key
            .map(|jwk| jwk.d)
            .flatten()
            .as_deref(),
    );

    match result {
        Ok((encrypted_message, ecdh_result)) => {
            // TODO: finish implementation
            let content = PostEncrypedResp {
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
