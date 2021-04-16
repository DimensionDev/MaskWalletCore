pub mod stored_key;
pub mod account;
pub mod encryption_parameters;

use crypto::Error as CryptoError;

pub enum Error {
    CryptoError(CryptoError),
}

impl From<CryptoError> for Error {
    fn from(err: CryptoError) -> Error {
        Error::CryptoError(err)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
