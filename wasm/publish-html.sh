#!/bin/sh
set -e -u

wasm-pack build --target browser --out-dir target/browser

cd wasm/html
rm -Rf node_modules package-lock.json
npm install
webpack --config webpack.config.js

echo "wasm/html/dist/ has been created"
echo "This is published at https://fornwall.net/advent-of-code-2019/"
