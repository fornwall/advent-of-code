#!/bin/sh
# See https://github.com/rustwasm/wasm-bindgen/tree/master/examples/raytrace-parallel

set -ex

NIGHTLY_DATE=2020-10-13
NIGHTLY_TOOLCHAIN=nightly-${NIGHTLY_DATE}

# Preparations:
# rustup toolchain install $NIGHTLY_TOOLCHAIN && rustup component add --toolchain $NIGHTLY_TOOLCHAIN rust-src

# A couple of steps are necessary to get this build working which makes it slightly
# nonstandard compared to most other builds.
#
# * First, the Rust standard library needs to be recompiled with atomics
#   enabled. to do that we use Cargo's unstable `-Zbuild-std` feature.
#
# * Next we need to compile everything with the `atomics` and `bulk-memory`
#   features enabled, ensuring that LLVM will generate atomic instructions,
#   shared memory, passive segments, etc.

#RUSTFLAGS='-C target-feature=+atomics,+bulk-memory' \
  #cargo +nightly build --target wasm32-unknown-unknown --release -Z build-std=std,panic_abort

# Note the usage of `--target no-modules` here which is required for passing
# the memory import to each wasm module.
# cargo +nightly run -p wasm-bindgen-cli -- \
# TODO: Is this still required?

RUSTFLAGS='-C target-feature=+atomics,+bulk-memory' \
    rustup run $NIGHTLY_TOOLCHAIN \
    wasm-pack build \
      --dev \
      --target no-modules \
      --out-dir ../site \
      -- -Z build-std=std,panic_abort
