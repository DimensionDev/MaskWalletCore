#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StoredKeyInfo {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(enumeration="StoredKeyType", tag="3")]
    pub r#type: i32,
    /// Raw data of the StoredKey, used in requests required an existing StoredKey
    #[prost(bytes="vec", tag="4")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StoredKeyAccountInfo {
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
// Begin of Requests/Responses definition 

///// 1. Get information from StoredKey raw data
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StoredKeyLoadParam {
    #[prost(bytes="vec", repeated, tag="1")]
    pub data: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StoredKeyLoadResp {
    #[prost(message, repeated, tag="1")]
    pub stored_keys: ::prost::alloc::vec::Vec<StoredKeyInfo>,
}
///// 2. Create a new StoredKey with random generated mnemonic
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateStoredKeyParam {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub password: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateStoredKeyResp {
    #[prost(message, optional, tag="1")]
    pub stored_key: ::core::option::Option<StoredKeyInfo>,
}
///// 3. Create a StoredKey with private key 
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrivateStoredKeyImportParam {
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
pub struct PrivateStoredKeyImportResp {
    #[prost(message, optional, tag="1")]
    pub stored_key: ::core::option::Option<StoredKeyInfo>,
}
///// 4. Create a new StoredKey with given mnemonic
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MnemonicStoredKeyImportParam {
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
pub struct MnemonicStoredKeyImportResp {
    #[prost(message, optional, tag="1")]
    pub stored_key: ::core::option::Option<StoredKeyInfo>,
}
///// 5. Create a new StoredKey with Json
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JsonStoredKeyImportParam {
    #[prost(string, tag="1")]
    pub json: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub password: ::prost::alloc::string::String,
    #[prost(enumeration="Coin", tag="4")]
    pub coin: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JsonStoredKeyImportResp {
    #[prost(message, optional, tag="1")]
    pub stored_key: ::core::option::Option<StoredKeyInfo>,
}
///// 5. Get count of accounts of a StoredKey
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStoredKeyAccountCountParam {
    /// StoredKey data returned from other response
    #[prost(bytes="vec", tag="1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStoredKeyAccountCountResp {
    #[prost(uint32, tag="1")]
    pub count: u32,
}
///// 6. Get the account at specific index of a StoredKey
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStoredKeyAccountParam {
    /// StoredKey data returned from other response
    #[prost(bytes="vec", tag="1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    /// Index of the account, begin from zero
    #[prost(uint32, tag="2")]
    pub index: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStoredKeyAccountResp {
    #[prost(message, optional, tag="1")]
    pub account: ::core::option::Option<StoredKeyAccountInfo>,
}
///// 7. Get all accounts information of a StoredKey
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStoredKeyAllAccountParam {
    #[prost(bytes="vec", tag="1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStoredKeyAllAccountResp {
    #[prost(message, repeated, tag="1")]
    pub accounts: ::prost::alloc::vec::Vec<StoredKeyAccountInfo>,
}
///// 8. Get the coin of specific coin form a StoredKey
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStoredKeyAccountOfCoinParam {
    #[prost(bytes="vec", tag="1")]
    pub stored_key_data: ::prost::alloc::vec::Vec<u8>,
    #[prost(enumeration="Coin", tag="2")]
    pub coin: i32,
    #[prost(oneof="get_stored_key_account_of_coin_param::OptionalWallet", tags="3")]
    pub optional_wallet: ::core::option::Option<get_stored_key_account_of_coin_param::OptionalWallet>,
}
/// Nested message and enum types in `GetStoredKeyAccountOfCoinParam`.
pub mod get_stored_key_account_of_coin_param {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum OptionalWallet {
        #[prost(bytes, tag="3")]
        WalletData(::prost::alloc::vec::Vec<u8>),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStoredKeyAccountOfCoinResp {
    #[prost(message, optional, tag="1")]
    pub stored_key: ::core::option::Option<StoredKeyInfo>,
    #[prost(oneof="get_stored_key_account_of_coin_resp::OptionalAccount", tags="2")]
    pub optional_account: ::core::option::Option<get_stored_key_account_of_coin_resp::OptionalAccount>,
}
/// Nested message and enum types in `GetStoredKeyAccountOfCoinResp`.
pub mod get_stored_key_account_of_coin_resp {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum OptionalAccount {
        #[prost(message, tag="2")]
        Account(super::StoredKeyAccountInfo),
    }
}
/// Begin of Structs definition used in Requests/Responses 
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Coin {
    Ethereum = 0,
    Polkadot = 1,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum StoredKeyType {
    PrivateKey = 0,
    Hd = 1,
}
