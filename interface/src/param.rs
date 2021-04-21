///// Create a stored_key with private key 
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrivateKeyStoreImportParam {
    /// Hex encoded private key string
    #[prost(string, tag="1")]
    pub private_key: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub password: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub name: ::prost::alloc::string::String,
    #[prost(enumeration="Coin", tag="4")]
    pub coin: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrivateKeyStoreImportResp {
    /// Uuid identifier of the created stored key
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
///// Get count of accounts of a stored_key
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStoredKeyAccountCountParam {
    /// Stored key data returned from other response
    #[prost(bytes="vec", tag="1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStoredKeyAccountCountResp {
    #[prost(uint32, tag="1")]
    pub count: u32,
}
///// Get the account of specific index of a stored_key
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStoredKeyAccountParam {
    /// Stored key data returned from other response
    #[prost(bytes="vec", tag="1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    /// Index of the account, begin from zero
    #[prost(uint32, tag="2")]
    pub index: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStoredKeyAccountResp {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub derivation_path: ::prost::alloc::string::String,
    /// Coin id
    #[prost(string, tag="3")]
    pub coin: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub extended_public_key: ::prost::alloc::string::String,
}
///// Create a new stored_key with random generated mnemonic
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateKeyStoreParam {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub password: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateKeyStoreResp {
    /// Uuid identifier of the created stored key
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
///// Create a new stored_key with mnemonic
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MnemonicKeyStoreImportParam {
    #[prost(string, tag="1")]
    pub mnemonic: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub password: ::prost::alloc::string::String,
    #[prost(enumeration="Coin", tag="4")]
    pub coin: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MnemonicKeyStoreImportResp {
    /// Uuid identifier of the created stored key
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Coin {
    Ethereum = 0,
    Polkadot = 1,
}
