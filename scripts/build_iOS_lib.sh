#!/bin/bash
set -x -euo pipefail

pushd target-mobile
cbindgen src/lib.rs -l c >MaskWalletCoreMobile.h
cargo lipo --release
popd

if [[ -d output ]]; then
    rm -rf output
fi

mkdir output
mkdir output/ios
mkdir output/ios/proto
mkdir output/ios/proto/sign

mv ./target-mobile/MaskWalletCoreMobile.h ./output/ios/MaskWalletCoreMobile.h
cp ./target/universal/release/libmask_wallet_core_mobile.a ./output/ios/

sh ./scripts/proto.sh
