use super::api::{ MwResponse, mw_request, MwResponseError };
use super::api::mw_request::Request::*;
use super::api::mw_response::Response;
use super::param::*;
use super::coin::get_coin_info;
use super::response_util::{ get_json_error_response, get_error_response_by_error };

use wallet::stored_key::*;

pub fn dispatch_request(request: mw_request::Request) -> MwResponse {
    match request {
        ParamImportPrivateKey(param) => {
            create_stored_key(param)
        },
        ParamGetStoredKeyAccountCount(param) => {
            get_stored_key_account_count(param)
        },
        ParamGetStoredKeyAccount(param) => {
            get_store_key_account(param)
        },
    }
}

fn create_stored_key(param: PrivateKeyStoreImportParam) -> MwResponse {
    let coin_info = get_coin_info(param.coin);
    let coin = match coin_info {
        Some(coin_info) => coin_info,
        None => {
            return MwResponse {
                is_success: false,
                response: Some(Response::Error(MwResponseError{
                    error_code: "-1".to_owned(),
                    error_msg: "Invalid Coin Type".to_owned(),
                }))
            };
        }
    };
    
    let stored_key = StoredKey::create_with_private_key_and_default_address(&param.name, &param.password, &param.private_key, coin.clone());
    match stored_key {
        Ok(key) => {
            let json = serde_json::to_string(&key).unwrap();
            MwResponse {
                is_success: true,
                response: Some(Response::RespImportPrivateKey(
                    PrivateKeyStoreImportResp {
                        data: json,
                    }
                ))
            }
        },
        Err(error) => {
            MwResponse {
                is_success: false,
                response: Some(Response::Error(MwResponseError{
                    error_code: "-1".to_owned(),  // TODO: error to error code
                    error_msg: "Invalid Coin Type".to_owned(),  // TODO: error to error msg
                }))
            }
        }
    }
}

fn get_stored_key_account_count(param: GetStoredKeyAccountCountParam) -> MwResponse {
    let stored_key: StoredKey = match serde_json::from_str(&param.data) {
        Ok(key) => key,
        Err(_) => {
            return get_json_error_response();
        }
    };
    MwResponse {
        is_success: true,
        response: Some(Response::RespGetStoredKeyAccountCount(
            GetStoredKeyAccountCountResp {
                count: stored_key.get_accounts_count(),
            }
        ))
    }
}
fn get_store_key_account(param: GetStoredKeyAccountParam) -> MwResponse {
    let stored_key: StoredKey = match serde_json::from_str(&param.data) {
        Ok(key) => key,
        Err(_) => {
            return get_json_error_response();
        }
    };
    let account = match stored_key.get_account(param.index) {
        Ok(account) => account,
        Err(error) => {
            return get_error_response_by_error(error);
        }
    };
    MwResponse {
        is_success: true,
        response: Some(Response::RespGetStoredKeyAccount(
            GetStoredKeyAccountResp {
                address: account.address.clone(),
                derivation_path: account.derivation_path.to_string(),
                coin: account.coin.id.clone(),
                extended_public_key: account.extended_public_key.clone(),
            }
        ))
    }
}