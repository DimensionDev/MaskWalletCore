# MaskWallet

## Project Structure

This library acts like a `Wallet API Service` by exposing an unified interface which receives and outputs protobuf messages.

The project structure is managed using a crate workspace, comprised by different crates:

| Crate        | Description                                                                | Status           |
| ------------ | -------------------------------------------------------------------------- | ---------------- |
| cli          | The CLI of the library                                                     | Not implemented  |
| interface    | The entry of the library, provides `request` function as the only entrance | Work in progress |
| crypto       | The collections of cryptography implementation used by other crates        | Work in progress |
| wallet       | The common structs and methods of the wallet                               | Work in progress |
| chain-common | The common structs and interface of all chain implementation               | Work in progress |
| chain/\*     | The specific implementation of each chain                                  | Work in progress |

## Usage

To `send API requests` to this library, you need to call the `request(&[u8]) -> Vec<u8>` by sending protobuf encoded requests,
then decode the response using protobuf to get the actual returned value.

The supported requests could be found from the proto definition files `Api.proto` and `Param.proto` in the `interface/proto` directory.

The corresponding responses could be found from the proto definition files `Api.proto` and `Respons.proto` in the `interface/proto` directory.

## New Chain Integration Checklist

- [ ] Add chain and coin info to `interface/resource/coin.json`
- [ ] Add a new crate under `chain`, e.g. to add a new chain named "mask", execute `cargo new mask --lib` in `chain` directory
- [ ] Implement `chain_common::entry::Entry` trait in the new added chain crate.
- [ ] Add new enum value to `enum Coin` in `interface/proto/Param.proto`
- [ ] Add the newly added chain to following location in `interface/src/coin.rs`

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

- [ ] Add the newly added chain `Entry` to `wallet/src/coin_dispatcher.rs` as following

```rust
pub fn get_dispatcher(coin: &Coin) -> Box<dyn Entry> {
    match coin.name.as_str() {
        "ethereum" => Box::new(EthereumEntry{}),
        // Add "${NEW_CHAIN_ID}" to the Box::new(${NEW_CHAIN_ENTRY})
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
wasm-pack build interface --target web
```
