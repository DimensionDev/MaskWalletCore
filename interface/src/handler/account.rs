use crate::coins::get_coin_info;
use crate::response_util::*;
use chain_common::api::mw_response::Response;
use chain_common::api::*;
use wallet::stored_key::StoredKey;

pub fn create_stored_key_account_of_coin_at_path(
    param: CreateStoredKeyNewAccountAtPathParam,
) -> MwResponse {
    let coin_info = get_coin_info(param.coin);
    let coin = match coin_info {
        Some(coin_info) => coin_info,
        None => {
            return MwResponse {
                response: Some(Response::Error(MwResponseError {
                    error_code: "-1".to_owned(),
                    error_msg: "Invalid Coin Type".to_owned(),
                })),
            };
        }
    };
    let stored_key: StoredKey = match serde_json::from_slice(&param.stored_key_data) {
        Ok(key) => key,
        Err(_) => {
            return get_json_error_response();
        }
    };
    let account = match stored_key.add_new_account_of_coin_and_derivation_path_by_password(
        &param.name,
        &coin,
        &param.derivation_path,
        &param.password,
    ) {
        Ok(account) => account,
        Err(error) => {
            return get_error_response_by_error(error);
        }
    };
    MwResponse {
        response: Some(Response::RespCreateAccountOfCoinAtPath(
            CreateStoredKeyNewAccountAtPathResp {
                account: Some(StoredKeyAccountInfo::from(&account)),
                stored_key: Some(StoredKeyInfo::from(stored_key)),
            },
        )),
    }
}
