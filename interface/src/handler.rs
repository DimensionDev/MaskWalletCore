use super::api::{ MwResponse, mw_request, MwResponseError };
use super::api::mw_request::Request::*;
use super::api::mw_response::Response;
use super::param::*;
use super::coin::get_coin_info;
use super::response_util::*;

use wallet::stored_key::*;
use wallet::hd_wallet::HdWallet;

pub fn dispatch_request(request: mw_request::Request) -> MwResponse {
    match request {
        ParamLoadStoredKey(param) => {
            load_stored_keys(param)
        },
        ParamCreateStoredKey(param) => {
            create_stored_key(param)
        },
        ParamImportPrivateKey(param) => {
            create_stored_key_with_private_key(param)
        },
        ParamImportMnemonic(param) => {
            create_stored_key_with_mnemonic(param)
        }
        ParamGetStoredKeyAccountCount(param) => {
            get_stored_key_account_count(param)
        },
        ParamGetStoredKeyAccount(param) => {
            get_store_key_account(param)
        },
        ParamGetStoredKeyAllAccounts(param) => {
            get_stored_key_all_accounts(param)
        },
        ParamGetStoredKeyOfCoin(param) => {
            get_stored_key_account_of_coin(param)
        },
    }
}

fn load_stored_keys(param: StoredKeyLoadParam) -> MwResponse {
    let stored_keys_result: Result<Vec<StoredKey>, _> = param.data.iter().map(|json| serde_json::from_slice(&json) ).collect();
    match stored_keys_result {
        Ok(stored_keys) => MwResponse {
            response: Some(Response::RespLoadStoredKey(
                StoredKeyLoadResp {
                    stored_keys: stored_keys.into_iter().map(StoredKeyInfo::from).collect()
                }
            ))
        },
        Err(_) => {
            get_json_error_response()
        }
    }
}

fn create_stored_key(param: CreateStoredKeyParam) -> MwResponse {
    let stored_key: StoredKey = match StoredKey::create_with_mnemonic_random(&param.name, &param.password) {
        Ok(key) => key,
        Err(error) => {
            return get_error_response_by_error(error);
        }
    };
    MwResponse {
        response: Some(Response::RespCreateStoredKey(
            CreateStoredKeyResp {
                stored_key: Some(StoredKeyInfo::from(stored_key))
            }
        ))
    }
}

fn create_stored_key_with_private_key(param: PrivateStoredKeyImportParam) -> MwResponse {
    let coin_info = get_coin_info(param.coin);
    let coin = match coin_info {
        Some(coin_info) => coin_info,
        None => {
            return MwResponse {
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
            MwResponse {
                response: Some(Response::RespImportPrivateKey(
                    PrivateStoredKeyImportResp {
                        stored_key: Some(StoredKeyInfo::from(key))
                    }
                ))
            }
        },
        Err(error) => {
            get_error_response_by_error(error)
        }
    }
}

fn create_stored_key_with_mnemonic(param: MnemonicStoredKeyImportParam) -> MwResponse {
    let stored_key: StoredKey = match StoredKey::create_with_mnemonic(&param.name, &param.password, &param.mnemonic) {
        Ok(key) => key,
        Err(error) => {
            return get_error_response_by_error(error);
        }
    };
    MwResponse {
        response: Some(Response::RespCreateMnemonic(
            MnemonicStoredKeyImportResp {
                stored_key: Some(StoredKeyInfo::from(stored_key))
            }
        ))
    }
}

fn get_stored_key_account_count(param: GetStoredKeyAccountCountParam) -> MwResponse {
    let stored_key: StoredKey = match serde_json::from_slice(&param.data) {
        Ok(key) => key,
        Err(_) => {
            return get_json_error_response();
        }
    };
    MwResponse {
        response: Some(Response::RespGetStoredKeyAccountCount(
            GetStoredKeyAccountCountResp {
                count: stored_key.get_accounts_count(),
            }
        ))
    }
}

fn get_store_key_account(param: GetStoredKeyAccountParam) -> MwResponse {
    let stored_key: StoredKey = match serde_json::from_slice(&param.data) {
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
        response: Some(Response::RespGetStoredKeyAccount(
            GetStoredKeyAccountResp {
                account: Some(StoredKeyAccountInfo::from(account))
            }
        ))
    }
}

fn get_stored_key_all_accounts(param: GetStoredKeyAllAccountParam) -> MwResponse {
    let stored_key: StoredKey = match serde_json::from_slice(&param.data) {
        Ok(key) => key,
        Err(_) => {
            return get_json_error_response();
        }
    };
    let accounts_info: Vec<StoredKeyAccountInfo> = stored_key.get_all_accounts().iter().map(StoredKeyAccountInfo::from).collect();
    MwResponse {
        response: Some(Response::RespGetStoredKeyAllAccounts(
            GetStoredKeyAllAccountResp {
                accounts: accounts_info
            }
        ))
    }
}

fn get_stored_key_account_of_coin(param: GetStoredKeyAccountOfCoinParam) -> MwResponse {
    let coin_info = get_coin_info(param.coin);
    let coin = match coin_info {
        Some(coin_info) => coin_info,
        None => {
            return MwResponse {
                response: Some(Response::Error(MwResponseError{
                    error_code: "-1".to_owned(),
                    error_msg: "Invalid Coin Type".to_owned(),
                }))
            };
        }
    };
    let mut stored_key: StoredKey = match serde_json::from_slice(&param.stored_key_data) {
        Ok(key) => key,
        Err(_) => {
            return get_json_error_response();
        }
    };
    let optional_wallet: Option<HdWallet> = match param.optional_wallet {
        Some(get_stored_key_account_of_coin_param::OptionalWallet::WalletData(wallet_data)) => {
            match serde_json::from_slice(&wallet_data) {
                Ok(wallet) => wallet,
                Err(_) => None,
            }
        },
        _ => None
    };

    let optional_account = match stored_key.get_or_create_account_for_coin(coin, optional_wallet) {
        Ok(account) => account.map(|x| get_stored_key_account_of_coin_resp::OptionalAccount::Account(StoredKeyAccountInfo::from(x)) ),
        Err(error) => {
            return get_error_response_by_error(error);
        }
    };
    MwResponse {
        response: Some(Response::RespGetStoredKeyAccountOfCoin(
            GetStoredKeyAccountOfCoinResp {
                stored_key: Some(StoredKeyInfo::from(stored_key)),
                optional_account
            }
        ))
    }
}

