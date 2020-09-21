#!/bin/bash
set -e -u

wasm-pack build --target nodejs --out-dir target/nodejs

cp README.md target/nodejs/
cp cli.js target/nodejs/

cd target/nodejs
cp package.json old-package.json
jq -s add <(echo '{"bin":{ "advent-of-code-wasm": "./cli.js" }}') old-package.json > package.json

cd ../../test-project
npm install
npm test

#OUTPUT=`echo 14 | npx advent-of-code-wasm 2019 1 1`
#if [ "$OUTPUT" != 3 ]; then
#	echo "Invalid output: $OUTPUT"
#	exit 1
#fi
