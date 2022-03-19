#!/bin/bash
set -x -euo pipefail

PUSH="true"
while [[ "$#" -gt 0 ]]; do case $1 in
    --push)
        PUSH="$2"
        shift
        shift
        ;;
    *)
        echo "Unknown parameter: $1"
        exit 1
        ;;
    esac done

if [ "$PUSH" == "true" ]; then
    pushd chain-common/proto
fi

protoc --swift_opt=Visibility=Public --swift_out=../../output/ios/proto ./*.proto

pushd sign
protoc --swift_opt=Visibility=Public --swift_out=../../../output/ios/proto/sign ./*.proto
