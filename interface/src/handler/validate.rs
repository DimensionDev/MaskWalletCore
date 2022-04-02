use crate::coins::get_coin_info;
use crate::response_util::*;
use chain_common::api::mw_response::Response;
use chain_common::api::*;
use chain_common::private_key::PrivateKey;
use crypto::bip39::Mnemonic;
use crypto::key_store_json::KeyStoreJson;
use std::str::FromStr;
use wallet::coin_dispatcher::CoinDispatcher;
use wallet::stored_key::StoredKey;

pub fn validate(param: ValidateParam) -> MwResponse {
    let input_param = match param.input {
        Some(input) => input,
        None => return get_json_error_response(),
    };
    let valid = match input_param {
        validate_param::Input::Mnemonic(mnemonic) => Mnemonic::is_valid(&mnemonic),
        validate_param::Input::PrivateKey(private_key) => {
            PrivateKey::from_str(&private_key).is_ok()
        }
        validate_param::Input::KeyStoreJson(json) => KeyStoreJson::from_str(&json).is_ok(),
        validate_param::Input::StoredKeyPassword(password_param) => {
            let stored_key: StoredKey =
                match serde_json::from_slice(&password_param.stored_key_data) {
                    Ok(key) => key,
                    Err(_) => {
                        return get_json_error_response();
                    }
                };
            stored_key.validate_password(&password_param.password)
        }
        validate_param::Input::AddressValidationParam(addr_param) => {
            let coin_info = get_coin_info(addr_param.coin);
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
            let entry = match CoinDispatcher::get_entry(coin) {
                Ok(entry) => entry,
                Err(error) => {
                    return get_error_response_by_error(error);
                }
            };
            entry.validate_address(&addr_param.address)
        }
    };
    MwResponse {
        response: Some(Response::RespValidate(ValidateResp { valid })),
    }
}
