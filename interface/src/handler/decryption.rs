use crypto::{payload_decode_v38::decode_payload_v38, Error};

use chain_common::api::{
    mw_response::Response, MwResponse, PostDecryptionParam, PostDecryptionResp,
};

pub fn decode_post(param: PostDecryptionParam) -> MwResponse {
    let post_identifier = param.post_identifier.as_deref();
    let local_key = param.local_key_data;

    let decoded_result =
        decode_payload_v38(post_identifier, local_key.as_deref(), &param.post_content);

    match decoded_result {
        Err(_) => Error::DecryptContentFailed.into(),
        Ok(decoded_text) => Response::RespPostDecryption(PostDecryptionResp {
            content_text: decoded_text,
        })
        .into(),
    }
}
