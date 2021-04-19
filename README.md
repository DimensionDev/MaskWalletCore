# MaskWallet

## Project Structure

This library acts like a `Wallet API Service` by exposing an unified interface which receives and outputs protobuf messages.

The project structure is managed using a crate workspace, comprised by different crates:

| Crate        | Description                                                  | Status          |
| ------------ | ------------------------------------------------------------ | --------------- |
| cli          | The CLI of the library                                       | Not Implemented |
| interface    | The entry of the library, provides `request` function as the only entrance | In Progress     |
| crypto       | The collections of cryptography implementation used by other crates | In Progress     |
| wallet       | The common structs and methods of the wallet                 | In Progress     |
| chain-common | The common structs and interface of all chain implementation | In Progress     |
| chain/*      | The specific implementation of each chain                    | In Progress     |

## Usage

To `send API requests` to this library, you need to call the `request(&[u8]) -> Vec<u8>` by sending protobuf encoded requests, then decode the response using protobuf to get the actual returned value.

The supported requests could be found from the proto definition files `Api.proto` and `Param.proto` in the `interface/proto` directory.

The corresponding responses could be found from the proto definition files `Api.proto` and `Respons.proto` in the `interface/proto` directory.

## Build WebAssembly Instruction

**!!IMPORTANT**: Please notice that you could not build WebAssembly of this library on **MacOS** due to this [issue](https://github.com/DimensionDev/MaskWallet/issues/1) of compiling Secp256k1 Wasm on mac.

### Build on Ubuntu

#### Prerequisites

Following below steps to install all the required dependencies.

```
sudo apt-get update && sudo apt install cmake

sudo apt install pkg-config libssl-dev clang

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

cargo install wasm-pack
```

After successfully installing all dependencies, build the WebAssembly wasm by simply running:

```
wasm-pack build
```

