/// Input data necessary to create a signed transaction.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignInput {
    #[prost(uint32, tag="1")]
    pub chain_id: u32,
    #[prost(string, tag="2")]
    pub nonce: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub gas_price: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub gas_limit: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub amount: ::prost::alloc::string::String,
    /// Recipient's address.
    #[prost(string, tag="6")]
    pub to_address: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="7")]
    pub payload: ::prost::alloc::vec::Vec<u8>,
}
/// Transaction signing output.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignOutput {
    /// Signed and encoded transaction bytes.
    #[prost(bytes="vec", tag="1")]
    pub encoded: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub v: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub r: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="4")]
    pub s: ::prost::alloc::vec::Vec<u8>,
    /// The payload part, supplied in the input or assembled from input parameters
    #[prost(bytes="vec", tag="5")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
