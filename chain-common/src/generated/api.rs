/// Get the version code of MaskWalletCore library
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetVersionParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetVersionResp {
    #[prost(string, tag="1")]
    pub version: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StoredKeyInfo {
    #[prost(string, tag="1")]
    pub id: ::prost::alloc::string::String,
    /// The unique hash of the StoredKey, users could check whether two StoredKeys are same by comparing the hashes
    #[prost(string, tag="2")]
    pub hash: ::prost::alloc::string::String,
    #[prost(enumeration="StoredKeyType", tag="3")]
    pub r#type: i32,
    /// Raw data of the StoredKey, used in requests required an existing StoredKey
    #[prost(bytes="vec", tag="4")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StoredKeyAccountInfo {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub derivation_path: ::prost::alloc::string::String,
    /// Coin id
    #[prost(string, tag="4")]
    pub coin: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub extended_public_key: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EncryptOption {
    #[prost(enumeration="encrypt_option::Version", tag="1")]
    pub version: i32,
}
/// Nested message and enum types in `EncryptOption`.
pub mod encrypt_option {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Version {
        V37 = 0,
        V38 = 1,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Jwk {
    #[prost(string, tag="1")]
    pub crv: ::prost::alloc::string::String,
    #[prost(bool, tag="3")]
    pub ext: bool,
    #[prost(string, tag="4")]
    pub x: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub y: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="6")]
    pub key_ops: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag="7")]
    pub kty: ::prost::alloc::string::String,
    #[prost(string, optional, tag="8")]
    pub d: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AesJwk {
    #[prost(string, tag="1")]
    pub alg: ::prost::alloc::string::String,
    #[prost(bool, tag="2")]
    pub ext: bool,
    #[prost(string, tag="3")]
    pub k: ::prost::alloc::string::String,
    #[prost(string, repeated, tag="4")]
    pub key_ops: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag="5")]
    pub kty: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct E2eEncryptParam {
    #[prost(bytes="vec", tag="1")]
    pub local_key_data: ::prost::alloc::vec::Vec<u8>,
    #[prost(map="string, bytes", tag="2")]
    pub target: ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", tag="3")]
    pub author_private_key: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Coin {
    Ethereum = 0,
    Polkadot = 1,
    Solana = 2,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum StoredKeyType {
    PrivateKey = 0,
    Mnemonic = 1,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum StoredKeyImportType {
    PrivateKeyImportType = 0,
    MnemonicImportType = 1,
    KeyStoreJsonImportType = 2,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum StoredKeyExportType {
    PrivateKeyExportType = 0,
    MnemonicExportType = 1,
    KeyStoreJsonExportType = 2,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Curve {
    Secp256k1 = 0,
    Ed25519 = 1,
}
/// Create a new account to the StoredKey at specific derivation path. Fail if the StoredKey is not a Hd StoredKey
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateStoredKeyNewAccountAtPathParam {
    #[prost(bytes="vec", tag="1")]
    pub stored_key_data: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    #[prost(enumeration="Coin", tag="3")]
    pub coin: i32,
    #[prost(string, tag="4")]
    pub derivation_path: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub password: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateStoredKeyNewAccountAtPathResp {
    #[prost(message, optional, tag="1")]
    pub account: ::core::option::Option<StoredKeyAccountInfo>,
    #[prost(message, optional, tag="2")]
    pub stored_key: ::core::option::Option<StoredKeyInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetKeyStoreSupportExportTypeParam {
    #[prost(enumeration="Coin", tag="1")]
    pub coin: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetKeyStoreSupportExportTypeResp {
    #[prost(enumeration="StoredKeyImportType", repeated, tag="1")]
    pub r#type: ::prost::alloc::vec::Vec<i32>,
}
/// Export the private key of StoredKey
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportKeyStorePrivateKeyParam {
    #[prost(bytes="vec", tag="1")]
    pub stored_key_data: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="2")]
    pub password: ::prost::alloc::string::String,
    #[prost(enumeration="Coin", tag="3")]
    pub coin: i32,
}
/// Export the private key of StoredKey at specific derivation path. Fail if the StoredKey is not a Hd StoredKey
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportKeyStorePrivateKeyOfPathParam {
    #[prost(bytes="vec", tag="1")]
    pub stored_key_data: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="2")]
    pub password: ::prost::alloc::string::String,
    #[prost(enumeration="Coin", tag="3")]
    pub coin: i32,
    #[prost(string, tag="4")]
    pub derivation_path: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportKeyStorePrivateKeyResp {
    #[prost(string, tag="1")]
    pub private_key: ::prost::alloc::string::String,
}
/// Export the mnemonic of the StoredKey. Fail if the StoredKey is not a Hd StoredKey
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportKeyStoreMnemonicParam {
    #[prost(bytes="vec", tag="1")]
    pub stored_key_data: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="2")]
    pub password: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportKeyStoreMnemonicResp {
    #[prost(string, tag="1")]
    pub mnemonic: ::prost::alloc::string::String,
}
/// Export the V3 KeyStoreJSON string of the StoredKey at the specific address
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportKeyStoreJsonOfAddressParam {
    #[prost(bytes="vec", tag="1")]
    pub stored_key_data: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="2")]
    pub password: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub new_password: ::prost::alloc::string::String,
    #[prost(enumeration="Coin", tag="4")]
    pub coin: i32,
    #[prost(string, tag="5")]
    pub address: ::prost::alloc::string::String,
}
/// Export the V3 KeyStoreJSON string of the StoredKey at specific derivation path. Fail if the StoredKey is not a Hd StoredKey
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportKeyStoreJsonOfPathParam {
    #[prost(bytes="vec", tag="1")]
    pub stored_key_data: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="2")]
    pub password: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub new_password: ::prost::alloc::string::String,
    #[prost(enumeration="Coin", tag="4")]
    pub coin: i32,
    #[prost(string, tag="5")]
    pub derivation_path: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportKeyStoreJsonResp {
    #[prost(string, tag="1")]
    pub json: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetKeyStoreSupportImportTypeParam {
    #[prost(enumeration="Coin", tag="1")]
    pub coin: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetKeyStoreSupportImportTypeResp {
    #[prost(enumeration="StoredKeyImportType", repeated, tag="1")]
    pub r#type: ::prost::alloc::vec::Vec<i32>,
}
/// Get information from StoredKey raw data
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoadStoredKeyParam {
    #[prost(bytes="vec", repeated, tag="1")]
    pub data: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoadStoredKeyResp {
    #[prost(message, repeated, tag="1")]
    pub stored_keys: ::prost::alloc::vec::Vec<StoredKeyInfo>,
}
/// Create a new StoredKey with random generated mnemonic, this request will NOT create any account
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateStoredKeyParam {
    #[prost(string, tag="2")]
    pub password: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateStoredKeyResp {
    #[prost(message, optional, tag="1")]
    pub stored_key: ::core::option::Option<StoredKeyInfo>,
    /// The mnemonic of the new generated StoredKey
    #[prost(string, tag="2")]
    pub mnemonic: ::prost::alloc::string::String,
}
/// Create a StoredKey with private key, please aware that this request DOES create an account with the specific coin
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportPrivateStoredKeyParam {
    /// Hex encoded private key string
    #[prost(string, tag="1")]
    pub private_key: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub password: ::prost::alloc::string::String,
    /// The name of created ACCOUNT
    #[prost(string, tag="3")]
    pub name: ::prost::alloc::string::String,
    #[prost(enumeration="Coin", tag="4")]
    pub coin: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportPrivateStoredKeyResp {
    #[prost(message, optional, tag="1")]
    pub stored_key: ::core::option::Option<StoredKeyInfo>,
}
/// Create a new StoredKey with given mnemonic, this request will NOT create any account
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportMnemonicStoredKeyParam {
    #[prost(string, tag="1")]
    pub mnemonic: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub password: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportMnemonicStoredKeyResp {
    #[prost(message, optional, tag="1")]
    pub stored_key: ::core::option::Option<StoredKeyInfo>,
}
/// Create a new StoredKey with JSON, please aware that this request DOES create an account with the specific coin
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportJsonStoredKeyParam {
    #[prost(string, tag="1")]
    pub json: ::prost::alloc::string::String,
    /// The name of created ACCOUNT
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    /// Password to decrypt the KeyStoreJson
    #[prost(string, tag="3")]
    pub key_store_json_password: ::prost::alloc::string::String,
    /// Password of the created StoredKey
    #[prost(string, tag="4")]
    pub password: ::prost::alloc::string::String,
    #[prost(enumeration="Coin", tag="5")]
    pub coin: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportJsonStoredKeyResp {
    #[prost(message, optional, tag="1")]
    pub stored_key: ::core::option::Option<StoredKeyInfo>,
}
/// Update the password of an exisiting StoredKey
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateStoredKeyPasswordParam {
    #[prost(bytes="vec", tag="1")]
    pub stored_key_data: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="2")]
    pub old_password: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub new_password: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateStoredKeyPasswordResp {
    #[prost(message, optional, tag="1")]
    pub stored_key: ::core::option::Option<StoredKeyInfo>,
}
/// Generate a random mnemonic
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateMnemonicParam {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateMnemonicResp {
    /// The generated random mnemonic
    #[prost(string, tag="1")]
    pub mnemonic: ::prost::alloc::string::String,
}
/// Sign a transaction
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignTransactionParam {
    #[prost(bytes="vec", tag="1")]
    pub stored_key_data: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="2")]
    pub derivation_path: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub password: ::prost::alloc::string::String,
    #[prost(enumeration="Coin", tag="4")]
    pub coin: i32,
    #[prost(oneof="sign_transaction_param::Input", tags="5")]
    pub input: ::core::option::Option<sign_transaction_param::Input>,
}
/// Nested message and enum types in `SignTransactionParam`.
pub mod sign_transaction_param {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Input {
        #[prost(message, tag="5")]
        SignInput(super::super::ethereum::SignInput),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignTransactionResp {
    #[prost(oneof="sign_transaction_resp::Output", tags="1")]
    pub output: ::core::option::Option<sign_transaction_resp::Output>,
}
/// Nested message and enum types in `SignTransactionResp`.
pub mod sign_transaction_resp {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Output {
        #[prost(message, tag="1")]
        SignOutput(super::super::ethereum::SignOutput),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PasswordValidationParam {
    #[prost(bytes="vec", tag="1")]
    pub stored_key_data: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="2")]
    pub password: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressValidationParam {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    #[prost(enumeration="Coin", tag="2")]
    pub coin: i32,
}
/// Get the version code of MaskWalletCore library
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidateParam {
    #[prost(oneof="validate_param::Input", tags="1, 2, 3, 4, 5")]
    pub input: ::core::option::Option<validate_param::Input>,
}
/// Nested message and enum types in `ValidateParam`.
pub mod validate_param {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Input {
        #[prost(string, tag="1")]
        PrivateKey(::prost::alloc::string::String),
        #[prost(string, tag="2")]
        Mnemonic(::prost::alloc::string::String),
        #[prost(string, tag="3")]
        KeyStoreJson(::prost::alloc::string::String),
        #[prost(message, tag="4")]
        StoredKeyPassword(super::PasswordValidationParam),
        #[prost(message, tag="5")]
        AddressValidationParam(super::AddressValidationParam),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidateResp {
    #[prost(bool, tag="1")]
    pub valid: bool,
}
/// Generate a persona
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PersonaGenerationParam {
    #[prost(string, tag="1")]
    pub mnemonic: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub password: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub path: ::prost::alloc::string::String,
    #[prost(enumeration="Curve", tag="4")]
    pub curve: i32,
    #[prost(message, optional, tag="5")]
    pub option: ::core::option::Option<EncryptOption>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PersonaGenerationResp {
    #[prost(string, tag="1")]
    pub identifier: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub private_key: ::core::option::Option<Jwk>,
    #[prost(message, optional, tag="3")]
    pub public_key: ::core::option::Option<Jwk>,
    #[prost(message, optional, tag="4")]
    pub local_key: ::core::option::Option<AesJwk>,
    #[prost(message, optional, tag="5")]
    pub option: ::core::option::Option<EncryptOption>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PostEncryptionParam {
    #[prost(enumeration="encrypt_option::Version", tag="1")]
    pub version: i32,
    #[prost(bool, tag="2")]
    pub is_plublic: bool,
    #[prost(string, tag="3")]
    pub content: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub network: ::prost::alloc::string::String,
    #[prost(bytes="vec", optional, tag="5")]
    pub author_public_key_data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(string, optional, tag="6")]
    pub author_user_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(enumeration="PublicKeyAlgorithm", optional, tag="7")]
    pub author_public_key_algr: ::core::option::Option<i32>,
    #[prost(message, optional, tag="8")]
    pub param: ::core::option::Option<E2eEncryptParam>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct E2eEncryptionResult {
    #[prost(bytes="vec", optional, tag="1")]
    pub iv: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(bytes="vec", tag="2")]
    pub encrypted_post_key_data: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", optional, tag="3")]
    pub ephemeral_public_key_data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PostEncryptedResp {
    #[prost(string, tag="1")]
    pub content: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub post_identifier: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="3")]
    pub post_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(map="string, message", tag="4")]
    pub results: ::std::collections::HashMap<::prost::alloc::string::String, E2eEncryptionResult>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PublicKeyAlgorithm {
    Ed25519Algr = 0,
    Secp256p1Algr = 1,
    Secp256k1Algr = 2,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MwRequest {
    #[prost(oneof="mw_request::Request", tags="1, 2, 3, 4, 5, 10, 13, 14, 15, 16, 17, 18, 20, 21, 22, 23, 24, 25, 26, 27")]
    pub request: ::core::option::Option<mw_request::Request>,
}
/// Nested message and enum types in `MWRequest`.
pub mod mw_request {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Request {
        #[prost(message, tag="1")]
        ParamLoadStoredKey(super::LoadStoredKeyParam),
        #[prost(message, tag="2")]
        ParamCreateStoredKey(super::CreateStoredKeyParam),
        #[prost(message, tag="3")]
        ParamImportPrivateKey(super::ImportPrivateStoredKeyParam),
        #[prost(message, tag="4")]
        ParamImportMnemonic(super::ImportMnemonicStoredKeyParam),
        #[prost(message, tag="5")]
        ParamImportJson(super::ImportJsonStoredKeyParam),
        #[prost(message, tag="10")]
        ParamCreateAccountOfCoinAtPath(super::CreateStoredKeyNewAccountAtPathParam),
        #[prost(message, tag="13")]
        ParamExportPrivateKey(super::ExportKeyStorePrivateKeyParam),
        #[prost(message, tag="14")]
        ParamExportPrivateKeyOfPath(super::ExportKeyStorePrivateKeyOfPathParam),
        #[prost(message, tag="15")]
        ParamExportMnemonic(super::ExportKeyStoreMnemonicParam),
        #[prost(message, tag="16")]
        ParamExportKeyStoreJsonOfAddress(super::ExportKeyStoreJsonOfAddressParam),
        #[prost(message, tag="17")]
        ParamExportKeyStoreJsonOfPath(super::ExportKeyStoreJsonOfPathParam),
        #[prost(message, tag="18")]
        ParamUpdateKeyStorePassword(super::UpdateStoredKeyPasswordParam),
        #[prost(message, tag="20")]
        ParamSignTransaction(super::SignTransactionParam),
        #[prost(message, tag="21")]
        ParamGetVersion(super::GetVersionParam),
        #[prost(message, tag="22")]
        ParamValidation(super::ValidateParam),
        #[prost(message, tag="23")]
        ParamGetStoredKeyImportType(super::GetKeyStoreSupportImportTypeParam),
        #[prost(message, tag="24")]
        ParamGetStoredKeyExportType(super::GetKeyStoreSupportExportTypeParam),
        #[prost(message, tag="25")]
        ParamGenerateMnemonic(super::GenerateMnemonicParam),
        #[prost(message, tag="26")]
        ParamGeneratePersona(super::PersonaGenerationParam),
        #[prost(message, tag="27")]
        ParamPostEncryption(super::PostEncryptionParam),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MwResponse {
    #[prost(oneof="mw_response::Response", tags="1, 2, 3, 4, 5, 6, 11, 14, 15, 16, 17, 19, 20, 21, 22, 23, 24, 25, 26")]
    pub response: ::core::option::Option<mw_response::Response>,
}
/// Nested message and enum types in `MWResponse`.
pub mod mw_response {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        #[prost(message, tag="1")]
        Error(super::MwResponseError),
        #[prost(message, tag="2")]
        RespLoadStoredKey(super::LoadStoredKeyResp),
        #[prost(message, tag="3")]
        RespCreateStoredKey(super::CreateStoredKeyResp),
        #[prost(message, tag="4")]
        RespImportPrivateKey(super::ImportPrivateStoredKeyResp),
        #[prost(message, tag="5")]
        RespImportMnemonic(super::ImportMnemonicStoredKeyResp),
        #[prost(message, tag="6")]
        RespImportJson(super::ImportJsonStoredKeyResp),
        #[prost(message, tag="11")]
        RespCreateAccountOfCoinAtPath(super::CreateStoredKeyNewAccountAtPathResp),
        #[prost(message, tag="14")]
        RespExportPrivateKey(super::ExportKeyStorePrivateKeyResp),
        #[prost(message, tag="15")]
        RespExportMnemonic(super::ExportKeyStoreMnemonicResp),
        #[prost(message, tag="16")]
        RespExportKeyStoreJson(super::ExportKeyStoreJsonResp),
        #[prost(message, tag="17")]
        RespUpdateKeyStorePassword(super::UpdateStoredKeyPasswordResp),
        #[prost(message, tag="19")]
        RespSignTransaction(super::SignTransactionResp),
        #[prost(message, tag="20")]
        RespGetVersion(super::GetVersionResp),
        #[prost(message, tag="21")]
        RespValidate(super::ValidateResp),
        #[prost(message, tag="22")]
        RespGetStoredKeyImportType(super::GetKeyStoreSupportImportTypeResp),
        #[prost(message, tag="23")]
        RespGetStoredKeyExportType(super::GetKeyStoreSupportExportTypeResp),
        #[prost(message, tag="24")]
        RespGenerateMnemonic(super::GenerateMnemonicResp),
        #[prost(message, tag="25")]
        RespGeneratePersona(super::PersonaGenerationResp),
        #[prost(message, tag="26")]
        RespPostEncryption(super::PostEncryptedResp),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MwResponseError {
    #[prost(string, tag="1")]
    pub error_code: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub error_msg: ::prost::alloc::string::String,
}
