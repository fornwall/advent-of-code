[![Crates.io](https://img.shields.io/crates/v/advent-of-code.svg)](https://crates.io/crates/advent-of-code)
[![Docker Hub](https://img.shields.io/docker/v/fredrikfornwall/advent-of-code.svg?label=docker)](https://hub.docker.com/r/fredrikfornwall/advent-of-code)
[![npm](https://img.shields.io/npm/v/advent-of-code-wasm.svg)](https://www.npmjs.com/package/advent-of-code-wasm)
[![PyPi](https://img.shields.io/pypi/v/advent-of-code.svg)](https://pypi.org/project/advent-of-code/)

[![Build](https://github.com/fornwall/advent-of-code/workflows/Github%20CI/badge.svg)](https://github.com/fornwall/advent-of-code/actions?query=workflow%3A%22Github+CI%22)
[![Netlify Status](https://api.netlify.com/api/v1/badges/8cb47a76-7cd7-4545-9f10-56ba075c8e41/deploy-status)](https://app.netlify.com/sites/mystifying-blackwell-9e705f/deploys)

# Advent of Code solutions
Solutions to [Advent of Code](https://adventofcode.com/) problems in Rust.

# Libraries
The `solve(year, day, part, input)` library function is published to these package repositories:

- [crates.io](https://crates.io/crates/advent-of-code) for the Rust library.
- [npm](https://www.npmjs.com/package/advent-of-code-wasm) for a Node.js library using WebAssembly built with [wasm-pack](https://rustwasm.github.io/wasm-pack/).
- [PyPi](https://pypi.org/project/advent-of-code) for a Python library using [PyO3](https://pyo3.rs/) built with [cibuildwheel](https://cibuildwheel.readthedocs.io/).

```rust
// Rust with dependency added: advent-of-code = "2019"
use advent_of_code::solve;

fn main() {
    assert_eq!(solve(2019, 1, 1, "14"), Ok("2".to_string()));
}
```

```python
# Python with dependency added: pip install --upgrade advent-of-code
from advent_of_code import solve

assert solve(2019, 1, 1, "14") == "2"
assert solve(2019, 3, 2, "R8,U5,L5,D3\nU7,R6,D4,L4") == "30"
```

```js
// Node.js with dependency added: npm add advent-of-code-wasm
const assert = require('assert').strict;
const solve = require('advent-of-code-wasm').solve;

assert.equal(solve(2019, 1, 1, "14"), "2");
assert.equal(solve(2019, 3, 2, "R8,U5,L5,D3\nU7,R6,D4,L4"), "30");
```

# Running in the browser
The solutions can be run client-side in a browser using WebAssembly at https://aoc.fornwall.net.

# Post to HTTP endpoint
There is an HTTP endpoint running on [Netlify Functions](https://www.netlify.com/products/functions/) (using Node.js and WebAssembly) that can be used as follows:

```sh
$ curl -d 14 "https://aoc.fornwall.net/solve/2019/1/1"
14
$ curl -s https://raw.githubusercontent.com/fornwall/advent-of-code/master/crates/core/src/year2019/day02_input.txt | \
  curl --data-binary @- \
     "https://aoc.fornwall.net/solve/2019/2/2"
5485
```

# Command line tools
All tools are invoked with day and part as arguments, and expect input on stdin:

```sh
$ $TOOL $YEAR $DAY $PART < $INPUT
```

## Docker image on Docker Hub
```sh
$ docker pull fredrikfornwall/advent-of-code:latest
$ echo 14 | docker run -i fredrikfornwall/advent-of-code:latest 2019 1 1
2
```

## Running a checkout of this code
To run a solution against given input on stdin:

```sh
$ cd crates/core
$ echo 14 | cargo run -q 2019 1 1
2
```

## Rust tool installable from crates.io
```sh
$ cargo install advent-of-code
$ echo 14 | advent-of-code 2019 1 1
2
```

## Node tool installable from npm
```sh
$ npm install advent-of-code-wasm -g
$ echo 14 | advent-of-code-wasm 2019 1 1
2
```

## Python tool installable from PyPi
```sh
$ pip install --upgrade advent-of-code
$ echo 14 | advent-of-code-py 2019 1 1
2
```
