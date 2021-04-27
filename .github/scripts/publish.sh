#!/bin/bash
set -x -euo pipefail

cp -R ./chain-common/proto ./package

pushd interface
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
npx pkg-jq -i ".version = "\""${VERSION}-${BUILD_VERSION}"\"""
npm ci
pushd proto
npx pbjs --out index.js --target static-module *.proto
npx pbts --out index.d.ts index.js
popd
npm publish
