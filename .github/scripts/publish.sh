#!/bin/bash
set -x -euo pipefail

cp -R ./chain-common/proto/* ./package/proto

pushd wasm
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
OUT_PARAMS=(
	--target static-module
	--keep-case
	--no-create
	--no-verify
	--no-convert
	--no-delimited
	proto/*.proto
)
npx pbjs --out proto/index.js --wrap commonjs "${OUT_PARAMS[@]}"
npx pbjs --out proto/index.esm.js --wrap es6 --es6 "${OUT_PARAMS[@]}"
npx pbts --out proto/index.d.ts proto/index.js
npm publish
