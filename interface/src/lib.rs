use prost::Message;
use prost::EncodeError;

pub mod api;
pub mod param;
pub mod handler;

use bytes::BytesMut;
use api::{ MwRequest, MwResponse};

use handler::dispatch_request;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn request(input: &[u8]) -> Vec<u8> {
    return call_api(input);
}

fn encode_message(msg: impl Message) -> Result<Vec<u8>, EncodeError> {
    let mut buf = BytesMut::with_capacity(msg.encoded_len());
    msg.encode(&mut buf)?;
    Ok(buf.to_vec())
}

pub fn call_api(input: &[u8]) -> Vec<u8> {
    let mw_request: MwRequest = MwRequest::decode(input).expect("decode api");

    let response: MwResponse;
    if let Some(request) = mw_request.request {
        response = dispatch_request(request)
    } else {
        response = MwResponse { 
            is_success: false, 
            error_code: "invalid request".to_owned(),
            error_msg: "-1".to_owned(),
            data: "".to_owned(),
        };
    }
    let encoded_result = encode_message(response).expect("");
    return encoded_result;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_import_private_key() {
        assert_eq!(2 + 2, 4);
    }
}
