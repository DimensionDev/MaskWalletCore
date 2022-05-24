use crypto::post_encryption::encrypt;

use chain_common::api::{mw_response::Response, MwResponse, PostEncrypedResp, PostEncryptionParam};

pub fn encode(param: PostEncryptionParam) -> MwResponse {
    let result = encrypt(
        "",
        param.content.as_str(),
        0,
        param.network.as_bytes(),
        param.content.as_bytes(),
    );

    match result {
        Ok(encrypted_message) => {
            // TODO: finish implementation
            let content = PostEncrypedResp {
                content: "".to_string(),
            };
            let _ = encrypted_message;

            Response::RespPostEncryption(content).into()
        }

        Err(err) => err.into(),
    }
}
