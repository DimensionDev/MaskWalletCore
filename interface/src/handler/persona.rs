use std::convert::TryInto;

use chain_common::api::{
    mw_response::Response, persona_generation_param::Curve, EncryptVersion, JwkResp, MwResponse,
    MwResponseError, PersonaGenerationParam, PersonaGenerationResp,
};

use crypto::{jwk::JWK, Error};

pub fn generate_persona(param: &PersonaGenerationParam) -> MwResponse {
    generate_persona_inner(param).into()
}

fn generate_persona_inner(param: &PersonaGenerationParam) -> Result<Response, MwResponseError> {
    let version = param.version.try_into()?;

    // currently only support v37
    if let EncryptVersion::V38 = version {
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

        _ => Err(Error::NotSupportedCurve),
    }?;

    Ok(Response::RespGeneratePersona(
        JWKWrapper(jwk).resp(param.version),
    ))
}

#[derive(Debug, Clone)]
struct JWKWrapper(JWK);

impl JWKWrapper {
    fn resp(self, version: i32) -> PersonaGenerationResp {
        let private_key = self.to_jwkresp(true);
        let public_key = self.to_jwkresp(false);

        PersonaGenerationResp {
            identifier: self.0.identifier,
            private_key: Some(private_key),
            public_key: Some(public_key),
            version: version,
        }
    }

    fn to_jwkresp(&self, include_d: bool) -> JwkResp {
        JwkResp {
            crv: self.0.crv.clone(),
            identifier: self.0.identifier.clone(),
            ext: self.0.ext.clone(),
            x: self.0.x.clone(),
            y: self.0.y.clone(),
            key_ops: self.0.key_ops.clone(),
            kty: self.0.kty.clone(),
            d: if include_d { self.0.d.clone() } else { None },
        }
    }
}
