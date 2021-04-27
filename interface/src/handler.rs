use std::convert::From;
use prost::Message;
use chain_common::api::{ MwResponse, mw_request, MwResponseError };
use chain_common::api::mw_request::Request::*;
use chain_common::api::mw_response::Response;
use chain_common::param::*;
use chain_common::ethereum;
use super::coins::get_coin_info;
use super::response_util::*;
use crate::encode_message;

use wallet::stored_key::*;

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
        },
        ParamImportJson(param) => {
            create_with_json(param)
        },
        ParamGetStoredKeyAccountCount(param) => {
            get_stored_key_account_count(param)
        },
        ParamGetStoredKeyAccount(param) => {
            get_store_key_account(param)
        },
        ParamGetStoredKeyAllAccounts(param) => {
            get_stored_key_all_accounts(param)
        },
        ParamGetStoredKeyAccountsOfCoin(param) => {
            get_stored_key_accounts_of_coin(param)
        },
        ParamAddAccountOfCoin(param) => {
            add_stored_key_account_of_coin(param)
        },
        ParamRemoveAccountsOfCoin(param) => {
            remove_stored_key_account_of_coin(param)
        },
        ParamRemoveAccountOfAddress(param) => {
            remove_account_of_address(param)
        },
        ParamExportPrivateKey(param) => {
            export_private_key(param)
        },
        ParamExportPrivateKeyOfPath(param) => {
            export_private_key_of_path(param)
        },
        ParamExportMnemonic(param) => {
            export_mnemonic(param)
        },
        ParamExportKeyStoreJson(param) => {
            export_key_store_json(param)
        },
        ParamExportKeyStoreJsonOfPath(param) => {
            export_key_store_json_of_path(param)
        },
        ParamSignTransaction(param) => {
            sign_transaction(param)
        },
    }
}

fn load_stored_keys(param: LoadStoredKeyParam) -> MwResponse {
    let stored_keys_result: Result<Vec<StoredKey>, _> = param.data.iter().map(|json| serde_json::from_slice(&json) ).collect();
    match stored_keys_result {
        Ok(stored_keys) => MwResponse {
            response: Some(Response::RespLoadStoredKey(
                LoadStoredKeyResp {
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

fn create_stored_key_with_private_key(param: ImportPrivateStoredKeyParam) -> MwResponse {
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
                    ImportPrivateStoredKeyResp {
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

fn create_stored_key_with_mnemonic(param: ImportMnemonicStoredKeyParam) -> MwResponse {
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
    let stored_key: StoredKey = match StoredKey::create_with_mnemonic_and_default_address(&param.name, &param.password, &param.mnemonic, coin.clone()) {
        Ok(key) => key,
        Err(error) => {
            return get_error_response_by_error(error);
        }
    };
    MwResponse {
        response: Some(Response::RespImportMnemonic(
            ImportMnemonicStoredKeyResp {
                stored_key: Some(StoredKeyInfo::from(stored_key))
            }
        ))
    }
}

fn create_with_json(param: ImportJsonStoredKeyParam) -> MwResponse {
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
    let stored_key: StoredKey = match StoredKey::create_with_json(&param.name, &param.password, &param.json, coin.clone()) {
        Ok(key) => key,
        Err(error) => {
            return get_error_response_by_error(error);
        }
    };
    MwResponse {
        response: Some(Response::RespImportJson(
            ImportJsonStoredKeyResp {
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

fn get_stored_key_accounts_of_coin(param: GetStoredKeyAccountsOfCoinParam) -> MwResponse {
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
    let stored_key: StoredKey = match serde_json::from_slice(&param.stored_key_data) {
        Ok(key) => key,
        Err(_) => {
            return get_json_error_response();
        }
    };

    let accounts: Vec<StoredKeyAccountInfo> = stored_key.get_accounts_of_coin(coin).iter().map(StoredKeyAccountInfo::from).collect();
    MwResponse {
        response: Some(Response::RespGetStoredKeyAccountsOfCoin(
            GetStoredKeyAccountsOfCoinResp {
                stored_key: Some(StoredKeyInfo::from(stored_key)),
                accounts
            }
        ))
    }
}

fn add_stored_key_account_of_coin(param: AddStoredKeyAccountOfCoinParam) -> MwResponse {
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
    let account = match stored_key.add_new_account_of_coin(&param.address, coin.clone(), &param.derivation_path, &param.extetnded_public_key) {
        Ok(account) => account,
        Err(error) => {
            return get_error_response_by_error(error);
        }
    };
    MwResponse {
        response: Some(Response::RespAddAccountOfCoin(
            AddStoredKeyAccountOfCoinResp {
                account: Some(StoredKeyAccountInfo::from(&account)),
                stored_key: Some(StoredKeyInfo::from(stored_key)),
            }
        ))
    }
}

fn remove_stored_key_account_of_coin(param: RemoveStoredKeyAccountsOfCoinParam) -> MwResponse {
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
    stored_key.remove_accounts_of_coin(coin);
    MwResponse {
        response: Some(Response::RespRemoveAccountOfCoin(
            RemoveStoredKeyAccountsOfCoinResp {
                stored_key: Some(StoredKeyInfo::from(stored_key)),
            }
        ))
    }
}

fn remove_account_of_address(param: RemoveStoredKeyAccountOfAddressParam) -> MwResponse {
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
    stored_key.remove_account_of_address(&param.address, coin);
    MwResponse {
        response: Some(Response::RespRemoveAccountOfAddress(
            RemoveStoredKeyAccountOfAddressResp {
                stored_key: Some(StoredKeyInfo::from(stored_key)),
            }
        ))
    }
}

fn export_private_key(param: ExportKeyStorePrivateKeyParam) -> MwResponse {
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
    let private_key = match stored_key.export_private_key(&param.password, coin) {
        Ok(key) => key,
        Err(error) => {
            return get_error_response_by_error(error);
        }
    };
    MwResponse {
        response: Some(Response::RespExportPrivateKey(
            ExportKeyStorePrivateKeyResp {
                private_key
            }
        ))
    }
}

fn export_private_key_of_path(param: ExportKeyStorePrivateKeyOfPathParam) -> MwResponse {
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
    let private_key = match stored_key.export_private_key_of_path(&param.password, coin, &param.derivation_path) {
        Ok(key) => key,
        Err(error) => {
            return get_error_response_by_error(error);
        }
    };
    MwResponse {
        response: Some(Response::RespExportPrivateKey(
            ExportKeyStorePrivateKeyResp {
                private_key
            }
        ))
    }
}

fn export_mnemonic(param: ExportKeyStoreMnemonicParam) -> MwResponse {
    let mut stored_key: StoredKey = match serde_json::from_slice(&param.stored_key_data) {
        Ok(key) => key,
        Err(_) => {
            return get_json_error_response();
        }
    };
    let mnemonic = match stored_key.export_mnemonic(&param.password) {
        Ok(key) => key,
        Err(error) => {
            return get_error_response_by_error(error);
        }
    };
    MwResponse {
        response: Some(Response::RespExportMnemonic(
            ExportKeyStoreMnemonicResp {
                mnemonic
            }
        ))
    }
}

fn export_key_store_json(param: ExportKeyStoreJsonParam) -> MwResponse {
    let mut stored_key: StoredKey = match serde_json::from_slice(&param.stored_key_data) {
        Ok(key) => key,
        Err(_) => {
            return get_json_error_response();
        }
    };
    let json = match stored_key.export_key_store_json(&param.password, &param.new_password) {
        Ok(key) => key,
        Err(error) => {
            return get_error_response_by_error(error);
        }
    };
    MwResponse {
        response: Some(Response::RespExportKeyStoreJson(
            ExportKeyStoreJsonResp {
                json
            }
        ))
    }
}

fn export_key_store_json_of_path(param: ExportKeyStoreJsonOfPathParam) -> MwResponse {
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
    let json = match stored_key.export_key_store_json_of_path(&param.password, &param.new_password, coin, &param.derivation_path) {
        Ok(key) => key,
        Err(error) => {
            return get_error_response_by_error(error);
        }
    };
    MwResponse {
        response: Some(Response::RespExportKeyStoreJson(
            ExportKeyStoreJsonResp {
                json
            }
        ))
    }
}

fn sign_transaction(param: SignTransactionParam) -> MwResponse {
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
    let input_struct = match param.sign_input {
        Some(input) => input,
        None => {
            return MwResponse {
                response: Some(Response::Error(MwResponseError{
                    error_code: "-1".to_owned(),
                    error_msg: "Invalid sign input".to_owned(),
                }))
            };
        }
    };
    let sign_transaction_param::SignInput::SignInput(chain_input) = input_struct;
    let encoded_input = match encode_message(&chain_input) {
        Ok(encoded) => encoded,
        Err(_) => {
            return MwResponse {
                response: Some(Response::Error(MwResponseError{
                    error_code: "-1".to_owned(),
                    error_msg: "Invalid sign input".to_owned(),
                }))
            };
        }
    };
    let sign_output = match stored_key.sign(&coin, &param.password, &param.address, &encoded_input) {
        Ok(key) => key,
        Err(error) => {
            return get_error_response_by_error(error);
        }
    };

    let ethereum::SignInput{ .. } = chain_input;
    let decoded_output_result = ethereum::SignOutput::decode(&sign_output[..]);

    let decoded_output = match decoded_output_result {
        Ok(decoded) => decoded,
        Err(_) => {
            return MwResponse {
                response: Some(Response::Error(MwResponseError{
                    error_code: "-1".to_owned(),
                    error_msg: "Invalid sign output".to_owned(),
                }))
            };
        }
    };

    MwResponse {
        response: Some(Response::RespSignTransaction(
            SignTransactionResp {
                sign_output: Some(sign_transaction_resp::SignOutput::SignOutput(decoded_output))
            }
        ))
    }
}