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
/// Create a new account to the StoredKey at specific derivation path. Fail if the StoredKey is not a Hd StoredKey
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
/// Get count of accounts of a StoredKey
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
/// Get the account at specific index of a StoredKey
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
/// Get all accounts information of a StoredKey
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
/// Get the accounts of specific coin form a StoredKey
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
/// Add a new account of specific coin type into a StoredKey
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
/// Remove all accounts of specific coin type into a StoredKey
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
/// Remove account of the specific address and coin type
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
/// Export the private key of StoredKey
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportKeyStorePrivateKeyParam {
    #[prost(bytes="vec", tag="1")]
    pub stored_key_data: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="2")]
    pub password: ::prost::alloc::string::String,
    #[prost(enumeration="Coin", tag="3")]
    pub coin: i32,
}
/// Export the private key of StoredKey at specific derivation path. Fail if the StoredKey is not a Hd StoredKey
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
/// Export the mnemonic of the StoredKey. Fail if the StoredKey is not a Hd StoredKey
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
/// Export the V3 KeyStoreJSON string of the StoredKey
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
/// Export the V3 KeyStoreJSON string of the StoredKey at specific derivation path. Fail if the StoredKey is not a Hd StoredKey
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
/// Get information from StoredKey raw data
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
/// Create a new StoredKey with random generated mnemonic
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
/// Create a StoredKey with private key 
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
/// Create a new StoredKey with given mnemonic
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
/// Create a new StoredKey with JSON
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
/// Sign a transaction
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
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MwRequest {
    #[prost(oneof="mw_request::Request", tags="1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18")]
    pub request: ::core::option::Option<mw_request::Request>,
}
/// Nested message and enum types in `MWRequest`.
pub mod mw_request {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Request {
        #[prost(message, tag="1")]
        ParamLoadStoredKey(super::LoadStoredKeyParam),
        #[prost(message, tag="2")]
        ParamCreateStoredKey(super::CreateStoredKeyParam),
        #[prost(message, tag="3")]
        ParamImportPrivateKey(super::ImportPrivateStoredKeyParam),
        #[prost(message, tag="4")]
        ParamImportMnemonic(super::ImportMnemonicStoredKeyParam),
        #[prost(message, tag="5")]
        ParamImportJson(super::ImportJsonStoredKeyParam),
        #[prost(message, tag="6")]
        ParamGetStoredKeyAccountCount(super::GetStoredKeyAccountCountParam),
        #[prost(message, tag="7")]
        ParamGetStoredKeyAccount(super::GetStoredKeyAccountParam),
        #[prost(message, tag="8")]
        ParamGetStoredKeyAllAccounts(super::GetStoredKeyAllAccountParam),
        #[prost(message, tag="9")]
        ParamGetStoredKeyAccountsOfCoin(super::GetStoredKeyAccountsOfCoinParam),
        #[prost(message, tag="10")]
        ParamCreateAcccountOfCoinAtPath(super::CreateStoredKeyNewAccountParam),
        #[prost(message, tag="11")]
        ParamRemoveAccountsOfCoin(super::RemoveStoredKeyAccountsOfCoinParam),
        #[prost(message, tag="12")]
        ParamRemoveAccountOfAddress(super::RemoveStoredKeyAccountOfAddressParam),
        #[prost(message, tag="13")]
        ParamExportPrivateKey(super::ExportKeyStorePrivateKeyParam),
        #[prost(message, tag="14")]
        ParamExportPrivateKeyOfPath(super::ExportKeyStorePrivateKeyOfPathParam),
        #[prost(message, tag="15")]
        ParamExportMnemonic(super::ExportKeyStoreMnemonicParam),
        #[prost(message, tag="16")]
        ParamExportKeyStoreJson(super::ExportKeyStoreJsonParam),
        #[prost(message, tag="17")]
        ParamExportKeyStoreJsonOfPath(super::ExportKeyStoreJsonOfPathParam),
        #[prost(message, tag="18")]
        ParamSignTransaction(super::SignTransactionParam),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MwResponse {
    #[prost(oneof="mw_response::Response", tags="1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17")]
    pub response: ::core::option::Option<mw_response::Response>,
}
/// Nested message and enum types in `MWResponse`.
pub mod mw_response {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        #[prost(message, tag="1")]
        Error(super::MwResponseError),
        #[prost(message, tag="2")]
        RespLoadStoredKey(super::LoadStoredKeyResp),
        #[prost(message, tag="3")]
        RespCreateStoredKey(super::CreateStoredKeyResp),
        #[prost(message, tag="4")]
        RespImportPrivateKey(super::ImportPrivateStoredKeyResp),
        #[prost(message, tag="5")]
        RespImportMnemonic(super::ImportMnemonicStoredKeyResp),
        #[prost(message, tag="6")]
        RespImportJson(super::ImportJsonStoredKeyResp),
        #[prost(message, tag="7")]
        RespGetStoredKeyAccountCount(super::GetStoredKeyAccountCountResp),
        #[prost(message, tag="8")]
        RespGetStoredKeyAccount(super::GetStoredKeyAccountResp),
        #[prost(message, tag="9")]
        RespGetStoredKeyAllAccounts(super::GetStoredKeyAllAccountResp),
        #[prost(message, tag="10")]
        RespGetStoredKeyAccountsOfCoin(super::GetStoredKeyAccountsOfCoinResp),
        #[prost(message, tag="11")]
        RespCreateAccountOfCoinAtPath(super::CreateStoredKeyNewAccountResp),
        #[prost(message, tag="12")]
        RespRemoveAccountOfCoin(super::RemoveStoredKeyAccountsOfCoinResp),
        #[prost(message, tag="13")]
        RespRemoveAccountOfAddress(super::RemoveStoredKeyAccountOfAddressResp),
        #[prost(message, tag="14")]
        RespExportPrivateKey(super::ExportKeyStorePrivateKeyResp),
        #[prost(message, tag="15")]
        RespExportMnemonic(super::ExportKeyStoreMnemonicResp),
        #[prost(message, tag="16")]
        RespExportKeyStoreJson(super::ExportKeyStoreJsonResp),
        #[prost(message, tag="17")]
        RespSignTransaction(super::SignTransactionResp),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MwResponseError {
    #[prost(string, tag="1")]
    pub error_code: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub error_msg: ::prost::alloc::string::String,
}
