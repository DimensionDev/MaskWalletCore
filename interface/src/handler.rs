use super::api::{ MwResponse, mw_request};
use super::api::mw_request::Request::*;
use super::param::*;
use super::coin::prost_coin_to_chain_coin;

use wallet::stored_key::*;

pub fn dispatch_request(request: mw_request::Request) -> MwResponse {
    match request {
        ParamImportPrivateKey(param) => {
            create_stored_key(param)
        }
    }
}

fn create_stored_key(param: PrivateKeyStoreImportParam) -> MwResponse {
    let coin = prost_coin_to_chain_coin(param.coin.unwrap());
    let stored_key = StoredKey::create_with_private_key_and_default_address(&param.name, &param.password, &param.private_key, coin);
    match stored_key {
        Ok(key) => {
            let json = serde_json::to_string(&key).unwrap();
            MwResponse {
                is_success: true,
                error_code: "".to_owned(),
                error_msg: "".to_owned(),
                data: json
            }
        },
        Err(error) => {
            MwResponse {
                is_success: true,
                error_code: "".to_owned(), //TODO: error to error code
                error_msg: "".to_owned(),  //TODO: error to error msg
                data: "".to_owned(),
            }
        }
    }
}