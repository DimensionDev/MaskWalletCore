#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MwRequest {
    #[prost(oneof="mw_request::Request", tags="1, 2, 3, 4, 5, 6, 7, 8")]
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
        ParamGetStoredKeyAccountCount(super::super::param::GetStoredKeyAccountCountParam),
        #[prost(message, tag="6")]
        ParamGetStoredKeyAccount(super::super::param::GetStoredKeyAccountParam),
        #[prost(message, tag="7")]
        ParamGetStoredKeyAllAccounts(super::super::param::GetStoredKeyAllAccountParam),
        #[prost(message, tag="8")]
        ParamGetStoredKeyOfCoin(super::super::param::GetStoredKeyAccountOfCoinParam),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MwResponse {
    #[prost(oneof="mw_response::Response", tags="1, 2, 3, 4, 5, 6, 7, 8, 9")]
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
        RespGetStoredKeyAccountCount(super::super::param::GetStoredKeyAccountCountResp),
        #[prost(message, tag="7")]
        RespGetStoredKeyAccount(super::super::param::GetStoredKeyAccountResp),
        #[prost(message, tag="8")]
        RespGetStoredKeyAllAccounts(super::super::param::GetStoredKeyAllAccountResp),
        #[prost(message, tag="9")]
        RespGetStoredKeyAccountOfCoin(super::super::param::GetStoredKeyAccountOfCoinResp),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MwResponseError {
    #[prost(string, tag="1")]
    pub error_code: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub error_msg: ::prost::alloc::string::String,
}
