# MaskWallet

## Project Structure

This library acts like a `Wallet API Service` by exposing an unified interface which receives and outputs protobuf messages.

The project structure is managed using a crate workspace, comprised by different crates:

| Crate         | Description                                                  | Dev Status      | Audit Status |
| ------------- | ------------------------------------------------------------ | --------------- | ------------ |
| cli           | The CLI of the library                                       | Not implemented | N/A          |
| interface     | The entry of the library, provides `request` function as the only entrance | Done            | Not Audited  |
| crypto        | The collections of cryptography implementation used by other crates | Done            | Not Audited  |
| wallet        | The common structs and methods of the wallet                 | Done            | Not Audited  |
| chain-common  | The common structs and interface of all chain implementation, including the proto definition files | Done            | Not Audited  |
| chain/\*      | The specific implementation of each chain, only Ethereum supported for now | Done            | Not Audited  |
| package       | The npm library wrapper                                      | Done            | N/A          |
| target-wasm   | The exposed interface for WebAssembly                        | Done            | Not Audited  |
| target-mobile | The exposed interface for iOS & Android                      | Done            | Not Audited  |

## Usage

The definition of supported requests and corresponding responses could be found from the proto definition files `Api.proto` and `Response.proto` in the `chain-common/proto` directory.

### For Wasm

To `send API requests` to this library, you need to call the `request(&[u8]) -> Vec<u8>` by sending protobuf encoded requests,
then decode the response using protobuf to get the actual returned value.

### For iOS

#### Crate And Target Requirements

* Make sure crates `cargo-lipo` have been installded.
* `rust-std`s for `aarch64-apple-ios`, `aarch64-apple-ios-sim` and `x86_64-apple-ios` are downloaded (can be downloaded by excuting `rustup target add x86_64-apple-ios`, `rustup target add aarch64-apple-ios-sim` or `rustup target add aarch64-apple-ios`).
* Install `swift-protobuf` via `brew install swift-protobuf`.

#### Static lib

1. Execute `sh build_iOS_lib.sh` under `scripts` directory.
2. Add the generated `proto`, `MaskWalletCoreMobile.h` and `libmask_wallet_core_mobile.a` in `scripts/ios` directory to your iOS project.

#### XCFramework

Excute `sh build-xcframework.sh` in `targe-mobile/iOS` directory, `MaskWalletCoreMobile.xcframework` and `Protos` will be generated.

### For Android

In development

## New Chain Integration Checklist

* [ ] Add chain and coin info to `interface/resource/coin.json`.

* [ ] Add a new crate under `chain`, e.g. to add a new chain named "mask", execute `cargo new mask --lib` in `chain` directory.

* [ ] Implement `chain_common::entry::Entry` trait in the new added chain crate.

* [ ] Add new enum value to `enum Coin` in `chain-common/proto/Param.proto`.

* [ ] Add the newly added chain to following location in `chain-common/src/coin.rs`.

```rust
impl ToString for CoinType {
    fn to_string(&self) -> String {
        match self {
            CoinType::Ethereum => "ethereum".to_owned(),
            CoinType::Polkadot => "polkadot".to_owned(),
            // Add the new chain here to return the `id`
        }
    }
}
```

* [ ] Add the newly added chain `Entry` to `wallet/src/coin_dispatcher.rs` as following.

```rust
pub fn get_entry(coin: &Coin) -> Result<Box<dyn Entry>, Error> {
    let coin_proto_type = ProtoCoinType::from_str(&coin.name)?;
    match coin_proto_type {
        ProtoCoinType::Ethereum => Ok(Box::new(EthereumEntry {})),
        _ => Err(Error::ChainError(ChainError::NotSupportedCoin)),
    }
}
```

## Build WebAssembly Instruction

**!!IMPORTANT**:
Please notice that you could not build WebAssembly of
this library on **MacOS**zdue to this [issue](https://github.com/DimensionDev/MaskWallet/issues/1)
of compiling Secp256k1 Wasm on mac.

### Build on Ubuntu 20.04

#### Pre-requirements

To build the wasm using `wasm-pack` on Ubuntu, please ensure you are using Ubuntu **20.04**, not Ubuntu 18.
Following below steps to install all the required dependencies.

```bash
sudo apt update
sudo apt install cmake
sudo apt install pkg-config libssl-dev clang

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

source ~/.cargo/env

cargo install wasm-pack
```

After successfully installing all dependencies, build the WebAssembly wasm by simply running:

```bash
wasm-pack build target-wasm --target web
```
