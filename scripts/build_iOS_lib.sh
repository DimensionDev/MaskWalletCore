#!/bin/bash
set -x -euo pipefail

pushd target-mobile
cbindgen src/lib.rs -l c > rust.h
cargo lipo --release
popd

if [[ -d output ]]; then
    rm -rf output
fi

mkdir output
mkdir output/ios
mkdir output/ios/proto
mkdir output/ios/proto/sign

mv ./target-mobile/rust.h ./output/ios/rust.h
cp ./target/universal/release/libmask_wallet_core_mobile.a ./output/ios/

pushd chain-common/proto
protoc --swift_opt=Visibility=Public --swift_out=../../output/ios/proto ./*.proto

pushd sign
protoc --swift_opt=Visibility=Public --swift_out=../../../output/ios/proto/sign ./*.proto
