#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MwRequest {
    #[prost(oneof="mw_request::Request", tags="1")]
    pub request: ::core::option::Option<mw_request::Request>,
}
/// Nested message and enum types in `MWRequest`.
pub mod mw_request {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Request {
        #[prost(message, tag="1")]
        ParamImportPrivateKey(super::super::param::PrivateKeyStoreImportParam),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MwResponse {
    #[prost(bool, tag="1")]
    pub is_success: bool,
    #[prost(string, tag="2")]
    pub error: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub data: ::prost::alloc::string::String,
}
