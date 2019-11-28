#!/bin/sh
set -e

# wasm-pack takes care of this, but it's intestering to perform steps manually.

cargo build \
	--release \
	--lib \
	--target wasm32-unknown-unknown

BUILT_WASM=target/wasm32-unknown-unknown/release/advent_of_code_rs.wasm
OPTIMIZED_WASM=target/wasm32-unknown-unknown/release/advent_of_code_rs-optimized.wasm

echo "After building:"
ls -lha $BUILT_WASM

wasm-opt -Oz -o $OPTIMIZED_WASM $BUILT_WASM
echo "After wasm-opt:"
ls -lha $OPTIMIZED_WASM

wasm-gc $OPTIMIZED_WASM
echo "After wasm-gc:"
ls -lha $OPTIMIZED_WASM
