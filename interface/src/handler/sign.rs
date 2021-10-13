use crate::coins::get_coin_info;
use crate::encode_message;
use crate::response_util::*;
use chain_common::api::mw_response::Response;
use chain_common::api::*;
use chain_common::ethereum;
use prost::Message;
use wallet::stored_key::StoredKey;

pub fn sign_transaction(param: SignTransactionParam) -> MwResponse {
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
    let input_struct = match param.input {
        Some(input) => input,
        None => {
            return MwResponse {
                response: Some(Response::Error(MwResponseError {
                    error_code: "-1".to_owned(),
                    error_msg: "Invalid sign input".to_owned(),
                })),
            };
        }
    };
    let sign_transaction_param::Input::SignInput(chain_input) = input_struct;
    let encoded_input = match encode_message(&chain_input) {
        Ok(encoded) => encoded,
        Err(_) => {
            return MwResponse {
                response: Some(Response::Error(MwResponseError {
                    error_code: "-1".to_owned(),
                    error_msg: "Invalid sign input".to_owned(),
                })),
            };
        }
    };
    let sign_output = match stored_key.sign(
        &coin,
        &param.password,
        &param.derivation_path,
        &encoded_input,
    ) {
        Ok(key) => key,
        Err(error) => {
            return get_error_response_by_error(error);
        }
    };

    let ethereum::SignInput { .. } = chain_input;
    let decoded_output_result = ethereum::SignOutput::decode(&sign_output[..]);

    let decoded_output = match decoded_output_result {
        Ok(decoded) => decoded,
        Err(_) => {
            return MwResponse {
                response: Some(Response::Error(MwResponseError {
                    error_code: "-1".to_owned(),
                    error_msg: "Invalid sign output".to_owned(),
                })),
            };
        }
    };

    MwResponse {
        response: Some(Response::RespSignTransaction(SignTransactionResp {
            output: Some(sign_transaction_resp::Output::SignOutput(decoded_output)),
        })),
    }
}
