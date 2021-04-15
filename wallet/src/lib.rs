pub mod stored_key;
pub mod account;
pub mod encryption_parameters;
pub mod coin;
pub mod private_key;
pub mod public_key;

use crypto::Error as CryptoError;

pub enum Error {
    CryptoError(CryptoError),

    NotSupportedCurve,
    NotSupportedPublicKeyType,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
