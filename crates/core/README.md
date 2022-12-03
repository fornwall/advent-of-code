[![Crates.io](https://img.shields.io/crates/v/advent-of-code.svg)](https://crates.io/crates/advent-of-code)
[![Docker Hub](https://img.shields.io/docker/v/fredrikfornwall/advent-of-code.svg?label=docker)](https://hub.docker.com/r/fredrikfornwall/advent-of-code)
[![codecov](https://codecov.io/gh/fornwall/advent-of-code/branch/master/graph/badge.svg?token=BDNM8eS7I1)](https://codecov.io/gh/fornwall/advent-of-code)

# Advent of Code solutions
Solutions to [Advent of Code](https://adventofcode.com/) problems in Rust.

Just a test line.

## Running locally
The command line tool takes `<year> <day> <part>` as arguments and reads the problem input from stdin:

```sh
$ echo 14 | cargo run -q 2019 1 1
2
```

## Published crate
This crate is published to [crates.io/crates/advent-of-code](https://crates.io/crates/advent-of-code).

As a library it exposes a `solve(year, day, part, input)` function as documented on [docs.rs/advent-of-code](https://docs.rs/advent-of-code/):

```rust
// Using the dependency added: advent-of-code = "*":
use advent_of_code::solve;

fn main() {
    assert_eq!(solve(2019, 1, 1, "14"), Ok("2".to_string()));
}
```

It also contains the command line program to run the solution:

```sh
$ cargo install advent-of-code
$ echo 14 | advent-of-code 2019 1 1
2
````

## Docker image on Docker Hub
The command line interface is published to a [fredrikfornwall/advent-of-code](https://hub.docker.com/r/fredrikfornwall/advent-of-code) Docker image:

```sh
$ docker pull fredrikfornwall/advent-of-code:latest
$ echo 14 | docker run -i fredrikfornwall/advent-of-code:latest 2019 1 1
2
```

## Generating flamegraphs on macOS
Install [flamegraph](https://github.com/flamegraph-rs/flamegraph) with `cargo install flamegraph` and build a benchmark binary with:

```sh
RUSTFLAGS='-g' cargo build --release --bench benchmark
```

This will create a benchmark binary under something like `ls ../../target/release/deps/benchmark-31ba773f80f7f5d8`. Then profile and generate a `flamegraph.svg` by running

```sh
sudo flamegraph ../../target/release/deps/benchmark-31ba773f80f7f5d8  --bench 2020_07_1
```
