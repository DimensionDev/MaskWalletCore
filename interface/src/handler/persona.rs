use std::convert::TryInto;

use chain_common::api::{
    encrypt_option::Version, mw_response::Response, persona_generation_param::Curve, EncryptOption,
    JwkResp, MwResponse, MwResponseError, PersonaGenerationParam, PersonaGenerationResp,
};

use crypto::{jwk::JWK, Error};

pub fn generate_persona(param: &PersonaGenerationParam) -> MwResponse {
    generate_persona_inner(param).into()
}

fn generate_persona_inner(param: &PersonaGenerationParam) -> Result<Response, MwResponseError> {
    let option = param.option.clone().ok_or(Error::NotSupportedCipher)?;
    let version = option.version.try_into()?;

    // currently only support v37
    if let Version::V38 = version {
        return Err(Error::NotSupportedCipher.into());
    }

    let curve = param.curve.try_into();
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

        Err(e) => return Err(e),
    }?;

    Ok(Response::RespGeneratePersona(
        JWKWrapper(jwk).resp(Some(option)),
    ))
}

#[derive(Debug, Clone)]
struct JWKWrapper(JWK);

impl JWKWrapper {
    fn resp(self, option: Option<EncryptOption>) -> PersonaGenerationResp {
        let private_key = self.to_jwkresp(true);
        let public_key = self.to_jwkresp(false);

        PersonaGenerationResp {
            identifier: self.0.identifier,
            private_key: private_key,
            public_key: public_key,
            option,
        }
    }

    fn to_jwkresp(&self, include_d: bool) -> JwkResp {
        JwkResp {
            crv: self.0.crv.clone(),
            identifier: self.0.identifier.clone(),
            ext: self.0.ext,
            x: self.0.x.clone(),
            y: self.0.y.clone(),
            key_ops: self.0.key_ops.clone(),
            kty: self.0.kty.clone(),
            d: if include_d { self.0.d.clone() } else { None },
        }
    }
}
