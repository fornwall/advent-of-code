[![Crates.io](https://img.shields.io/crates/v/advent-of-code.svg)](https://crates.io/crates/advent-of-code)
[![Docker Hub](https://img.shields.io/docker/v/fredrikfornwall/advent-of-code.svg?label=docker)](https://hub.docker.com/r/fredrikfornwall/advent-of-code)
[![npm](https://img.shields.io/npm/v/advent-of-code-wasm.svg)](https://www.npmjs.com/package/advent-of-code-wasm)
[![PyPi](https://img.shields.io/pypi/v/advent-of-code.svg)](https://pypi.org/project/advent-of-code/)

[![Build](https://github.com/fornwall/advent-of-code/workflows/Github%20CI/badge.svg)](https://github.com/fornwall/advent-of-code/actions?query=workflow%3A%22Github+CI%22)
[![Netlify Status](https://api.netlify.com/api/v1/badges/8cb47a76-7cd7-4545-9f10-56ba075c8e41/deploy-status)](https://app.netlify.com/sites/mystifying-blackwell-9e705f/deploys)

# Advent of Code solutions
Solutions to [Advent of Code](https://adventofcode.com/) problems in Rust. Consists of the following parts:

- [crates/core](crates/core): The solutions themselves and a command-line tool on top of them.
- [crates/grpc-server](crates/grpc-server): Server with a gRPC API using [tonic](https://docs.rs/tonic).
- [crates/http-server](crates/http-server): Server with a HTTP API using [warp](https://github.com/seanmonstar/warp).
- [crates/fuzzing-afl](crates/fuzzing-afl): Fuzzing of the solutions using [afl](https://lcamtuf.coredump.cx/afl/).
- [crates/fuzzing-hfuzz](crates/fuzzing-hfuzz): Fuzzing of the solutions using [hongfuzz](https://honggfuzz.dev/).
- [crates/fuzzing-libfuzzer](crates/fuzzing-libfuzzer): Fuzzing of the solutions using [libFuzzer](https://llvm.org/docs/LibFuzzer.html).
- [crates/python](crates/python): A python library wrapping the solutions using [PyO3](https://pyo3.rs/).
- [crates/wasm](crates/wasm): The solutions built as WebAssembly.
  - Published to a static site at [https://aoc.fornwall.net](https://aoc.fornwall.net).
  - Published as a Node.js package.
  - Deployed to [Cloudflare Workers](https://workers.cloudflare.com/).
  - Deployed to [Netlify Functions](https://www.netlify.com/products/functions/).
