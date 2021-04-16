use super::address::EthereumAddress;

pub enum ChecksumType {
    eip55,
    wanchain,
}

pub fn checksum(address: EthereumAddress, r#type: ChecksumType) {
    
}