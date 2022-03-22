use prost::EncodeError;
use prost::Message;

mod coins;
pub mod handler;
pub mod response_util;

#[macro_use]
extern crate lazy_static;

use bytes::BytesMut;
use chain_common::api::{MwRequest, MwResponseError};
use response_util::get_invalid_proto_resposne;

use handler::dispatch_request;

fn encode_message<T>(msg: &T) -> Result<Vec<u8>, EncodeError>
where
    T: Message,
{
    let mut buf = BytesMut::with_capacity(msg.encoded_len());
    msg.encode(&mut buf)?;
    Ok(buf.to_vec())
}

pub fn call_api(input: &[u8]) -> Vec<u8> {
    let mw_request: MwRequest = match MwRequest::decode(input) {
        Ok(request) => request,
        Err(_) => {
            return encode_message(&get_invalid_proto_resposne()).expect("invalid request");
        }
    };
    let response = match mw_request.request {
        Some(request) => dispatch_request(request),
        None => MwResponseError {
            error_code: "-1".to_owned(),
            error_msg: "Empty Request".to_owned(),
        }
        .into(),
    };
    encode_message(&response).expect("invalid request")
}
