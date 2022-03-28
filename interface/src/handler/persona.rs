use std::convert::Into;

use chain_common::api::{
    curve::Curve, mw_response::Response, MwResponse, MwResponseError, PersonaGenerationParam,
    PersonaGenerationResp,
};

use crypto::{jwk::JWK, Error};

pub fn generate_persona(param: &PersonaGenerationParam) -> MwResponse {
    generate_persona_inner(param).into()
}

fn generate_persona_inner(param: &PersonaGenerationParam) -> Result<Response, MwResponseError> {
    let curve = param.curve.clone().and_then(|x| x.curve);
    let resp = match curve {
        Some(curve) => {
            let _jwk = match curve {
                Curve::Secp256k1(_) => JWK::derive_on(
                    &param.mnemonic,
                    &param.password,
                    &param.path,
                    crypto::curve::Curve::Secp256k1,
                ),
                Curve::Ed25519(_) => JWK::derive_on(
                    &param.mnemonic,
                    &param.password,
                    &param.path,
                    crypto::curve::Curve::Ed25519,
                ),
            }?;

            Ok(PersonaGenerationResp {
                identifier: "".to_string(),
                private_key: "".to_string(),
                public_key: "".to_string(),
            })
        }

        None => Err(Error::NotSupportedCurve),
    }?;

    Ok(Response::RespGeneratePersona(resp))
}
