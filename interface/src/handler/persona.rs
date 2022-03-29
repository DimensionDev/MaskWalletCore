use std::convert::{Into, TryFrom};

use chain_common::api::{
    mw_response::Response, persona_generation_param::Curve, JwkResp, MwResponse, MwResponseError,
    PersonaGenerationParam, PersonaGenerationResp,
};

use crypto::{jwk::JWK, Error};

pub fn generate_persona(param: &PersonaGenerationParam) -> MwResponse {
    generate_persona_inner(param).into()
}

fn generate_persona_inner(param: &PersonaGenerationParam) -> Result<Response, MwResponseError> {
    let curve = param.curve.map(|x| Curve::try_from(x));

    let resp = match curve {
        Some(curve) => {
            let jwk = match curve {
                Ok(Curve::Secp256k1) => JWK::derive_on(
                    &param.mnemonic,
                    &param.password,
                    &param.path,
                    crypto::curve::Curve::Secp256k1,
                ),
                Ok(Curve::Ed25519) => JWK::derive_on(
                    &param.mnemonic,
                    &param.password,
                    &param.path,
                    crypto::curve::Curve::Ed25519,
                ),

                _ => Err(Error::NotSupportedCurve),
            }?;

            Ok(JWKWrapper(jwk).resp())
        }

        None => Err(Error::NotSupportedCurve),
    }?;

    Ok(Response::RespGeneratePersona(resp))
}

#[derive(Debug, Clone)]
struct JWKWrapper(JWK);

impl JWKWrapper {
    fn resp(self) -> PersonaGenerationResp {
        let private_key = self.to_jwkresp(true);
        let public_key = self.to_jwkresp(false);

        PersonaGenerationResp {
            identifier: self.0.identifier,
            private_key: Some(private_key),
            public_key: Some(public_key),
        }
    }

    fn to_jwkresp(&self, contain_d: bool) -> JwkResp {
        JwkResp {
            crv: self.0.crv.clone(),
            identifier: self.0.identifier.clone(),
            ext: self.0.ext.clone(),
            x: self.0.x.clone(),
            y: self.0.y.clone(),
            key_ops: self.0.key_ops.clone(),
            kty: self.0.kty.clone(),
            d: if contain_d { self.0.d.clone() } else { None },
        }
    }
}
