use chain_common::api::mw_response::Response;
use chain_common::api::{MwResponse, PersonaGenerationParam, PersonaGenerationResp};

pub fn generate_persona(param: &PersonaGenerationParam) -> MwResponse {
    let resp = PersonaGenerationResp {
        identifier: "".to_owned(),
        private_key: "".to_owned(),
        public_key: "".to_owned(),
    };

    MwResponse {
        response: Some(Response::RespGeneratePersona(resp)),
    }
}
