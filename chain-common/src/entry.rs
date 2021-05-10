use super::coin::Coin;
use super::private_key::PrivateKey;
use super::public_key::PublicKey;
use crypto::Error;

pub enum ChainImportType {
    PrivateKey = 0,
    Mnemonic = 1,
    KeyStoreJSON = 2,
}

pub enum ChainExportType {
    PrivateKey = 0,
    Mnemonic = 1,
    KeyStoreJSON = 2,
}

pub trait Entry {
    fn get_supported_import_types(&self) -> Vec<ChainImportType>;
    fn get_supported_export_types(&self) -> Vec<ChainExportType>;
    fn validate_address(&self, address: &str) -> bool;
    fn derive_address(
        &self,
        coin: &Coin,
        public_key: &PublicKey,
        p2pkh: &[u8],
        hrp: &[u8],
    ) -> Result<String, Error>;
    fn sign(&self, coin: &Coin, private_key: &PrivateKey, payload: &[u8])
        -> Result<Vec<u8>, Error>;
}
