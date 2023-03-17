[![Build](https://github.com/fornwall/advent-of-code/workflows/Github%20CI/badge.svg)](https://github.com/fornwall/advent-of-code/actions?query=workflow%3A%22Github+CI%22)

# Advent of Code solutions
Solutions to [Advent of Code](https://adventofcode.com/) problems (2015-2022) in Rust.

For learning and demonstration purposes, this repository contains additional resources building on the solutions:

- [crates/core](crates/core): The solutions themselves as a library and a command-line tool.
  - Published as a [crate](https://crates.io/crates/advent-of-code).
  - The command-line tool is wrapped in Docker and published to [Docker Hub](https://hub.docker.com/r/fredrikfornwall/advent-of-code).
  - The command-line tool is also published as a [homebrew](https://brew.sh/) tap.
  - No unsafe code.
- [crates/grpc-server](crates/grpc-server): Server with a gRPC API using [Tonic](https://docs.rs/tonic).
  - Deployed to [fly.io](https://fly.io/).
- [crates/http-server](crates/http-server): Server with a HTTP API using [axum](https://docs.rs/axum/latest/axum/).
  - Deployed to [fly.io](https://fly.io/).
  - API is described by a [OpenAPI](https://www.openapis.org/) interface at [aoc.fornwall.net/api/](https://aoc.fornwall.net/api/).
- [crates/fuzzing-afl](crates/fuzzing-afl): Fuzzing using [afl](https://lcamtuf.coredump.cx/afl/).
- [crates/fuzzing-hfuzz](crates/fuzzing-hfuzz): Fuzzing using [hongfuzz](https://honggfuzz.dev/).
- [crates/fuzzing-libfuzzer](crates/fuzzing-libfuzzer): Fuzzing using [libFuzzer](https://llvm.org/docs/LibFuzzer.html).
- [crates/java](crates/java): Java library using [JNI](https://github.com/jni-rs/jni-rs).
  - Published to [Maven Central](https://search.maven.org/artifact/net.fornwall/aoc).
- [crates/python](crates/python): Python library wrapping the solutions using [PyO3](https://pyo3.rs/).
  - Published to [PyPI](https://pypi.org/project/advent-of-code/).
- [crates/wasm](crates/wasm): WebAssembly build using [wasm-bindgen](https://rustwasm.github.io/docs/wasm-bindgen/).
  - Published to a static site at [aoc.fornwall.net](https://aoc.fornwall.net).
  - Published to [npm](https://www.npmjs.com/package/advent-of-code-wasm).
  - Deployed to [Netlify Functions](https://www.netlify.com/products/functions/).
- [crates/worker](crates/worker): A [Cloudflare Worker](https://workers.cloudflare.com/).

All deployments are done using [GitHub Actions workflows](.github/workflows/).
