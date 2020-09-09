#!/bin/sh
set -e -u

wasm-pack build --target nodejs --out-dir target/nodejs

cp README.md target/nodejs
cd target/nodejs
npm publish
cd -

rm -Rf target/advent_of_code_rs_bin
mkdir -p target/advent_of_code_rs_bin
cd target/advent_of_code_rs_bin

cp ../../cli.js .

# Generate package.json to get computed version:
PACKAGE_VERSION=`jq -r .version < ../../target/nodejs/package.json`
cat > package.json <<HERE
{
  "name": "advent_of_code_rs_bin",
  "version": "$PACKAGE_VERSION",
  "repository": "https://github.com/fornwall/advent-of-code-2019-rs",
  "description": "Installs advent_of_code_rs for cli usage",
  "main": "index.js",
  "scripts": {
    "test": "test"
  },
  "dependencies": {
    "advent_of_code_rs": "$PACKAGE_VERSION"
  },
  "bin": {
    "advent-of-code-rs": "./cli.js"
  },
  "author": "Fredrik Fornwall",
  "license": "MIT"
}
HERE
npm publish
