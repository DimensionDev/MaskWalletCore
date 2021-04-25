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
///// 9. Get the coin of specific coin form a StoredKey
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStoredKeyAccountsOfCoinParam {
    #[prost(bytes="vec", tag="1")]
    pub stored_key_data: ::prost::alloc::vec::Vec<u8>,
    #[prost(enumeration="Coin", tag="2")]
    pub coin: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStoredKeyAccountOfCoinResp {
    #[prost(message, optional, tag="1")]
    pub stored_key: ::core::option::Option<StoredKeyInfo>,
    #[prost(message, repeated, tag="2")]
    pub account: ::prost::alloc::vec::Vec<StoredKeyAccountInfo>,
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
///// 13. Get the vault of secrects contained in the StoredKey
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetVaultOfStoredKeyParam {
    #[prost(bytes="vec", tag="1")]
    pub stored_key_data: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="2")]
    pub password: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetVaultOfStoredKeyResp {
    #[prost(bytes="vec", tag="1")]
    pub vault: ::prost::alloc::vec::Vec<u8>,
}
///// 14. Export the private key of StoredKey
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportKeyStorePrivateKeyParam {
    #[prost(bytes="vec", tag="1")]
    pub stored_key_data: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="2")]
    pub password: ::prost::alloc::string::String,
    #[prost(enumeration="Coin", tag="3")]
    pub coin: i32,
}
///// 15. Export the private key of StoredKey at specific derivation path. Fail if the StoredKey is not a Hd StoredKey
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
///// 16. Export the mnemonic of the StoredKey. Fail if the StoredKey is not a Hd StoredKey
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
///// 17. Export the V3 KeyStoreJson string of the StoredKey
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportKeyStoreJsonOfParam {
    #[prost(bytes="vec", tag="1")]
    pub stored_key_data: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="2")]
    pub password: ::prost::alloc::string::String,
    #[prost(enumeration="Coin", tag="3")]
    pub coin: i32,
}
///// 18. Export the V3 KeyStoreJson string of the StoredKey at specific derivation path. Fail if the StoredKey is not a Hd StoredKey
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportKeyStoreJsonOfPathParam {
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
pub struct ExportKeyStoreJsonResp {
    #[prost(string, tag="1")]
    pub json: ::prost::alloc::string::String,
}
///// 19. Create a new account to the StoredKey at specific derivation path. Fail if the StoredKey is not a Hd StoredKey
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
