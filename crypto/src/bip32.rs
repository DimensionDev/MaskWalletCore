use bitcoin::Network;
use bitcoin::util::bip32::{ ExtendedPrivKey };
use crate::Error;

pub fn get_new_master(seed: &[u8]) -> Result<ExtendedPrivKey, Error> {
    ExtendedPrivKey::new_master(Network::Bitcoin, &seed).map_err(|_| Error::InvalidSeed)
}

pub fn derive(path: &str) {
    
}