use crypto::post_encryption::encrypt;

use chain_common::api::{mw_response::Response, MwResponse, PostEncrypedResp, PostEncryptionParam};

pub fn encode(param: PostEncryptionParam) -> MwResponse {
    let result = encrypt(param.content, param.author_key, param.network);

    match result {
        Ok(encrypted_message) => {
            // TODO: finish implementation
            let content = PostEncrypedResp {
                content: "".to_string(),
            };
            let resp = Response::RespEncrypedPost(content);
            let _ = encrypted_message.cipher;
            MwResponse {
                response: Some(resp),
            }
        }

        Err(err) => err.into(),
    }
}
