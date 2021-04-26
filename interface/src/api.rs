#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MwRequest {
    #[prost(oneof="mw_request::Request", tags="1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12")]
    pub request: ::core::option::Option<mw_request::Request>,
}
/// Nested message and enum types in `MWRequest`.
pub mod mw_request {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Request {
        #[prost(message, tag="1")]
        ParamLoadStoredKey(super::super::param::StoredKeyLoadParam),
        #[prost(message, tag="2")]
        ParamCreateStoredKey(super::super::param::CreateStoredKeyParam),
        #[prost(message, tag="3")]
        ParamImportPrivateKey(super::super::param::PrivateStoredKeyImportParam),
        #[prost(message, tag="4")]
        ParamImportMnemonic(super::super::param::MnemonicStoredKeyImportParam),
        #[prost(message, tag="5")]
        ParamImportJson(super::super::param::JsonStoredKeyImportParam),
        #[prost(message, tag="6")]
        ParamGetStoredKeyAccountCount(super::super::param::GetStoredKeyAccountCountParam),
        #[prost(message, tag="7")]
        ParamGetStoredKeyAccount(super::super::param::GetStoredKeyAccountParam),
        #[prost(message, tag="8")]
        ParamGetStoredKeyAllAccounts(super::super::param::GetStoredKeyAllAccountParam),
        #[prost(message, tag="9")]
        ParamGetStoredKeyAccountsOfCoin(super::super::param::GetStoredKeyAccountsOfCoinParam),
        #[prost(message, tag="10")]
        ParamAddAccountOfCoin(super::super::param::AddStoredKeyAccountOfCoinParam),
        #[prost(message, tag="11")]
        ParamRemoveAccountsOfCoin(super::super::param::RemoveStoredKeyAccountsOfCoinParam),
        #[prost(message, tag="12")]
        ParamRemoveAccountOfAddress(super::super::param::RemoveStoredKeyAccountOfAddressParam),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MwResponse {
    #[prost(oneof="mw_response::Response", tags="1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13")]
    pub response: ::core::option::Option<mw_response::Response>,
}
/// Nested message and enum types in `MWResponse`.
pub mod mw_response {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        #[prost(message, tag="1")]
        Error(super::MwResponseError),
        #[prost(message, tag="2")]
        RespLoadStoredKey(super::super::param::StoredKeyLoadResp),
        #[prost(message, tag="3")]
        RespCreateStoredKey(super::super::param::CreateStoredKeyResp),
        #[prost(message, tag="4")]
        RespImportPrivateKey(super::super::param::PrivateStoredKeyImportResp),
        #[prost(message, tag="5")]
        RespCreateMnemonic(super::super::param::MnemonicStoredKeyImportResp),
        #[prost(message, tag="6")]
        RespImportJson(super::super::param::JsonStoredKeyImportResp),
        #[prost(message, tag="7")]
        RespGetStoredKeyAccountCount(super::super::param::GetStoredKeyAccountCountResp),
        #[prost(message, tag="8")]
        RespGetStoredKeyAccount(super::super::param::GetStoredKeyAccountResp),
        #[prost(message, tag="9")]
        RespGetStoredKeyAllAccounts(super::super::param::GetStoredKeyAllAccountResp),
        #[prost(message, tag="10")]
        RespGetStoredKeyAccountsOfCoin(super::super::param::GetStoredKeyAccountsOfCoinResp),
        #[prost(message, tag="11")]
        RespAddAccountOfCoin(super::super::param::AddStoredKeyAccountOfCoinResp),
        #[prost(message, tag="12")]
        RespRemoveAccountOfCoin(super::super::param::RemoveStoredKeyAccountsOfCoinResp),
        #[prost(message, tag="13")]
        RespRemoveAccountOfAddress(super::super::param::RemoveStoredKeyAccountOfAddressResp),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MwResponseError {
    #[prost(string, tag="1")]
    pub error_code: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub error_msg: ::prost::alloc::string::String,
}
