#!/bin/bash
set -e -u

cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --target nodejs --out-dir target/nodejs ../../target/wasm32-unknown-unknown/release/advent_of_code_wasm.wasm
which wasm-opt
wasm-opt --version
wasm-opt -O3 -o target/nodejs/advent_of_code_wasm_bg.wasm-optimized target/nodejs/advent_of_code_wasm_bg.wasm
mv target/nodejs/advent_of_code_wasm_bg.wasm-optimized target/nodejs/advent_of_code_wasm_bg.wasm

cp README.md target/nodejs/
cp cli.js target/nodejs/

PACKAGE_VERSION=`cargo metadata --format-version 1 | jq -r '.packages[0].version'`

cd target/nodejs

cat > package.json << EOF
{
  "bin": {
    "advent-of-code-wasm": "./cli.js"
  },
  "name": "advent-of-code-wasm",
  "collaborators": [
    "Fredrik Fornwall <fredrik@fornwall.net>"
  ],
  "description": "Solutions to Advent of Code written in Rust",
  "version": "$PACKAGE_VERSION",
  "license": "MIT",
  "repository": {
    "type": "git",
    "url": "https://github.com/fornwall/advent-of-code"
  },
  "files": [
    "advent_of_code_wasm_bg.wasm",
    "advent_of_code_wasm.js",
    "advent_of_code_wasm_bg.js",
    "advent_of_code_wasm.d.ts",
    "LICENSE.txt"
  ],
  "main": "advent_of_code_wasm.js",
  "types": "advent_of_code_wasm.d.ts"
}
EOF


cd ../../test-project
npm install
npm test

#OUTPUT=`echo 14 | npx advent-of-code-wasm 2019 1 1`
#if [ "$OUTPUT" != 3 ]; then
#	echo "Invalid output: $OUTPUT"
#	exit 1
#fi
