[![Build Status](https://travis-ci.org/fornwall/advent-of-code-2019-rs.svg?branch=master)](https://travis-ci.org/fornwall/advent-of-code-2019-rs)
[![Netlify Status](https://api.netlify.com/api/v1/badges/8cb47a76-7cd7-4545-9f10-56ba075c8e41/deploy-status)](https://app.netlify.com/sites/mystifying-blackwell-9e705f/deploys)

# Advent of Code 2019 in Rust
This repository contains solutions to [Advent of Code 2019](https://adventofcode.com/2019) problems in Rust.

# Running the tests
[Install rust](https://www.rust-lang.org/tools/install) if necessary, then run tests with:

```
cargo test
```

# Running the solutions against custom input
To run a solution against given input on stdin:

```sh
# Debug mode:
cargo run -q <day> <part> < path/to/input.txt
# Release mode:
cargo run -q --release <day> <part> < path/to/input.txt

# Examples:
cargo run 2 1 < path/to/input_day2_part1.txt
```

# Running using Docker
A published Docker image can be used to run against custom input:

```sh
docker run -i fredrikfornwall/advent-of-code-2019-rs:latest <day> <part> < path/to/input.txt

# Example:
docker run -i fredrikfornwall/advent-of-code-2019-rs:latest 1 1 < src/day1_input.txt
```

# Libraries
The `solve(day, part, input)` library function is published on these package repositories:

- (crates.io)[https://crates.io/crates/advent_of_code_rs] for the Rust library.
- (npm)[https://www.npmjs.com/package/advent_of_code_rs_wasm] for a Node.js library using WebAssembly built by [wasm-pack](https://rustwasm.github.io/wasm-pack/).
- (PyPi)[https://pypi.org/project/advent-of-code-rs-python] for a Python library using [PyO3](https://pyo3.rs/) and built with [cibuildwheel](https://cibuildwheel.readthedocs.io/).

```rust
fn todo() {}
```

```python
# Python with dependency added: pip install --upgrade advent-of-code-rs-python
>>> import advent_of_code_rs_python
>>> advent_of_code_rs_python.solve(1, 1, "12")
'2'
```

```js
# Node.js with dependency added: npm add advent_of_code_rs_wasm
const assert = require('assert').strict;
const solve = require('advent_of_code_rs_wasm').solve;

assert.equal(solve(1, 1, '14'), '2');
assert.equal(solve(3, 2, "R8,U5,L5,D3\nU7,R6,D4,L4"), '30');
```

# Command line tools
All tools are invoked with day and part as arguments, and expect input on stdin:

```sh
$ $TOOL $DAY $PART < $INPUT
```

## Rust tool installable from crates.io
```sh
$ cargo install advent_of_code_rs
# [..]
$ echo 14 | advent_of_code_rs 1 1
2
```

## Node tool installable from npm
```sh
$ npm install advent_of_code_rs_bin -g
# [..]
$ echo 14 | advent-of-code-rs 1 1
2
```

## Python tool installable from PyPi
```sh
$ echo TODO
```

# Running in the browser
The solutions can run using WebAssembly at https://advent2019.fornwall.net.

# Post to HTTP endpoint
There is a HTTP endpoint running on [Netlify Functions](https://www.netlify.com/products/functions/) (using Node.js and WebAssembly) that can be used as follows:

```sh
$ curl --data-binary @crates/core/src/day02_input.txt \
     "https://advent2019.fornwall.net/.netlify/functions/solve?day=2&part=2"
```

# Days
| Puzzle                                         | Solution                     | Alternatives |
| ---------------------------------------------- | ---------------------------- | ------------ |
| [Day 1](https://adventofcode.com/2019/day/1)   | [day01.rs](crates/core/src/day01.rs) | [Reddit](https://www.reddit.com/r/adventofcode/comments/e4axxe/2019_day_1_solutions/)
| [Day 2](https://adventofcode.com/2019/day/2)   | [day02.rs](crates/core/src/day02.rs) | [Reddit](https://www.reddit.com/r/adventofcode/comments/e4u0rw/2019_day_2_solutions/)
| [Day 3](https://adventofcode.com/2019/day/3)   | [day03.rs](crates/core/src/day03.rs) | [Reddit](https://www.reddit.com/r/adventofcode/comments/e5bz2w/2019_day_3_solutions/)
| [Day 4](https://adventofcode.com/2019/day/4)   | [day04.rs](crates/core/src/day04.rs) | [Reddit](https://www.reddit.com/r/adventofcode/comments/e5u5fv/2019_day_4_solutions/)
| [Day 5](https://adventofcode.com/2019/day/5)   | [day05.rs](crates/core/src/day05.rs) | [Reddit](https://www.reddit.com/r/adventofcode/comments/e6carb/2019_day_5_solutions/)
| [Day 6](https://adventofcode.com/2019/day/6)   | [day06.rs](crates/core/src/day06.rs) | [Reddit](https://www.reddit.com/r/adventofcode/comments/e6tyva/2019_day_6_solutions/)
| [Day 7](https://adventofcode.com/2019/day/7)   | [day07.rs](crates/core/src/day07.rs) | [Reddit](https://www.reddit.com/r/adventofcode/comments/e7a4nj/2019_day_7_solutions/)
| [Day 8](https://adventofcode.com/2019/day/8)   | [day08.rs](crates/core/src/day08.rs) | [Reddit](https://www.reddit.com/r/adventofcode/comments/e7pkmt/2019_day_8_solutions/)
| [Day 9](https://adventofcode.com/2019/day/9)   | [day09.rs](crates/core/src/day09.rs) | [Reddit](https://www.reddit.com/r/adventofcode/comments/e85b6d/2019_day_9_solutions/)
| [Day 10](https://adventofcode.com/2019/day/10) | [day10.rs](crates/core/src/day10.rs) | [Reddit](https://www.reddit.com/r/adventofcode/comments/e8m1z3/2019_day_10_solutions/)
| [Day 11](https://adventofcode.com/2019/day/11) | [day11.rs](crates/core/src/day11.rs) | [Reddit](https://www.reddit.com/r/adventofcode/comments/e92jm2/2019_day_11_solutions/)
| [Day 12](https://adventofcode.com/2019/day/12) | [day12.rs](crates/core/src/day12.rs) | [Reddit](https://www.reddit.com/r/adventofcode/comments/e9j0ve/2019_day_12_solutions/)
| [Day 13](https://adventofcode.com/2019/day/13) | [day13.rs](crates/core/src/day13.rs) | [Reddit](https://www.reddit.com/r/adventofcode/comments/e9zgse/2019_day_13_solutions/)
| [Day 14](https://adventofcode.com/2019/day/14) | [day14.rs](crates/core/src/day14.rs) | [Reddit](https://www.reddit.com/r/adventofcode/comments/eafj32/2019_day_14_solutions/)
| [Day 15](https://adventofcode.com/2019/day/15) | [day15.rs](crates/core/src/day15.rs) | [Reddit](https://www.reddit.com/r/adventofcode/comments/eaurfo/2019_day_15_solutions/)
| [Day 16](https://adventofcode.com/2019/day/16) | [day16.rs](crates/core/src/day16.rs) | [Reddit](https://www.reddit.com/r/adventofcode/comments/ebai4g/2019_day_16_solutions/)
| [Day 17](https://adventofcode.com/2019/day/17) | [day17.rs](crates/core/src/day17.rs) | [Reddit](https://www.reddit.com/r/adventofcode/comments/ebr7dg/2019_day_17_solutions/)
| [Day 18](https://adventofcode.com/2019/day/18) | [day18.rs](crates/core/src/day18.rs) | [Reddit](https://www.reddit.com/r/adventofcode/comments/ec8090/2019_day_18_solutions/)
| [Day 19](https://adventofcode.com/2019/day/19) | [day19.rs](crates/core/src/day19.rs) | [Reddit](https://www.reddit.com/r/adventofcode/comments/ecogl3/2019_day_19_solutions/)
| [Day 20](https://adventofcode.com/2019/day/20) | [day20.rs](crates/core/src/day20.rs) | [Reddit](https://www.reddit.com/r/adventofcode/comments/ed5ei2/2019_day_20_solutions/)
| [Day 21](https://adventofcode.com/2019/day/21) | [day21.rs](crates/core/src/day21.rs) | [Reddit](https://www.reddit.com/r/adventofcode/comments/edll5a/2019_day_21_solutions/)
| [Day 22](https://adventofcode.com/2019/day/22) | [day22.rs](crates/core/src/day22.rs) | [Reddit](https://www.reddit.com/r/adventofcode/comments/ee0rqi/2019_day_22_solutions/)
| [Day 23](https://adventofcode.com/2019/day/23) | [day23.rs](crates/core/src/day23.rs) | [Reddit](https://www.reddit.com/r/adventofcode/comments/eefva8/2019_day_23_solutions/)
| [Day 24](https://adventofcode.com/2019/day/24) | [day24.rs](crates/core/src/day24.rs) | [Reddit](https://www.reddit.com/r/adventofcode/comments/eewjtt/2019_day_24_solutions/)
| [Day 25](https://adventofcode.com/2019/day/25) | [day25.rs](crates/core/src/day25.rs) | [Reddit](https://www.reddit.com/r/adventofcode/comments/efca4m/2019_day_25_solutions/)
