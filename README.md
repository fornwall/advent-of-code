[![Build](https://github.com/fornwall/advent-of-code/workflows/Github%20CI/badge.svg)](https://github.com/fornwall/advent-of-code/actions?query=workflow%3A%22Github+CI%22)

# Advent of Code solutions
Solutions to [Advent of Code](https://adventofcode.com/) problems (all years, 2015-2020) in Rust. Consists of the following parts:

- [crates/core](crates/core): The solutions themselves and a command-line tool on top of them.
- [crates/grpc-server](crates/grpc-server): Server with a gRPC API using [Tonic](https://docs.rs/tonic).
- [crates/http-server](crates/http-server): Server with a HTTP API using [warp](https://github.com/seanmonstar/warp).
- [crates/fuzzing-afl](crates/fuzzing-afl): Fuzzing using [afl](https://lcamtuf.coredump.cx/afl/).
- [crates/fuzzing-hfuzz](crates/fuzzing-hfuzz): Fuzzing using [hongfuzz](https://honggfuzz.dev/).
- [crates/fuzzing-libfuzzer](crates/fuzzing-libfuzzer): Fuzzing using [libFuzzer](https://llvm.org/docs/LibFuzzer.html).
- [crates/python](crates/python): A python library wrapping the solutions using [PyO3](https://pyo3.rs/).
- [crates/wasm](crates/wasm): The solutions built as WebAssembly using [wasm-bindgen](https://rustwasm.github.io/docs/wasm-bindgen/).
  - Published to a static site at [https://aoc.fornwall.net](https://aoc.fornwall.net).
  - Published as a Node.js package.
  - Deployed to [Cloudflare Workers](https://workers.cloudflare.com/).
  - Deployed to [Netlify Functions](https://www.netlify.com/products/functions/).
