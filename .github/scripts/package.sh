#!/bin/bash
cp -R -v "$GITHUB_WORKSPACE/chain-common/proto" .

set -x

npx pkg-jq -i '.name = "@dimensiondev/mask-wallet-core"'
npx pkg-jq -i '.files += ["proto.js", "proto.d.ts", "proto"]'

npm install protobufjs

pushd proto
npx pbjs --out index.js --target static-module *.proto
npx pbts --out index.d.ts index.js
popd

npm pack
