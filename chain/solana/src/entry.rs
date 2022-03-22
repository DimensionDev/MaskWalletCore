use super::address::SolanaAddress;
use chain_common::coin::Coin;
use chain_common::entry::{ChainExportType, ChainImportType, Entry};
use chain_common::private_key::PrivateKey;
use chain_common::public_key::PublicKey;
use crypto::Error;

pub struct SolanaEntry;

impl Entry for SolanaEntry {
    fn get_supported_import_types(&self) -> Vec<ChainImportType> {
        vec![ChainImportType::PrivateKey, ChainImportType::Mnemonic]
    }

    fn get_supported_export_types(&self) -> Vec<ChainExportType> {
        vec![ChainExportType::PrivateKey]
    }

    fn validate_address(&self, address: &str) -> bool {
        SolanaAddress::is_valid(&address)
    }

    fn derive_address(
        &self,
        _coin: &Coin,
        public_key: &PublicKey,
        _p2pkh: &[u8],
        _hrp: &[u8],
    ) -> Result<String, Error> {
        let address = SolanaAddress::new(public_key)?;
        Ok(address.to_string())
    }

    fn sign(
        &self,
        _coin: &Coin,
        _private_key: &PrivateKey,
        _payload: &[u8],
    ) -> Result<Vec<u8>, Error> {
        Ok(vec![])
        // let sign_input: SignInput = match SignInput::decode(payload) {
        //     Ok(request) => request,
        //     Err(_) => return Err(Error::InvalidPrivateKey),
        // };
        // let output =
        //     Signer::sign(&private_key, &sign_input).map_err(|_| Error::InvalidPrivateKey)?;

        // let mut buf = BytesMut::with_capacity(output.encoded_len());
        // output
        //     .encode(&mut buf)
        //     .expect("Fail to encode the SignOutput");
        // Ok(buf.to_vec())
    }
}
