use chain_common::api::mw_response::Response;
use chain_common::api::{MwResponse, MwResponseError};
use wallet::Error;

pub fn get_json_response_error() -> Option<Response> {
    Some(Response::Error(MwResponseError {
        error_code: "-1".to_owned(),
        error_msg: "Invalid Data".to_owned(),
    }))
}

pub fn get_json_error_response() -> MwResponse {
    MwResponse {
        response: get_json_response_error(),
    }
}

pub fn get_invalid_proto_resposne() -> MwResponse {
    MwResponse {
        response: Some(Response::Error(MwResponseError {
            error_code: "-1".to_owned(),
            error_msg: "Invalid Input".to_owned(),
        })),
    }
}

fn get_error_response(_error: Error) -> Response {
    Response::Error(MwResponseError {
        error_code: "-1".to_owned(),          // TODO: error to error code
        error_msg: "Invalid Data".to_owned(), // TODO: error to error message
    })
}

pub fn get_error_response_by_error(error: Error) -> MwResponse {
    MwResponse {
        response: Some(get_error_response(error)),
    }
}
