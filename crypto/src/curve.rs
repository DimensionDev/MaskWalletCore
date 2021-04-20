use std::str::FromStr;

pub enum Curve {
    Secp256k1,
}

impl FromStr for Curve {
    type Err = ();

    fn from_str(input: &str) -> Result<Curve, Self::Err> {
        match input.to_lowercase().as_str() {
            "secp256k1" => Ok(Self::Secp256k1),
            _  => Err(()),
        }
    }
}