#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrivateKeyStoreImportParam {
    #[prost(string, tag="1")]
    pub private_key: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub password: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub encoding: ::prost::alloc::string::String,
}
