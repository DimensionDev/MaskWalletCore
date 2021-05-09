use chain_common::api::mw_response::Response;
use chain_common::api::*;
use std::env;

const VERSION: &str = include_str!(concat!(env!("OUT_DIR"), "/VERSION"));

pub fn get_lib_version() -> MwResponse {
    MwResponse {
        response: Some(Response::RespGetVersion(GetVersionResp {
            version: VERSION.to_owned(),
        })),
    }
}
