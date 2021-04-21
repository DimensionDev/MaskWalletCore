#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MwRequest {
    #[prost(oneof="mw_request::Request", tags="1, 2, 3, 4, 5")]
    pub request: ::core::option::Option<mw_request::Request>,
}
/// Nested message and enum types in `MWRequest`.
pub mod mw_request {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Request {
        #[prost(message, tag="1")]
        ParamImportPrivateKey(super::super::param::PrivateKeyStoreImportParam),
        #[prost(message, tag="2")]
        ParamGetStoredKeyAccountCount(super::super::param::GetStoredKeyAccountCountParam),
        #[prost(message, tag="3")]
        ParamGetStoredKeyAccount(super::super::param::GetStoredKeyAccountParam),
        #[prost(message, tag="4")]
        ParamCreateStoredKey(super::super::param::CreateKeyStoreParam),
        #[prost(message, tag="5")]
        ParamImportMnemonic(super::super::param::MnemonicKeyStoreImportParam),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MwResponse {
    #[prost(oneof="mw_response::Response", tags="1, 2, 3, 4, 5, 6")]
    pub response: ::core::option::Option<mw_response::Response>,
}
/// Nested message and enum types in `MWResponse`.
pub mod mw_response {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        #[prost(message, tag="1")]
        Error(super::MwResponseError),
        #[prost(message, tag="2")]
        RespImportPrivateKey(super::super::param::PrivateKeyStoreImportResp),
        #[prost(message, tag="3")]
        RespGetStoredKeyAccountCount(super::super::param::GetStoredKeyAccountCountResp),
        #[prost(message, tag="4")]
        RespGetStoredKeyAccount(super::super::param::GetStoredKeyAccountResp),
        #[prost(message, tag="5")]
        RespCreateStoredKey(super::super::param::CreateKeyStoreResp),
        #[prost(message, tag="6")]
        RespCreateMnemonic(super::super::param::MnemonicKeyStoreImportResp),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MwResponseError {
    #[prost(string, tag="1")]
    pub error_code: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub error_msg: ::prost::alloc::string::String,
}
