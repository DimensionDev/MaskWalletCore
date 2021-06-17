use crate::coins::get_coin_info;
use crate::response_util::*;
use chain_common::api::mw_response::Response;
use chain_common::api::*;
use wallet::stored_key::StoredKey;

pub fn get_stored_key_account_count(param: GetStoredKeyAccountCountParam) -> MwResponse {
    let stored_key: StoredKey = match serde_json::from_slice(&param.stored_key_data) {
        Ok(key) => key,
        Err(_) => {
            return get_json_error_response();
        }
    };
    MwResponse {
        response: Some(Response::RespGetStoredKeyAccountCount(
            GetStoredKeyAccountCountResp {
                count: stored_key.get_accounts_count(),
            },
        )),
    }
}

pub fn get_store_key_account(param: GetStoredKeyAccountParam) -> MwResponse {
    let stored_key: StoredKey = match serde_json::from_slice(&param.stored_key_data) {
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
        response: Some(Response::RespGetStoredKeyAccount(GetStoredKeyAccountResp {
            account: Some(StoredKeyAccountInfo::from(account)),
        })),
    }
}

pub fn get_stored_key_all_accounts(param: GetStoredKeyAllAccountParam) -> MwResponse {
    let stored_key: StoredKey = match serde_json::from_slice(&param.stored_key_data) {
        Ok(key) => key,
        Err(_) => {
            return get_json_error_response();
        }
    };
    let accounts_info: Vec<StoredKeyAccountInfo> = stored_key
        .get_all_accounts()
        .iter()
        .map(StoredKeyAccountInfo::from)
        .collect();
    MwResponse {
        response: Some(Response::RespGetStoredKeyAllAccounts(
            GetStoredKeyAllAccountResp {
                accounts: accounts_info,
            },
        )),
    }
}

pub fn get_stored_key_accounts_of_coin(param: GetStoredKeyAccountsOfCoinParam) -> MwResponse {
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

    let accounts: Vec<StoredKeyAccountInfo> = stored_key
        .get_accounts_of_coin(coin)
        .iter()
        .map(StoredKeyAccountInfo::from)
        .collect();
    MwResponse {
        response: Some(Response::RespGetStoredKeyAccountsOfCoin(
            GetStoredKeyAccountsOfCoinResp { accounts },
        )),
    }
}

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
    let mut stored_key: StoredKey = match serde_json::from_slice(&param.stored_key_data) {
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

pub fn remove_stored_key_account_of_coin(param: RemoveStoredKeyAccountOfCoinParam) -> MwResponse {
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
    let mut stored_key: StoredKey = match serde_json::from_slice(&param.stored_key_data) {
        Ok(key) => key,
        Err(_) => {
            return get_json_error_response();
        }
    };
    stored_key.remove_accounts_of_coin(coin);
    MwResponse {
        response: Some(Response::RespRemoveAccountOfCoin(
            RemoveStoredKeyAccountOfCoinResp {
                stored_key: Some(StoredKeyInfo::from(stored_key)),
            },
        )),
    }
}

pub fn remove_account_of_address(param: RemoveStoredKeyAccountOfAddressParam) -> MwResponse {
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
    let mut stored_key: StoredKey = match serde_json::from_slice(&param.stored_key_data) {
        Ok(key) => key,
        Err(_) => {
            return get_json_error_response();
        }
    };
    match stored_key.remove_account_of_address(&param.address, coin) {
        Ok(_) => MwResponse {
            response: Some(Response::RespRemoveAccountOfAddress(
                RemoveStoredKeyAccountOfAddressResp {
                    stored_key: Some(StoredKeyInfo::from(stored_key)),
                },
            )),
        },
        Err(error) => get_error_response_by_error(error),
    }
}
