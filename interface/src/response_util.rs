use std::convert::From;
use super::api::{ MwResponse, MwResponseError };
use super::api::mw_response::Response;
use super::param::*;
use wallet::stored_key;
use wallet::account::Account;
use wallet::Error;

pub fn get_json_response_error() -> Option<Response> {
    Some(Response::Error(MwResponseError{
        error_code: "-1".to_owned(),
        error_msg: "Invalid Data".to_owned(),
    }))
}

pub fn get_json_error_response() -> MwResponse {
    MwResponse {
        response: get_json_response_error()
    }
}

pub fn get_invalid_proto_resposne() -> MwResponse {
    MwResponse {
        response: Some(Response::Error(MwResponseError{
            error_code: "-1".to_owned(),
            error_msg: "Invalid Input".to_owned(),
        }))
    }
}

fn get_error_response(_error: Error) -> Response {
    Response::Error(MwResponseError{
        error_code: "-1".to_owned(), // TODO: error to error code
        error_msg: "Invalid Data".to_owned(),  // TODO: error to error message
    })
}

pub fn get_error_response_by_error(error: Error) -> MwResponse {
    MwResponse {
        response: Some(get_error_response(error))
    }
}

// Begin of convinience function help converting wallet types to protobuf types
impl From<stored_key::StoredKeyType> for StoredKeyType {
    fn from(stored_key_type: stored_key::StoredKeyType) -> Self {
        match stored_key_type {
            stored_key::StoredKeyType::PrivateKey => StoredKeyType::PrivateKey,
            stored_key::StoredKeyType::Mnemonic => StoredKeyType::Hd,
        }
    }
}

impl From<stored_key::StoredKey> for StoredKeyInfo {
    fn from(stored_key: stored_key::StoredKey) -> Self {
        let json = serde_json::to_vec(&stored_key).unwrap().to_vec();
        StoredKeyInfo {
            data: json,
            id: stored_key.id,
            name: stored_key.name,
            r#type: StoredKeyType::from(stored_key.r#type) as i32
        }
    }
}

impl From<&Account> for StoredKeyAccountInfo {
    fn from(account: &Account) -> Self {
        StoredKeyAccountInfo {
            address: account.address.to_owned(),
            derivation_path: account.derivation_path.to_string(),
            coin: account.coin.id.to_owned(),
            extended_public_key: account.extended_public_key.to_owned()
        }
    }
}