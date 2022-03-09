#!/usr/bin/env bash
#
# This script builds the Rust crate in its directory into a staticlib XCFramework for iOS.

BUILD_PROFILE="release"
FRAMEWORK_NAME="libmask_wallet_core_mobile"
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
TARGET_ROOT="$(cd ../target &>/dev/null && pwd)"
CARGO="$HOME/.cargo/bin/cargo"

cargo_build() {
    TARGET=$1
    LIBS_DIR="$TARGET_ROOT/$TARGET/$BUILD_PROFILE"
    "$CARGO" "build" --target "$TARGET"
}

# Intel iOS simulator
# CFLAGS_x86_64_apple_ios="-target x86_64-apple-ios" \
#     cargo_build x86_64-apple-ios

# Hardware iOS targets
# cargo_build aarch64-apple-ios

#
# M1 iOS simulator.
# CFLAGS_aarch64_apple_ios_sim="--target aarch64-apple-ios-sim" \
#     cargo_build aarch64-apple-ios-sim

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
cp "$THIS_DIR/MozillaRustComponents.h" "$COMMON/Headers"
cp "$REPO_ROOT/components/rc_log/ios/RustLogFFI.h" "$COMMON/Headers"
cp "$REPO_ROOT/components/viaduct/ios/RustViaductFFI.h" "$COMMON/Headers"
cp "$REPO_ROOT/components/external/glean/glean-core/ffi/glean.h" "$COMMON/Headers"
# TODO: https://github.com/mozilla/uniffi-rs/issues/1060
# it would be neat if there was a single UniFFI command that would spit out
# all of the generated headers for all UniFFIed dependencies of a given crate.
# For now we generate the Swift bindings to get the headers as a side effect,
# then delete the generated Swift code. Bleh.
# $CARGO uniffi-bindgen generate "$REPO_ROOT/components/nimbus/src/nimbus.udl" -l swift -o "$COMMON/Headers"
# $CARGO uniffi-bindgen generate "$REPO_ROOT/components/crashtest/src/crashtest.udl" -l swift -o "$COMMON/Headers"
# $CARGO uniffi-bindgen generate "$REPO_ROOT/components/fxa-client/src/fxa_client.udl" -l swift -o "$COMMON/Headers"
# $CARGO uniffi-bindgen generate "$REPO_ROOT/components/logins/src/logins.udl" -l swift -o "$COMMON/Headers"
# $CARGO uniffi-bindgen generate "$REPO_ROOT/components/autofill/src/autofill.udl" -l swift -o "$COMMON/Headers"
# $CARGO uniffi-bindgen generate "$REPO_ROOT/components/push/src/push.udl" -l swift -o "$COMMON/Headers"
# $CARGO uniffi-bindgen generate "$REPO_ROOT/components/tabs/src/tabs.udl" -l swift -o "$COMMON/Headers"
# $CARGO uniffi-bindgen generate "$REPO_ROOT/components/places/src/places.udl" -l swift -o "$COMMON/Headers"
# rm -rf "$COMMON"/Headers/*.swift

# Flesh out the framework for each architecture based on the common files.
# It's a little fiddly, because we apparently need to put all the simulator targets
# together into a single fat binary, but keep the hardware target separate.
# (TODO: we should try harder to see if we can avoid using `lipo` here, eliminating it
# would make the overall system simpler to understand).

# iOS hardware
mkdir -p "$XCFRAMEWORK_ROOT/ios-arm64"
cp -r "$COMMON" "$XCFRAMEWORK_ROOT/ios-arm64/$FRAMEWORK_NAME.framework"
cp "$TARGET_DIR/aarch64-apple-ios/$BUILD_PROFILE/$LIB_NAME" "$XCFRAMEWORK_ROOT/ios-arm64/$FRAMEWORK_NAME.framework/$FRAMEWORK_NAME"

# iOS simulator, with both platforms as a fat binary for mysterious reasons
mkdir -p "$XCFRAMEWORK_ROOT/ios-arm64_x86_64-simulator"
cp -r "$COMMON" "$XCFRAMEWORK_ROOT/ios-arm64_x86_64-simulator/$FRAMEWORK_NAME.framework"
lipo -create \
  -output "$XCFRAMEWORK_ROOT/ios-arm64_x86_64-simulator/$FRAMEWORK_NAME.framework/$FRAMEWORK_NAME" \
  "$TARGET_DIR/aarch64-apple-ios-sim/$BUILD_PROFILE/$LIB_NAME" \
  "$TARGET_DIR/x86_64-apple-ios/$BUILD_PROFILE/$LIB_NAME"

# Set up the metadata for the XCFramework as a whole.

cp "$THIS_DIR/Info.plist" "$XCFRAMEWORK_ROOT/Info.plist"
cp "$THIS_DIR/DEPENDENCIES.md" "$XCFRAMEWORK_ROOT/DEPENDENCIES.md"

rm -rf "$XCFRAMEWORK_ROOT/common"

# Zip it all up into a bundle for distribution.

(cd "$THIS_DIR" && zip -9 -r "$FRAMEWORK_NAME.xcframework.zip" "$FRAMEWORK_NAME.xcframework")
