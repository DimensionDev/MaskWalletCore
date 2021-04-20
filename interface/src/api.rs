#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MwRequest {
    #[prost(oneof="mw_request::Request", tags="1, 2, 3")]
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
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MwResponse {
    #[prost(bool, tag="1")]
    pub is_success: bool,
    #[prost(oneof="mw_response::Response", tags="2, 3, 4, 5")]
    pub response: ::core::option::Option<mw_response::Response>,
}
/// Nested message and enum types in `MWResponse`.
pub mod mw_response {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        #[prost(message, tag="2")]
        Error(super::MwResponseError),
        #[prost(message, tag="3")]
        RespImportPrivateKey(super::super::param::PrivateKeyStoreImportResp),
        #[prost(message, tag="4")]
        RespGetStoredKeyAccountCount(super::super::param::GetStoredKeyAccountCountResp),
        #[prost(message, tag="5")]
        RespGetStoredKeyAccount(super::super::param::GetStoredKeyAccountResp),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MwResponseError {
    #[prost(string, tag="1")]
    pub error_code: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub error_msg: ::prost::alloc::string::String,
}
