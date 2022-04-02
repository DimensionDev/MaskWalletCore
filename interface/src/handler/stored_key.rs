use crate::coins::get_coin_info;
use crate::response_util::*;
use chain_common::api::mw_response::Response;
use chain_common::api::*;
use wallet::coin_dispatcher::CoinDispatcher;
use wallet::stored_key::StoredKey;

pub fn load_stored_keys(param: LoadStoredKeyParam) -> MwResponse {
    let stored_keys_result: Result<Vec<StoredKey>, _> = param
        .data
        .iter()
        .map(|json| serde_json::from_slice(json))
        .collect();
    match stored_keys_result {
        Ok(stored_keys) => MwResponse {
            response: Some(Response::RespLoadStoredKey(LoadStoredKeyResp {
                stored_keys: stored_keys.into_iter().map(StoredKeyInfo::from).collect(),
            })),
        },
        Err(_) => get_json_error_response(),
    }
}

pub fn create_stored_key(param: CreateStoredKeyParam) -> MwResponse {
    let (stored_key, mnemonic) = match StoredKey::create_with_mnemonic_random(&param.password) {
        Ok(key) => key,
        Err(error) => {
            return get_error_response_by_error(error);
        }
    };
    MwResponse {
        response: Some(Response::RespCreateStoredKey(CreateStoredKeyResp {
            stored_key: Some(StoredKeyInfo::from(stored_key)),
            mnemonic,
        })),
    }
}

pub fn create_stored_key_with_private_key(param: ImportPrivateStoredKeyParam) -> MwResponse {
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
    let stored_key =
        StoredKey::create_with_private_key_and_coin(&param.password, &param.private_key, coin);
    match stored_key {
        Ok(key) => MwResponse {
            response: Some(Response::RespImportPrivateKey(ImportPrivateStoredKeyResp {
                stored_key: Some(StoredKeyInfo::from(key)),
            })),
        },
        Err(error) => get_error_response_by_error(error),
    }
}

pub fn create_stored_key_with_mnemonic(param: ImportMnemonicStoredKeyParam) -> MwResponse {
    let stored_key: StoredKey =
        match StoredKey::create_with_mnemonic(&param.password, &param.mnemonic) {
            Ok(key) => key,
            Err(error) => {
                return get_error_response_by_error(error);
            }
        };
    MwResponse {
        response: Some(Response::RespImportMnemonic(ImportMnemonicStoredKeyResp {
            stored_key: Some(StoredKeyInfo::from(stored_key)),
        })),
    }
}

pub fn create_stored_key_with_json(param: ImportJsonStoredKeyParam) -> MwResponse {
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
    let stored_key: StoredKey = match StoredKey::create_with_json(
        &param.key_store_json_password,
        &param.password,
        &param.json,
        coin,
    ) {
        Ok(key) => key,
        Err(error) => {
            return get_error_response_by_error(error);
        }
    };
    MwResponse {
        response: Some(Response::RespImportJson(ImportJsonStoredKeyResp {
            stored_key: Some(StoredKeyInfo::from(stored_key)),
        })),
    }
}

pub fn export_private_key(param: ExportKeyStorePrivateKeyParam) -> MwResponse {
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
    let private_key = match stored_key.export_private_key(&param.password, coin) {
        Ok(key) => key,
        Err(error) => {
            return get_error_response_by_error(error);
        }
    };
    MwResponse {
        response: Some(Response::RespExportPrivateKey(
            ExportKeyStorePrivateKeyResp { private_key },
        )),
    }
}

pub fn export_private_key_of_path(param: ExportKeyStorePrivateKeyOfPathParam) -> MwResponse {
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
    let private_key = match stored_key.export_private_key_of_path(
        &param.password,
        coin,
        &param.derivation_path,
    ) {
        Ok(key) => key,
        Err(error) => {
            return get_error_response_by_error(error);
        }
    };
    MwResponse {
        response: Some(Response::RespExportPrivateKey(
            ExportKeyStorePrivateKeyResp { private_key },
        )),
    }
}

pub fn export_mnemonic(param: ExportKeyStoreMnemonicParam) -> MwResponse {
    let stored_key: StoredKey = match serde_json::from_slice(&param.stored_key_data) {
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
        response: Some(Response::RespExportMnemonic(ExportKeyStoreMnemonicResp {
            mnemonic,
        })),
    }
}

pub fn export_key_store_json_of_address(param: ExportKeyStoreJsonOfAddressParam) -> MwResponse {
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
    let json = match stored_key.export_key_store_json_of_address(
        &param.password,
        &param.new_password,
        coin,
        &param.address,
    ) {
        Ok(key) => key,
        Err(error) => {
            return get_error_response_by_error(error);
        }
    };
    MwResponse {
        response: Some(Response::RespExportKeyStoreJson(ExportKeyStoreJsonResp {
            json,
        })),
    }
}

pub fn export_key_store_json_of_path(param: ExportKeyStoreJsonOfPathParam) -> MwResponse {
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
    let json = match stored_key.export_key_store_json_of_path(
        &param.password,
        &param.new_password,
        coin,
        &param.derivation_path,
    ) {
        Ok(key) => key,
        Err(error) => {
            return get_error_response_by_error(error);
        }
    };
    MwResponse {
        response: Some(Response::RespExportKeyStoreJson(ExportKeyStoreJsonResp {
            json,
        })),
    }
}

pub fn update_key_store_password(param: UpdateStoredKeyPasswordParam) -> MwResponse {
    let mut stored_key: StoredKey = match serde_json::from_slice(&param.stored_key_data) {
        Ok(key) => key,
        Err(_) => {
            return get_json_error_response();
        }
    };
    match stored_key.update_password(&param.old_password, &param.new_password) {
        Ok(_) => MwResponse {
            response: Some(Response::RespUpdateKeyStorePassword(
                UpdateStoredKeyPasswordResp {
                    stored_key: Some(StoredKeyInfo::from(stored_key)),
                },
            )),
        },
        Err(error) => get_error_response_by_error(error),
    }
}

pub fn get_supported_import_types(param: GetKeyStoreSupportImportTypeParam) -> MwResponse {
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
    let entry = match CoinDispatcher::get_entry(coin) {
        Ok(entry) => entry,
        Err(error) => {
            return get_error_response_by_error(error);
        }
    };
    let types = entry
        .get_supported_import_types()
        .into_iter()
        .map(|r#type| r#type as i32)
        .collect();
    MwResponse {
        response: Some(Response::RespGetStoredKeyImportType(
            GetKeyStoreSupportImportTypeResp { r#type: types },
        )),
    }
}

pub fn get_supported_export_types(param: GetKeyStoreSupportExportTypeParam) -> MwResponse {
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
    let entry = match CoinDispatcher::get_entry(coin) {
        Ok(entry) => entry,
        Err(error) => {
            return get_error_response_by_error(error);
        }
    };
    let types = entry
        .get_supported_export_types()
        .into_iter()
        .map(|r#type| r#type as i32)
        .collect();
    MwResponse {
        response: Some(Response::RespGetStoredKeyExportType(
            GetKeyStoreSupportExportTypeResp { r#type: types },
        )),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_get_import_types() {
        use chain_common::coin::Coin;
        use std::collections::HashMap;
        use wallet::coin_dispatcher::CoinDispatcher;

        let derivation_path = "m/44'/60'/0'/0/0";
        let coin = Coin {
            id: "60".to_owned(),
            name: "ethereum".to_owned(),
            coin_id: 60,
            symbol: "ETH".to_owned(),
            decimals: 18,
            blockchain: "Ethereum".to_owned(),
            derivation_path: derivation_path.to_owned(),
            curve: "secp256k1".to_owned(),
            public_key_type: "secp256k1Extended".to_owned(),
            all_info: HashMap::new(),
        };

        let entry = CoinDispatcher::get_entry(&coin).unwrap();
        let types: Vec<i32> = entry
            .get_supported_import_types()
            .into_iter()
            .map(|r#type| r#type as i32)
            .collect();
        assert_eq!(types, vec![0, 1, 2]);
    }
}
