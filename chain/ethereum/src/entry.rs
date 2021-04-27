use bytes::BytesMut;
use prost::Message;
use chain_common::entry::Entry;
use chain_common::public_key::PublicKey;
use chain_common::private_key::PrivateKey;
use chain_common::coin::Coin;
use chain_common::ethereum::SignInput;
use crypto::Error;
use super::address::EthereumAddress;
use super::signer::Signer;

pub struct EthereumEntry;

impl Entry for EthereumEntry {
    fn derive_address(&self, coin: &Coin, public_key: &PublicKey, _p2pkh: &[u8], _hrp: &[u8]) -> Result<String, Error> {
        let address = EthereumAddress::new(public_key, &coin.id)?;
        Ok(address.to_string())
    }

    fn sign(&self, _coin: &Coin, private_key: &PrivateKey, payload: &[u8]) -> Result<Vec<u8>, Error> {
        let sign_input: SignInput = match SignInput::decode(payload) {
            Ok(request) => request,
            Err(_) => return Err(Error::InvalidPrivateKey),
        };
        let output = Signer::sign(&private_key, &sign_input).map_err(|_| Error::InvalidPrivateKey)?;

        let mut buf = BytesMut::with_capacity(output.encoded_len());
        output.encode(&mut buf).expect("Fail to encode the SignOutput");
        Ok(buf.to_vec())
    }
}