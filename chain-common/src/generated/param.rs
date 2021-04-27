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
pub struct LoadStoredKeyParam {
    #[prost(bytes="vec", repeated, tag="1")]
    pub data: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoadStoredKeyResp {
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
pub struct ImportPrivateStoredKeyParam {
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
pub struct ImportPrivateStoredKeyResp {
    #[prost(message, optional, tag="1")]
    pub stored_key: ::core::option::Option<StoredKeyInfo>,
}
///// 4. Create a new StoredKey with given mnemonic
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportMnemonicStoredKeyParam {
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
pub struct ImportMnemonicStoredKeyResp {
    #[prost(message, optional, tag="1")]
    pub stored_key: ::core::option::Option<StoredKeyInfo>,
}
///// 5. Create a new StoredKey with Json
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportJsonStoredKeyParam {
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
pub struct ImportJsonStoredKeyResp {
    #[prost(message, optional, tag="1")]
    pub stored_key: ::core::option::Option<StoredKeyInfo>,
}
///// 6. Get count of accounts of a StoredKey
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
///// 7. Get the account at specific index of a StoredKey
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
///// 8. Get all accounts information of a StoredKey
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
///// 9. Get the accounts of specific coin form a StoredKey
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStoredKeyAccountsOfCoinParam {
    #[prost(bytes="vec", tag="1")]
    pub stored_key_data: ::prost::alloc::vec::Vec<u8>,
    #[prost(enumeration="Coin", tag="2")]
    pub coin: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStoredKeyAccountsOfCoinResp {
    #[prost(message, optional, tag="1")]
    pub stored_key: ::core::option::Option<StoredKeyInfo>,
    #[prost(message, repeated, tag="2")]
    pub accounts: ::prost::alloc::vec::Vec<StoredKeyAccountInfo>,
}
///// 10. Add a new account of specific coin type into a StoredKey
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddStoredKeyAccountOfCoinParam {
    #[prost(bytes="vec", tag="1")]
    pub stored_key_data: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="2")]
    pub address: ::prost::alloc::string::String,
    #[prost(enumeration="Coin", tag="3")]
    pub coin: i32,
    #[prost(string, tag="4")]
    pub derivation_path: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub extetnded_public_key: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddStoredKeyAccountOfCoinResp {
    #[prost(message, optional, tag="1")]
    pub account: ::core::option::Option<StoredKeyAccountInfo>,
    #[prost(message, optional, tag="2")]
    pub stored_key: ::core::option::Option<StoredKeyInfo>,
}
///// 11. Remove all accounts of specific coin type into a StoredKey
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveStoredKeyAccountsOfCoinParam {
    #[prost(bytes="vec", tag="1")]
    pub stored_key_data: ::prost::alloc::vec::Vec<u8>,
    #[prost(enumeration="Coin", tag="2")]
    pub coin: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveStoredKeyAccountsOfCoinResp {
    #[prost(message, optional, tag="1")]
    pub stored_key: ::core::option::Option<StoredKeyInfo>,
}
///// 12. Remove account of the specific address and coin type
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveStoredKeyAccountOfAddressParam {
    #[prost(bytes="vec", tag="1")]
    pub stored_key_data: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="2")]
    pub address: ::prost::alloc::string::String,
    #[prost(enumeration="Coin", tag="3")]
    pub coin: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveStoredKeyAccountOfAddressResp {
    #[prost(message, optional, tag="1")]
    pub stored_key: ::core::option::Option<StoredKeyInfo>,
}
///// 13. Export the private key of StoredKey
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportKeyStorePrivateKeyParam {
    #[prost(bytes="vec", tag="1")]
    pub stored_key_data: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="2")]
    pub password: ::prost::alloc::string::String,
    #[prost(enumeration="Coin", tag="3")]
    pub coin: i32,
}
///// 14. Export the private key of StoredKey at specific derivation path. Fail if the StoredKey is not a Hd StoredKey
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportKeyStorePrivateKeyOfPathParam {
    #[prost(bytes="vec", tag="1")]
    pub stored_key_data: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="2")]
    pub password: ::prost::alloc::string::String,
    #[prost(enumeration="Coin", tag="3")]
    pub coin: i32,
    #[prost(string, tag="4")]
    pub derivation_path: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportKeyStorePrivateKeyResp {
    #[prost(string, tag="1")]
    pub private_key: ::prost::alloc::string::String,
}
///// 15. Export the mnemonic of the StoredKey. Fail if the StoredKey is not a Hd StoredKey
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportKeyStoreMnemonicParam {
    #[prost(bytes="vec", tag="1")]
    pub stored_key_data: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="2")]
    pub password: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportKeyStoreMnemonicResp {
    #[prost(string, tag="1")]
    pub mnemonic: ::prost::alloc::string::String,
}
///// 16. Export the V3 KeyStoreJson string of the StoredKey
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportKeyStoreJsonParam {
    #[prost(bytes="vec", tag="1")]
    pub stored_key_data: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="2")]
    pub password: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub new_password: ::prost::alloc::string::String,
    #[prost(enumeration="Coin", tag="4")]
    pub coin: i32,
}
///// 17. Export the V3 KeyStoreJson string of the StoredKey at specific derivation path. Fail if the StoredKey is not a Hd StoredKey
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportKeyStoreJsonOfPathParam {
    #[prost(bytes="vec", tag="1")]
    pub stored_key_data: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="2")]
    pub password: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub new_password: ::prost::alloc::string::String,
    #[prost(enumeration="Coin", tag="4")]
    pub coin: i32,
    #[prost(string, tag="5")]
    pub derivation_path: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportKeyStoreJsonResp {
    #[prost(string, tag="1")]
    pub json: ::prost::alloc::string::String,
}
///// 18. Create a new account to the StoredKey at specific derivation path. Fail if the StoredKey is not a Hd StoredKey
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateStoredKeyNewAccountParam {
    #[prost(bytes="vec", tag="1")]
    pub stored_key_data: ::prost::alloc::vec::Vec<u8>,
    #[prost(enumeration="Coin", tag="2")]
    pub coin: i32,
    #[prost(string, tag="3")]
    pub derivation_path: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub password: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateStoredKeyNewAccountResp {
    #[prost(message, optional, tag="1")]
    pub account: ::core::option::Option<StoredKeyAccountInfo>,
    #[prost(message, optional, tag="2")]
    pub stored_key: ::core::option::Option<StoredKeyInfo>,
}
/// 19. Sign a transaction
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignTransactionParam {
    #[prost(bytes="vec", tag="1")]
    pub stored_key_data: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="2")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub password: ::prost::alloc::string::String,
    #[prost(enumeration="Coin", tag="4")]
    pub coin: i32,
    #[prost(oneof="sign_transaction_param::Input", tags="5")]
    pub input: ::core::option::Option<sign_transaction_param::Input>,
}
/// Nested message and enum types in `SignTransactionParam`.
pub mod sign_transaction_param {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Input {
        #[prost(message, tag="5")]
        SignInput(super::super::ethereum::SignInput),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignTransactionResp {
    #[prost(oneof="sign_transaction_resp::Output", tags="1")]
    pub output: ::core::option::Option<sign_transaction_resp::Output>,
}
/// Nested message and enum types in `SignTransactionResp`.
pub mod sign_transaction_resp {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Output {
        #[prost(message, tag="1")]
        SignOutput(super::super::ethereum::SignOutput),
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
