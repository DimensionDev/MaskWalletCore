use super::api::{ MwResponse, mw_request};
use super::api::mw_request::Request::*;

use super::param::*;

use wallet::stored_key::*;
// use wallet::encryption::scrypt_parameters::ScryptParameters;

pub fn dispatch_request(request: mw_request::Request) -> MwResponse {
    let response = match request {
        ParamImportPrivateKey(param) => {
            create_stored_key(param)
        }
    };
    return MwResponse {
        is_success: true, 
        error_code: "".to_owned(),
        error_msg: "".to_owned(),
        data: "".to_owned(),
    };
}

fn create_stored_key(param: PrivateKeyStoreImportParam) -> MwResponse {
    let stored_key = StoredKey::create_with_private_key("test1", "password", "tt");

    MwResponse {
        is_success: true,
        error_code: "".to_owned(),
        error_msg: "".to_owned(),
        data: "".to_owned(),
    }
}