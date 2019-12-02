[![Build Status](https://travis-ci.org/fornwall/advent-of-code-2019-rs.svg?branch=master)](https://travis-ci.org/fornwall/advent-of-code-2019-rs)

# Advent of Code 2019 in Rust
This repository contains solutions to [Advent of Code 2019](https://adventofcode.com/2019) problems in Rust.

# Running the tests
[Install rust](https://www.rust-lang.org/tools/install) if necessary, then run tests with:

    cargo test

# Running the solutions against custom input
To run a solution against given input on stdin:

    cargo run <day> <part> < path/to/input.txt

    # Examples:
    cargo run 2 1 < path/to/input_day2_part1.txt
    curl https://example.com/example-input | cargo run 2 1

# Running using Docker
There is also a Docker image published for running the tests:

    docker run -i fredrikfornwall/advent-of-code-2019-rs:latest <day> <part> < path/to/input.txt

    # Example:
    docker run -i fredrikfornwall/advent-of-code-2019-rs:latest 1 1 < src/day1_input.txt

# Running using Node.js
As a proof of concept, an [npm module](https://www.npmjs.com/package/advent_of_code_rs) is available which uses WebAssembly to execute the solution:

    # Installation:
    npm install advent_of_code_rs_bin -g

    # Invocation:
    advent-of-code-rs <day> <part> < path/to/input.txt

# Running in the browser
As another proof of concept the solution can run inside the browser at https://fornwall.net/advent-of-code-2019/.

# Days
| Puzzle                                         | Solution                     | Alternatives |
| ---------------------------------------------- | ---------------------------- | ------------ |
| [Day 1](https://adventofcode.com/2019/day/1)   | [src/day1.rs](src/day1.rs)   | [Reddit](https://www.reddit.com/r/adventofcode/comments/e4axxe/2019_day_1_solutions/)
| [Day 2](https://adventofcode.com/2019/day/2)   | [src/day2.rs](src/day2.rs)   | [Reddit](https://www.reddit.com/r/adventofcode/comments/e4u0rw/2019_day_2_solutions/)
| [Day 3](https://adventofcode.com/2019/day/3)   | [src/day3.rs](src/day3.rs)   | [Reddit](https://www.reddit.com/r/adventofcode/comments/XXX/2019_day_3_solutions/)
| [Day 4](https://adventofcode.com/2019/day/4)   | [src/day4.rs](src/day4.rs)   | [Reddit](https://www.reddit.com/r/adventofcode/comments/XXX/2019_day_4_solutions/)
| [Day 5](https://adventofcode.com/2019/day/5)   | [src/day5.rs](src/day5.rs)   | [Reddit](https://www.reddit.com/r/adventofcode/comments/XXX/2019_day_5_solutions/)
| [Day 6](https://adventofcode.com/2019/day/6)   | [src/day6.rs](src/day6.rs)   | [Reddit](https://www.reddit.com/r/adventofcode/comments/XXX/2019_day_6_solutions/)
| [Day 7](https://adventofcode.com/2019/day/7)   | [src/day7.rs](src/day7.rs)   | [Reddit](https://www.reddit.com/r/adventofcode/comments/XXX/2019_day_7_solutions/)
| [Day 8](https://adventofcode.com/2019/day/8)   | [src/day8.rs](src/day8.rs)   | [Reddit](https://www.reddit.com/r/adventofcode/comments/XXX/2019_day_8_solutions/)
| [Day 9](https://adventofcode.com/2019/day/9)   | [src/day9.rs](src/day9.rs)   | [Reddit](https://www.reddit.com/r/adventofcode/comments/XXX/2019_day_9_solutions/)
| [Day 10](https://adventofcode.com/2019/day/10) | [src/day10.rs](src/day10.rs) | [Reddit](https://www.reddit.com/r/adventofcode/comments/XXX/2019_day_10_solutions/)
| [Day 11](https://adventofcode.com/2019/day/11) | [src/day11.rs](src/day11.rs) | [Reddit](https://www.reddit.com/r/adventofcode/comments/XXX/2019_day_11_solutions/)
| [Day 12](https://adventofcode.com/2019/day/12) | [src/day12.rs](src/day12.rs) | [Reddit](https://www.reddit.com/r/adventofcode/comments/XXX/2019_day_12_solutions/)
| [Day 13](https://adventofcode.com/2019/day/13) | [src/day13.rs](src/day13.rs) | [Reddit](https://www.reddit.com/r/adventofcode/comments/XXX/2019_day_13_solutions/)
| [Day 14](https://adventofcode.com/2019/day/14) | [src/day14.rs](src/day14.rs) | [Reddit](https://www.reddit.com/r/adventofcode/comments/XXX/2019_day_14_solutions/)
| [Day 15](https://adventofcode.com/2019/day/15) | [src/day15.rs](src/day15.rs) | [Reddit](https://www.reddit.com/r/adventofcode/comments/XXX/2019_day_15_solutions/)
| [Day 16](https://adventofcode.com/2019/day/16) | [src/day16.rs](src/day16.rs) | [Reddit](https://www.reddit.com/r/adventofcode/comments/XXX/2019_day_16_solutions/)
| [Day 17](https://adventofcode.com/2019/day/17) | [src/day17.rs](src/day17.rs) | [Reddit](https://www.reddit.com/r/adventofcode/comments/XXX/2019_day_17_solutions/)
| [Day 18](https://adventofcode.com/2019/day/18) | [src/day18.rs](src/day18.rs) | [Reddit](https://www.reddit.com/r/adventofcode/comments/XXX/2019_day_18_solutions/)
| [Day 19](https://adventofcode.com/2019/day/19) | [src/day19.rs](src/day19.rs) | [Reddit](https://www.reddit.com/r/adventofcode/comments/XXX/2019_day_19_solutions/)
| [Day 20](https://adventofcode.com/2019/day/20) | [src/day20.rs](src/day20.rs) | [Reddit](https://www.reddit.com/r/adventofcode/comments/XXX/2017_day_20_solutions/)
| [Day 21](https://adventofcode.com/2019/day/21) | [src/day21.rs](src/day21.rs) | [Reddit](https://www.reddit.com/r/adventofcode/comments/XXX/2019_day_21_solutions/)
| [Day 22](https://adventofcode.com/2019/day/22) | [src/day22.rs](src/day22.rs) | [Reddit](https://www.reddit.com/r/adventofcode/comments/XXX/2019_day_22_solutions/)
| [Day 23](https://adventofcode.com/2019/day/23) | [src/day23.rs](src/day23.rs) | [Reddit](https://www.reddit.com/r/adventofcode/comments/XXX/2019_day_23_solutions/)
| [Day 24](https://adventofcode.com/2019/day/24) | [src/day24.rs](src/day24.rs) | [Reddit](https://www.reddit.com/r/adventofcode/comments/XXX/2019_day_24_solutions/)
| [Day 25](https://adventofcode.com/2019/day/25) | [src/day25.rs](src/day25.rs) | [Reddit](https://www.reddit.com/r/adventofcode/comments/XXX/2019_day_25_solutions/)
