use std::convert::TryInto;

use chain_common::api::{
    encrypt_option::Version, mw_response::Response, Curve, AesJwkResp,
    EncryptOption, JwkResp, MwResponse, MwResponseError, PersonaGenerationParam,
    PersonaGenerationResp,
};

use crypto::{jwk::AesJWK, jwk::JWK, pbkdf2, Error};

pub fn generate_persona(param: &PersonaGenerationParam) -> MwResponse {
    generate_persona_inner(param).into()
}

fn generate_persona_inner(param: &PersonaGenerationParam) -> Result<Response, MwResponseError> {
    let option = param.option.clone().ok_or(Error::NotSupportedCipher)?;
    let version = option.version.try_into()?;

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

    let local_key: Option<AesJWK> = match version {
        Version::V38 => {
            let password = format!("{}{}", &jwk.x, &jwk.y);
            let key = pbkdf2::derive_key(password.as_bytes(), param.mnemonic.as_bytes(), 100_000);
            Some(AesJWK::new(&key))
        }
        _ => None,
    };

    Ok(Response::RespGeneratePersona(
        JWKWrapper(jwk).resp(local_key, Some(option)),
    ))
}

#[derive(Debug, Clone)]
struct JWKWrapper(JWK);

impl JWKWrapper {
    fn resp(
        self,
        local_key: Option<AesJWK>,
        option: Option<EncryptOption>,
    ) -> PersonaGenerationResp {
        let private_key = self.as_private_key();
        let public_key = self.as_public_key();

        let local_key_resp: Option<AesJwkResp> = match local_key {
            Some(aes_jwk) => Some(AesJwkResp {
                alg: aes_jwk.alg,
                ext: aes_jwk.ext,
                k: aes_jwk.k,
                key_ops: aes_jwk.key_ops,
                kty: aes_jwk.kty,
            }),
            None => None,
        };

        PersonaGenerationResp {
            identifier: self.0.identifier,
            private_key: Some(private_key),
            public_key: Some(public_key),
            local_key: local_key_resp,
            option,
        }
    }

    fn as_public_key(&self) -> JwkResp {
        JwkResp {
            crv: self.0.crv.clone(),
            ext: self.0.ext,
            x: self.0.x.clone(),
            y: self.0.y.clone(),
            key_ops: self.0.key_ops.clone(),
            kty: self.0.kty.clone(),
            d: None,
        }
    }

    fn as_private_key(&self) -> JwkResp {
        JwkResp {
            crv: self.0.crv.clone(),
            ext: self.0.ext,
            x: self.0.x.clone(),
            y: self.0.y.clone(),
            key_ops: self.0.key_ops.clone(),
            kty: self.0.kty.clone(),
            d: self.0.d.clone(),
        }
    }
}
