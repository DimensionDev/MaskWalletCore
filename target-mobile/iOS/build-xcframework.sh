#!/usr/bin/env bash
#
# This script builds the Rust crate in its directory into a staticlib XCFramework for iOS.
set -x -euo pipefail

BUILD_PROFILE="release"
FRAMEWORK_NAME="MaskWalletCoreMobile"
LIB_NAME="libmask_wallet_core_mobile.a"


# eg. sh build-xcframework.sh --build-profile release --framework-name MaskWallet
while [[ "$#" -gt 0 ]]; do case $1 in
    --build-profile)
        BUILD_PROFILE="$2"
        shift
        shift
        ;;
    --framework-name)
        FRAMEWORK_NAME="$2"
        shift
        shift
        ;;
    *)
        echo "Unknown parameter: $1"
        exit 1
        ;;
    esac done

####
##
## 1) Build the rust code individually for each target architecture.
##
####

THIS_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" &>/dev/null && pwd)"
TARGET_ROOT="$(cd ../../target &>/dev/null && pwd)"
CARGO="$HOME/.cargo/bin/cargo"
LIBS_DIR=""
HEADERS_DIR=""$(cd .. &>/dev/null && pwd)""

# Intel iOS simulator
cargo build --target x86_64-apple-ios --"$BUILD_PROFILE"

# Hardware iOS targets
cargo build --target aarch64-apple-ios --"$BUILD_PROFILE"

# M1 iOS simulator.
cargo build --target aarch64-apple-ios-sim --"$BUILD_PROFILE"

####
##
## 2) Stitch the individual builds together an XCFramework bundle.
##
####

XCFRAMEWORK_ROOT="$THIS_DIR/$FRAMEWORK_NAME.xcframework"

# Start from a clean slate.

rm -rf "$XCFRAMEWORK_ROOT"

# Build the directory structure right for an individual framework.
# Most of this doesn't change between architectures.

COMMON="$XCFRAMEWORK_ROOT/common/$FRAMEWORK_NAME.framework"

mkdir -p "$COMMON/Modules"
cp "$THIS_DIR/module.modulemap" "$COMMON/Modules/"

mkdir -p "$COMMON/Headers"
cp "$HEADERS_DIR/$FRAMEWORK_NAME.h" "$COMMON/Headers"

# Flesh out the framework for each architecture based on the common files.
# It's a little fiddly, because we apparently need to put all the simulator targets
# together into a single fat binary, but keep the hardware target separate.
# (TODO: we should try harder to see if we can avoid using `lipo` here, eliminating it
# would make the overall system simpler to understand).

# iOS hardware
mkdir -p "$XCFRAMEWORK_ROOT/ios-arm64"
cp -r "$COMMON" "$XCFRAMEWORK_ROOT/ios-arm64/$FRAMEWORK_NAME.framework"
cp "$TARGET_ROOT/aarch64-apple-ios/$BUILD_PROFILE/$LIB_NAME" "$XCFRAMEWORK_ROOT/ios-arm64/$FRAMEWORK_NAME.framework/$FRAMEWORK_NAME"

# iOS simulator, with both platforms as a fat binary for mysterious reasons
mkdir -p "$XCFRAMEWORK_ROOT/ios-arm64_x86_64-simulator"
cp -r "$COMMON" "$XCFRAMEWORK_ROOT/ios-arm64_x86_64-simulator/$FRAMEWORK_NAME.framework"
lipo -create \
    -output "$XCFRAMEWORK_ROOT/ios-arm64_x86_64-simulator/$FRAMEWORK_NAME.framework/$FRAMEWORK_NAME" \
    "$TARGET_ROOT/aarch64-apple-ios-sim/$BUILD_PROFILE/$LIB_NAME" \
    "$TARGET_ROOT/x86_64-apple-ios/$BUILD_PROFILE/$LIB_NAME"

# Set up the metadata for the XCFramework as a whole.

cp "$THIS_DIR/Info.plist" "$XCFRAMEWORK_ROOT/Info.plist"

rm -rf "$XCFRAMEWORK_ROOT/common"

# Zip it all up into a bundle for distribution.

(cd "$THIS_DIR" && zip -9 -r "$FRAMEWORK_NAME.xcframework.zip" "$FRAMEWORK_NAME.xcframework")

PROTO_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" &>/dev/null && pwd)/Protos"
PROTO_SIGN_DIR="$PROTO_DIR/Sign"

rm -rf "$PROTO_DIR"

mkdir "$PROTO_DIR"
mkdir "$PROTO_SIGN_DIR"

PROTO_ROOT="$(cd ../.. &>/dev/null && pwd)"

pushd "$PROTO_ROOT/chain-common/proto"
protoc --swift_opt=Visibility=Public --swift_out="$PROTO_DIR" ./*.proto

pushd sign
protoc --swift_opt=Visibility=Public --swift_out="$PROTO_SIGN_DIR" ./*.proto
