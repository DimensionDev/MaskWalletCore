/// Input data necessary to create a signed transaction.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignInput {
    /// Chain identifier (256-bit number)
    #[prost(bytes="vec", tag="1")]
    pub chain_id: ::prost::alloc::vec::Vec<u8>,
    /// Nonce (256-bit number)
    #[prost(bytes="vec", tag="2")]
    pub nonce: ::prost::alloc::vec::Vec<u8>,
    /// Gas price (256-bit number)
    #[prost(bytes="vec", tag="3")]
    pub gas_price: ::prost::alloc::vec::Vec<u8>,
    /// Gas limit (256-bit number)
    #[prost(bytes="vec", tag="4")]
    pub gas_limit: ::prost::alloc::vec::Vec<u8>,
    /// Recipient's address.
    #[prost(string, tag="5")]
    pub to_address: ::prost::alloc::string::String,
    /// Private key.
    #[prost(bytes="vec", tag="6")]
    pub private_key: ::prost::alloc::vec::Vec<u8>,
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
