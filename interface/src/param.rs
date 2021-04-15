#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Coin {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(int32, tag="3")]
    pub coin_id: i32,
    #[prost(string, tag="4")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(int32, tag="5")]
    pub decimal: i32,
    #[prost(string, tag="6")]
    pub blockchain: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub derivation_path: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub curve: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub public_key_type: ::prost::alloc::string::String,
}
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
    #[prost(message, optional, tag="5")]
    pub coin: ::core::option::Option<Coin>,
}
