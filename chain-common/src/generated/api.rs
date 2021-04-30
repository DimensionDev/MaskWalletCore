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
        ParamLoadStoredKey(super::super::param::LoadStoredKeyParam),
        #[prost(message, tag="2")]
        ParamCreateStoredKey(super::super::param::CreateStoredKeyParam),
        #[prost(message, tag="3")]
        ParamImportPrivateKey(super::super::param::ImportPrivateStoredKeyParam),
        #[prost(message, tag="4")]
        ParamImportMnemonic(super::super::param::ImportMnemonicStoredKeyParam),
        #[prost(message, tag="5")]
        ParamImportJson(super::super::param::ImportJsonStoredKeyParam),
        #[prost(message, tag="6")]
        ParamGetStoredKeyAccountCount(super::super::param::GetStoredKeyAccountCountParam),
        #[prost(message, tag="7")]
        ParamGetStoredKeyAccount(super::super::param::GetStoredKeyAccountParam),
        #[prost(message, tag="8")]
        ParamGetStoredKeyAllAccounts(super::super::param::GetStoredKeyAllAccountParam),
        #[prost(message, tag="9")]
        ParamGetStoredKeyAccountsOfCoin(super::super::param::GetStoredKeyAccountsOfCoinParam),
        #[prost(message, tag="10")]
        ParamCreateAcccountOfCoinAtPath(super::super::param::CreateStoredKeyNewAccountAtPathParam),
        #[prost(message, tag="11")]
        ParamRemoveAccountsOfCoin(super::super::param::RemoveStoredKeyAccountsOfCoinParam),
        #[prost(message, tag="12")]
        ParamRemoveAccountOfAddress(super::super::param::RemoveStoredKeyAccountOfAddressParam),
        #[prost(message, tag="13")]
        ParamExportPrivateKey(super::super::param::ExportKeyStorePrivateKeyParam),
        #[prost(message, tag="14")]
        ParamExportPrivateKeyOfPath(super::super::param::ExportKeyStorePrivateKeyOfPathParam),
        #[prost(message, tag="15")]
        ParamExportMnemonic(super::super::param::ExportKeyStoreMnemonicParam),
        #[prost(message, tag="16")]
        ParamExportKeyStoreJson(super::super::param::ExportKeyStoreJsonParam),
        #[prost(message, tag="17")]
        ParamExportKeyStoreJsonOfPath(super::super::param::ExportKeyStoreJsonOfPathParam),
        #[prost(message, tag="18")]
        ParamSignTransaction(super::super::param::SignTransactionParam),
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
        RespLoadStoredKey(super::super::param::LoadStoredKeyResp),
        #[prost(message, tag="3")]
        RespCreateStoredKey(super::super::param::CreateStoredKeyResp),
        #[prost(message, tag="4")]
        RespImportPrivateKey(super::super::param::ImportPrivateStoredKeyResp),
        #[prost(message, tag="5")]
        RespImportMnemonic(super::super::param::ImportMnemonicStoredKeyResp),
        #[prost(message, tag="6")]
        RespImportJson(super::super::param::ImportJsonStoredKeyResp),
        #[prost(message, tag="7")]
        RespGetStoredKeyAccountCount(super::super::param::GetStoredKeyAccountCountResp),
        #[prost(message, tag="8")]
        RespGetStoredKeyAccount(super::super::param::GetStoredKeyAccountResp),
        #[prost(message, tag="9")]
        RespGetStoredKeyAllAccounts(super::super::param::GetStoredKeyAllAccountResp),
        #[prost(message, tag="10")]
        RespGetStoredKeyAccountsOfCoin(super::super::param::GetStoredKeyAccountsOfCoinResp),
        #[prost(message, tag="11")]
        RespCreateAccountOfCoinAtPath(super::super::param::CreateStoredKeyNewAccountAtPathResp),
        #[prost(message, tag="12")]
        RespRemoveAccountOfCoin(super::super::param::RemoveStoredKeyAccountsOfCoinResp),
        #[prost(message, tag="13")]
        RespRemoveAccountOfAddress(super::super::param::RemoveStoredKeyAccountOfAddressResp),
        #[prost(message, tag="14")]
        RespExportPrivateKey(super::super::param::ExportKeyStorePrivateKeyResp),
        #[prost(message, tag="15")]
        RespExportMnemonic(super::super::param::ExportKeyStoreMnemonicResp),
        #[prost(message, tag="16")]
        RespExportKeyStoreJson(super::super::param::ExportKeyStoreJsonResp),
        #[prost(message, tag="17")]
        RespSignTransaction(super::super::param::SignTransactionResp),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MwResponseError {
    #[prost(string, tag="1")]
    pub error_code: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub error_msg: ::prost::alloc::string::String,
}
