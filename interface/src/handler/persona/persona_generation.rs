use std::convert::Into;

use chain_common::api::{
    curve::Curve, mw_response::Response, MwResponse, MwResponseError, PersonaGenerationParam,
    PersonaGenerationResp,
};

use crypto::{EncryptKey, Error, Secp256k1};

pub fn generate_persona(param: &PersonaGenerationParam) -> MwResponse {
    generate_persona_inner(param).into()
}

#[allow(dead_code)]
fn generate_persona_inner(param: &PersonaGenerationParam) -> Result<Response, MwResponseError> {
    // combine without password
    let resp = jwk_from(param)?;
    Ok(Response::RespGeneratePersona(resp))
}

fn jwk_from(param: &PersonaGenerationParam) -> Result<PersonaGenerationResp, Error> {
    let curve = param.curve.clone().and_then(|x| x.curve);

    if curve.is_some() {
        match curve.unwrap() {
            Curve::Secp256k1(_) => {
                let engine = EncryptKey::new(Secp256k1);
                let jwk = engine.generate_jwk(&param.mnemonic, &param.password, &param.path)?;
                Ok(PersonaGenerationResp {
                    identifier: jwk.identifier.to_string(),
                    private_key: "".to_string(),
                    public_key: "".to_string(),
                })
            }
            Curve::Ed25519(_) => Err(Error::NotSupportedCurve),
        }
    } else {
        Err(Error::NotSupportedCurve)
    }
}
#[cfg(test)]
#[allow(dead_code)]
mod test {
    use std::convert::TryInto;
    use std::str::FromStr;

    use chain_common::api::{curve::Curve, MwResponseError};
    use crypto::{bip39::Mnemonic, DerivationPath};

    #[test]
    fn test_mnemonic_validation() {
        let mnemonic1 = "";
        assert_eq!(Mnemonic::is_valid(mnemonic1), false);
    }

    #[test]
    fn test_curve_validation() {
        let curve_str = "";
        let curve: Result<Curve, MwResponseError> = curve_str.try_into();
        assert_eq!(curve.is_err(), true);
    }

    #[test]
    fn test_derivation_path_validation() {
        let path1_str = "m/44'/60'/0'/0'/98";
        let path = DerivationPath::from_str(path1_str);
        assert_eq!(path.is_ok(), true);
    }
}
