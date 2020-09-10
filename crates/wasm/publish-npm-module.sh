#!/bin/bash
set -e -u

wasm-pack build --target nodejs --out-dir target/nodejs

cp README.md target/nodejs/
cp cli.js target/nodejs/

cd target/nodejs

jq -s add <(echo '{"bin":{ "advent-of-code-wasm": "./cli.js" }}') package.json

npm publish
