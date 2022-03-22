use std::convert::TryInto;
use std::str::FromStr;

use chain_common::api::mw_response::Response;
use chain_common::api::{
    MwResponse, MwResponseError, PersonaGenerationParam, PersonaGenerationResp,
};

use crypto::{bip39::Mnemonic, curve::Curve, DerivationPath, Error};

pub fn generate_persona(param: &PersonaGenerationParam) -> MwResponse {
    let result = generate_persona_inner(param);
    match result {
        Ok(resp) => MwResponse {
            response: Some(resp),
        },
        Err(error) => MwResponse {
            response: Some(Response::Error(error)),
        },
    }
}

#[allow(dead_code)]
fn generate_persona_inner(param: &PersonaGenerationParam) -> Result<Response, MwResponseError> {
    let is_valid_mnomenioc = Mnemonic::is_valid(param.mnemonic.as_str());
    if !is_valid_mnomenioc {
        return Err(MwResponseError {
            error_code: "-1".to_string(),
            error_msg: "invalid mnomenioc".to_string(),
        });
    }

    let mnemonic = Mnemonic::new(param.mnemonic.as_str(), param.password.as_str())?;

    let path = DerivationPath::from_str(param.path.as_str());
    if path.is_err() {
        return Err(MwResponseError {
            error_code: "-1".to_string(),
            error_msg: "invalid derivation path".to_string(),
        });
    }

    let curve: Result<Curve, Error> = param.curve.as_str().try_into();
    if curve.is_err() {
        return Err(MwResponseError {
            error_code: "-1".to_string(),
            error_msg: "unsupport curve type".to_string(),
        });
    }

    let resp = PersonaGenerationResp {
        identifier: "".to_string(),
        private_key: "".to_string(),
        public_key: "".to_string(),
    };

    Ok(Response::RespGeneratePersona(resp))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_mnemonic_validation() {
        let mnemonic1 = "";
        assert_eq!(Mnemonic::is_valid(mnemonic1), false);
    }

    #[test]
    fn test_curve_validation() {
        let curve_str = "";
        let curve: Result<Curve, Error> = curve_str.try_into();
        assert_eq!(curve.is_err(), true);
    }

    #[test]
    fn test_derivation_path_validation() {
        let path1_str = "m/44'/60'/0'/0'/98";
        let path = DerivationPath::from_str(path1_str);
        assert_eq!(path.is_ok(), true);
    }
}
