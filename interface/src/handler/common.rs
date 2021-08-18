use crate::response_util::*;
use chain_common::api::mw_response::Response;
use chain_common::api::*;
use crypto::bip39::Mnemonic;
use std::env;

const VERSION: &str = include_str!(concat!(env!("OUT_DIR"), "/VERSION"));

pub fn get_lib_version() -> MwResponse {
    MwResponse {
        response: Some(Response::RespGetVersion(GetVersionResp {
            version: VERSION.to_owned(),
        })),
    }
}

pub fn generate_mnemonic() -> MwResponse {
    let mnemonic = match Mnemonic::generate_mnemonic_string(12) {
        Ok(mnemonic) => mnemonic,
        Err(_) => {
            return get_json_error_response();
        }
    };
    MwResponse {
        response: Some(Response::RespGenerateMnemonic(GenerateMnemonicResp {
            mnemonic,
        })),
    }
}
