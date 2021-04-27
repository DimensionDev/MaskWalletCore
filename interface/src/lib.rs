use prost::Message;
use prost::EncodeError;

pub mod handler;
mod coins;
pub mod response_util;

#[macro_use]
extern crate lazy_static;

use bytes::BytesMut;
use chain_common::api::{ MwRequest, MwResponse, MwResponseError };
use chain_common::api::mw_response::Response;
use response_util::get_invalid_proto_resposne;

use handler::dispatch_request;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn request(input: &[u8]) -> Vec<u8> {
    call_api(input)
}

fn encode_message(msg: impl Message) -> Result<Vec<u8>, EncodeError> {
    let mut buf = BytesMut::with_capacity(msg.encoded_len());
    msg.encode(&mut buf)?;
    Ok(buf.to_vec())
}

pub fn call_api(input: &[u8]) -> Vec<u8> {
    let mw_request: MwRequest = match MwRequest::decode(input) {
        Ok(request) => request,
        Err(_) => {
            return encode_message(get_invalid_proto_resposne()).expect("invalid request");
        }
    };
    let response: MwResponse;
    
    if let Some(request) = mw_request.request {
        response = dispatch_request(request)
    } else {
        response = MwResponse {
            response: Some(Response::Error(MwResponseError{
                error_code: "-1".to_owned(),
                error_msg: "Invalid Coin Type".to_owned(),
            }))
        };
    }
    encode_message(response).expect("invalid request")
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_import_private_key() {
        assert_eq!(2 + 2, 4);
    }
}
