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
        &param.network,
        param.author_user_id.as_deref(),
        algr,
        param.author_public_key_data.as_deref(),
        param.content.as_bytes(),
        local_key.as_deref(),
        target,
        author_private_key.as_deref(),
    );

    match result {
        Ok(encryption_result) => {
            // TODO: finish implementation

            let encryption_results = encryption_result
                .e2e_result
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
                .collect();

            let content = PostEncryptedResp {
                content: encryption_result.output,
                results: encryption_results,
                post_identifier: encryption_result.post_identifier,
                post_key: encryption_result.post_key,
            };

            Response::RespPostEncryption(content).into()
        }

        Err(err) => err.into(),
    }
}
