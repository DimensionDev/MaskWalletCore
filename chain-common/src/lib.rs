mod generated;
pub use generated::api;
pub use generated::ethereum;

pub mod coin;
pub mod entry;
pub mod private_key;
pub mod public_key;

#[derive(Debug, PartialEq)]
pub enum Error {
    NotSupportedCoin,
    InvalidSignInput,
    InvalidPrivateKey,
}
