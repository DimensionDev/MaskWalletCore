#!/bin/bash
set -x -euo pipefail

cp -R ./chain-common/proto/* ./package/proto

pushd target-wasm
wasm-pack build --target nodejs \
	--out-name index \
	--out-dir ../package/node
wasm-pack build --target bundler \
	--out-name index \
	--out-dir ../package/bundle
wasm-pack build --target web \
	--out-name index \
	--out-dir ../package/web
popd

pushd package
VERSION=$(npx pkg-jq -r '.version' node)
npx pkg-jq -i ".version = \"""$VERSION"-"$BUILD_VERSION""\""
npm ci
npx protoc \
	--ts_out proto \
	--ts_opt long_type_string \
	--ts_opt optimize_code_size \
	--proto_path proto \
	proto/*.proto
pushd proto
npx --no-install --package typescript tsc
popd
npm publish
