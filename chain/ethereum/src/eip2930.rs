use ethereum_types::{Address, H256};
use rlp_derive::{RlpEncodable, RlpEncodableWrapper};

#[derive(RlpEncodableWrapper)]
pub struct AccessList(pub Vec<AccessListItem>);

impl From<Vec<AccessListItem>> for AccessList {
    fn from(src: Vec<AccessListItem>) -> AccessList {
        AccessList(src)
    }
}

/// Access list item
#[derive(RlpEncodable)]
pub struct AccessListItem {
    /// Accessed address
    pub address: Address,
    /// Accessed storage keys
    pub storage_keys: Vec<H256>,
}
