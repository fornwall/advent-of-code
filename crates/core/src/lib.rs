#![cfg_attr(feature = "simd", feature(portable_simd))]
#![forbid(unsafe_code)]
/*!
This crate provides solutions for [Advent of Code](https://adventofcode.com/) problems.

# Usage

This crate is [on crates.io](https://crates.io/crates/advent_of_code) and can be
used by adding `advent_of_code` to your dependencies in your project's `Cargo.toml`.

```toml
[dependencies]
advent_of_code = "*"
```

# Example
By specifying a year, day and part together with a specific problem input
a solution can be obtained:

```rust
use advent_of_code::solve;
let solution = solve(2019, 1, 1, "14");
assert_eq!(solution, Ok("2".to_string()));
```
*/
#![crate_name = "advent_of_code"]

mod common;
#[cfg_attr(test, macro_use)]
mod input;
mod mod_exp;
mod year2015;
mod year2016;
mod year2017;
mod year2018;
mod year2019;
mod year2020;
mod year2021;
mod year2022;
mod year2023;
mod year2024;
mod year2025;

#[cfg(feature = "visualization")]
pub type ResultType = String;

#[cfg(not(feature = "visualization"))]
pub type ResultType = String;

// Never inline to prevent stack size from blowing up in release builds.
#[inline(never)]
fn to_stringer_input<T: ToString>(
    function: fn(&input::Input) -> Result<T, String>,
    input: &input::Input,
) -> Result<String, String> {
    function(input).map(|value| value.to_string())
}

/// Returns the solution for the specified given problem and input.
///
/// # Arguments
///
/// * `year` - The year of the problem, as in 2018 or 2019.
/// * `day` - The day of the problem - from 1 to 25.
/// * `part` - The part of the problem - either 1 or 2.
/// * `input` - The input to the problem.
///
/// # Example
/// ```
/// use advent_of_code::solve;
/// let solution = solve(2019, 1, 1, "14");
/// assert_eq!(solution, Ok("2".to_string()));
/// ```
pub fn solve(year: u16, day: u8, part: u8, input: &str) -> Result<ResultType, String> {
    #![allow(clippy::let_and_return)]
    use crate::input::{Input, Part};
    let input = input.trim_end();

    if input.is_empty() {
        return Err("Empty input".to_string());
    } else if input.len() > 200_000 {
        return Err("Too long input".to_string());
    } else if !input.is_ascii() {
        return Err("Non-ASCII input".to_string());
    } else if !matches!(day, 1..=25) {
        return Err(format!("Invalid day {day} - must be 1-25"));
    } else if !matches!(part, 1 | 2) {
        return Err(format!("Invalid part {part} - must be 1-2"));
    }

    #[cfg(feature = "visualization")]
    let visualization = std::cell::RefCell::new("".to_string());

    let input = Input {
        part: if part == 1 { Part::One } else { Part::Two },
        text: input,
        #[cfg(feature = "visualization")]
        visualization,
    };

    let result = match (year, day) {
        (2015, 1) => to_stringer_input(year2015::day01::solve, &input),
        (2015, 2) => to_stringer_input(year2015::day02::solve, &input),
        (2015, 3) => to_stringer_input(year2015::day03::solve, &input),
        (2015, 4) => to_stringer_input(year2015::day04::solve, &input),
        (2015, 5) => to_stringer_input(year2015::day05::solve, &input),
        (2015, 6) => to_stringer_input(year2015::day06::solve, &input),
        (2015, 7) => to_stringer_input(year2015::day07::solve, &input),
        (2015, 8) => to_stringer_input(year2015::day08::solve, &input),
        (2015, 9) => to_stringer_input(year2015::day09::solve, &input),
        (2015, 10) => to_stringer_input(year2015::day10::solve, &input),
        (2015, 11) => to_stringer_input(year2015::day11::solve, &input),
        (2015, 12) => to_stringer_input(year2015::day12::solve, &input),
        (2015, 13) => to_stringer_input(year2015::day13::solve, &input),
        (2015, 14) => to_stringer_input(year2015::day14::solve, &input),
        (2015, 15) => to_stringer_input(year2015::day15::solve, &input),
        (2015, 16) => to_stringer_input(year2015::day16::solve, &input),
        (2015, 17) => to_stringer_input(year2015::day17::solve, &input),
        (2015, 18) => to_stringer_input(year2015::day18::solve, &input),
        (2015, 19) => to_stringer_input(year2015::day19::solve, &input),
        (2015, 20) => to_stringer_input(year2015::day20::solve, &input),
        (2015, 21) => to_stringer_input(year2015::day21::solve, &input),
        (2015, 22) => to_stringer_input(year2015::day22::solve, &input),
        (2015, 23) => to_stringer_input(year2015::day23::solve, &input),
        (2015, 24) => to_stringer_input(year2015::day24::solve, &input),
        (2015, 25) => to_stringer_input(year2015::day25::solve, &input),
        (2016, 1) => to_stringer_input(year2016::day01::solve, &input),
        (2016, 2) => to_stringer_input(year2016::day02::solve, &input),
        (2016, 3) => to_stringer_input(year2016::day03::solve, &input),
        (2016, 4) => to_stringer_input(year2016::day04::solve, &input),
        (2016, 5) => to_stringer_input(year2016::day05::solve, &input),
        (2016, 6) => to_stringer_input(year2016::day06::solve, &input),
        (2016, 7) => to_stringer_input(year2016::day07::solve, &input),
        (2016, 8) => to_stringer_input(year2016::day08::solve, &input),
        (2016, 9) => to_stringer_input(year2016::day09::solve, &input),
        (2016, 10) => to_stringer_input(year2016::day10::solve, &input),
        (2016, 11) => to_stringer_input(year2016::day11::solve, &input),
        (2016, 12) => to_stringer_input(year2016::day12::solve, &input),
        (2016, 13) => to_stringer_input(year2016::day13::solve, &input),
        (2016, 14) => to_stringer_input(year2016::day14::solve, &input),
        (2016, 15) => to_stringer_input(year2016::day15::solve, &input),
        (2016, 16) => to_stringer_input(year2016::day16::solve, &input),
        (2016, 17) => to_stringer_input(year2016::day17::solve, &input),
        (2016, 18) => to_stringer_input(year2016::day18::solve, &input),
        (2016, 19) => to_stringer_input(year2016::day19::solve, &input),
        (2016, 20) => to_stringer_input(year2016::day20::solve, &input),
        (2016, 21) => to_stringer_input(year2016::day21::solve, &input),
        (2016, 22) => to_stringer_input(year2016::day22::solve, &input),
        (2016, 23) => to_stringer_input(year2016::day23::solve, &input),
        (2016, 24) => to_stringer_input(year2016::day24::solve, &input),
        (2016, 25) => to_stringer_input(year2016::day25::solve, &input),
        (2017, 1) => to_stringer_input(year2017::day01::solve, &input),
        (2017, 2) => to_stringer_input(year2017::day02::solve, &input),
        (2017, 3) => to_stringer_input(year2017::day03::solve, &input),
        (2017, 4) => to_stringer_input(year2017::day04::solve, &input),
        (2017, 5) => to_stringer_input(year2017::day05::solve, &input),
        (2017, 6) => to_stringer_input(year2017::day06::solve, &input),
        (2017, 7) => to_stringer_input(year2017::day07::solve, &input),
        (2017, 8) => to_stringer_input(year2017::day08::solve, &input),
        (2017, 9) => to_stringer_input(year2017::day09::solve, &input),
        (2017, 10) => to_stringer_input(year2017::day10::solve, &input),
        (2017, 11) => to_stringer_input(year2017::day11::solve, &input),
        (2017, 12) => to_stringer_input(year2017::day12::solve, &input),
        (2017, 13) => to_stringer_input(year2017::day13::solve, &input),
        (2017, 14) => to_stringer_input(year2017::day14::solve, &input),
        (2017, 15) => to_stringer_input(year2017::day15::solve, &input),
        (2017, 16) => to_stringer_input(year2017::day16::solve, &input),
        (2017, 17) => to_stringer_input(year2017::day17::solve, &input),
        (2017, 18) => to_stringer_input(year2017::day18::solve, &input),
        (2017, 19) => to_stringer_input(year2017::day19::solve, &input),
        (2017, 20) => to_stringer_input(year2017::day20::solve, &input),
        (2017, 21) => to_stringer_input(year2017::day21::solve, &input),
        (2017, 22) => to_stringer_input(year2017::day22::solve, &input),
        (2017, 23) => to_stringer_input(year2017::day23::solve, &input),
        (2017, 24) => to_stringer_input(year2017::day24::solve, &input),
        (2017, 25) => to_stringer_input(year2017::day25::solve, &input),
        (2018, 1) => to_stringer_input(year2018::day01::solve, &input),
        (2018, 2) => to_stringer_input(year2018::day02::solve, &input),
        (2018, 3) => to_stringer_input(year2018::day03::solve, &input),
        (2018, 4) => to_stringer_input(year2018::day04::solve, &input),
        (2018, 5) => to_stringer_input(year2018::day05::solve, &input),
        (2018, 6) => to_stringer_input(year2018::day06::solve, &input),
        (2018, 7) => to_stringer_input(year2018::day07::solve, &input),
        (2018, 8) => to_stringer_input(year2018::day08::solve, &input),
        (2018, 9) => to_stringer_input(year2018::day09::solve, &input),
        (2018, 10) => to_stringer_input(year2018::day10::solve, &input),
        (2018, 11) => to_stringer_input(year2018::day11::solve, &input),
        (2018, 12) => to_stringer_input(year2018::day12::solve, &input),
        (2018, 13) => to_stringer_input(year2018::day13::solve, &input),
        (2018, 14) => to_stringer_input(year2018::day14::solve, &input),
        (2018, 15) => to_stringer_input(year2018::day15::solve, &input),
        (2018, 16) => to_stringer_input(year2018::day16::solve, &input),
        (2018, 17) => to_stringer_input(year2018::day17::solve, &input),
        (2018, 18) => to_stringer_input(year2018::day18::solve, &input),
        (2018, 19) => to_stringer_input(year2018::day19::solve, &input),
        (2018, 20) => to_stringer_input(year2018::day20::solve, &input),
        (2018, 21) => to_stringer_input(year2018::day21::solve, &input),
        (2018, 22) => to_stringer_input(year2018::day22::solve, &input),
        (2018, 23) => to_stringer_input(year2018::day23::solve, &input),
        (2018, 24) => to_stringer_input(year2018::day24::solve, &input),
        (2018, 25) => to_stringer_input(year2018::day25::solve, &input),
        (2019, 1) => to_stringer_input(year2019::day01::solve, &input),
        (2019, 2) => to_stringer_input(year2019::day02::solve, &input),
        (2019, 3) => to_stringer_input(year2019::day03::solve, &input),
        (2019, 4) => to_stringer_input(year2019::day04::solve, &input),
        (2019, 5) => to_stringer_input(year2019::day05::solve, &input),
        (2019, 6) => to_stringer_input(year2019::day06::solve, &input),
        (2019, 7) => to_stringer_input(year2019::day07::solve, &input),
        (2019, 8) => to_stringer_input(year2019::day08::solve, &input),
        (2019, 9) => to_stringer_input(year2019::day09::solve, &input),
        (2019, 10) => to_stringer_input(year2019::day10::solve, &input),
        (2019, 11) => to_stringer_input(year2019::day11::solve, &input),
        (2019, 12) => to_stringer_input(year2019::day12::solve, &input),
        (2019, 13) => to_stringer_input(year2019::day13::solve, &input),
        (2019, 14) => to_stringer_input(year2019::day14::solve, &input),
        (2019, 15) => to_stringer_input(year2019::day15::solve, &input),
        (2019, 16) => to_stringer_input(year2019::day16::solve, &input),
        (2019, 17) => to_stringer_input(year2019::day17::solve, &input),
        (2019, 18) => to_stringer_input(year2019::day18::solve, &input),
        (2019, 19) => to_stringer_input(year2019::day19::solve, &input),
        (2019, 20) => to_stringer_input(year2019::day20::solve, &input),
        (2019, 21) => to_stringer_input(year2019::day21::solve, &input),
        (2019, 22) => to_stringer_input(year2019::day22::solve, &input),
        (2019, 23) => to_stringer_input(year2019::day23::solve, &input),
        (2019, 24) => to_stringer_input(year2019::day24::solve, &input),
        (2019, 25) => to_stringer_input(year2019::day25::solve, &input),
        (2020, 1) => to_stringer_input(year2020::day01::solve, &input),
        (2020, 2) => to_stringer_input(year2020::day02::solve, &input),
        (2020, 3) => to_stringer_input(year2020::day03::solve, &input),
        (2020, 4) => to_stringer_input(year2020::day04::solve, &input),
        (2020, 5) => to_stringer_input(year2020::day05::solve, &input),
        (2020, 6) => to_stringer_input(year2020::day06::solve, &input),
        (2020, 7) => to_stringer_input(year2020::day07::solve, &input),
        (2020, 8) => to_stringer_input(year2020::day08::solve, &input),
        (2020, 9) => to_stringer_input(year2020::day09::solve, &input),
        (2020, 10) => to_stringer_input(year2020::day10::solve, &input),
        (2020, 11) => to_stringer_input(year2020::day11::solve, &input),
        (2020, 12) => to_stringer_input(year2020::day12::solve, &input),
        (2020, 13) => to_stringer_input(year2020::day13::solve, &input),
        (2020, 14) => to_stringer_input(year2020::day14::solve, &input),
        (2020, 15) => to_stringer_input(year2020::day15::solve, &input),
        (2020, 16) => to_stringer_input(year2020::day16::solve, &input),
        (2020, 17) => to_stringer_input(year2020::day17::solve, &input),
        (2020, 18) => to_stringer_input(year2020::day18::solve, &input),
        (2020, 19) => to_stringer_input(year2020::day19::solve, &input),
        (2020, 20) => to_stringer_input(year2020::day20::solve, &input),
        (2020, 21) => to_stringer_input(year2020::day21::solve, &input),
        (2020, 22) => to_stringer_input(year2020::day22::solve, &input),
        (2020, 23) => to_stringer_input(year2020::day23::solve, &input),
        (2020, 24) => to_stringer_input(year2020::day24::solve, &input),
        (2020, 25) => to_stringer_input(year2020::day25::solve, &input),
        (2021, 1) => to_stringer_input(year2021::day01::solve, &input),
        (2021, 2) => to_stringer_input(year2021::day02::solve, &input),
        (2021, 3) => to_stringer_input(year2021::day03::solve, &input),
        (2021, 4) => to_stringer_input(year2021::day04::solve, &input),
        (2021, 5) => to_stringer_input(year2021::day05::solve, &input),
        (2021, 6) => to_stringer_input(year2021::day06::solve, &input),
        (2021, 7) => to_stringer_input(year2021::day07::solve, &input),
        (2021, 8) => to_stringer_input(year2021::day08::solve, &input),
        (2021, 9) => to_stringer_input(year2021::day09::solve, &input),
        (2021, 10) => to_stringer_input(year2021::day10::solve, &input),
        (2021, 11) => to_stringer_input(year2021::day11::solve, &input),
        (2021, 12) => to_stringer_input(year2021::day12::solve, &input),
        (2021, 13) => to_stringer_input(year2021::day13::solve, &input),
        (2021, 14) => to_stringer_input(year2021::day14::solve, &input),
        (2021, 15) => to_stringer_input(year2021::day15::solve, &input),
        (2021, 16) => to_stringer_input(year2021::day16::solve, &input),
        (2021, 17) => to_stringer_input(year2021::day17::solve, &input),
        (2021, 18) => to_stringer_input(year2021::day18::solve, &input),
        (2021, 19) => to_stringer_input(year2021::day19::solve, &input),
        (2021, 20) => to_stringer_input(year2021::day20::solve, &input),
        (2021, 21) => to_stringer_input(year2021::day21::solve, &input),
        (2021, 22) => to_stringer_input(year2021::day22::solve, &input),
        (2021, 23) => to_stringer_input(year2021::day23::solve, &input),
        (2021, 24) => to_stringer_input(year2021::day24::solve, &input),
        (2021, 25) => to_stringer_input(year2021::day25::solve, &input),
        (2022, 1) => to_stringer_input(year2022::day01::solve, &input),
        (2022, 2) => to_stringer_input(year2022::day02::solve, &input),
        (2022, 3) => to_stringer_input(year2022::day03::solve, &input),
        (2022, 4) => to_stringer_input(year2022::day04::solve, &input),
        (2022, 5) => to_stringer_input(year2022::day05::solve, &input),
        (2022, 6) => to_stringer_input(year2022::day06::solve, &input),
        (2022, 7) => to_stringer_input(year2022::day07::solve, &input),
        (2022, 8) => to_stringer_input(year2022::day08::solve, &input),
        (2022, 9) => to_stringer_input(year2022::day09::solve, &input),
        (2022, 10) => to_stringer_input(year2022::day10::solve, &input),
        (2022, 11) => to_stringer_input(year2022::day11::solve, &input),
        (2022, 12) => to_stringer_input(year2022::day12::solve, &input),
        (2022, 13) => to_stringer_input(year2022::day13::solve, &input),
        (2022, 14) => to_stringer_input(year2022::day14::solve, &input),
        (2022, 15) => to_stringer_input(year2022::day15::solve, &input),
        (2022, 16) => to_stringer_input(year2022::day16::solve, &input),
        (2022, 17) => to_stringer_input(year2022::day17::solve, &input),
        (2022, 18) => to_stringer_input(year2022::day18::solve, &input),
        (2022, 19) => to_stringer_input(year2022::day19::solve, &input),
        (2022, 20) => to_stringer_input(year2022::day20::solve, &input),
        (2022, 21) => to_stringer_input(year2022::day21::solve, &input),
        (2022, 22) => to_stringer_input(year2022::day22::solve, &input),
        (2022, 23) => to_stringer_input(year2022::day23::solve, &input),
        (2022, 24) => to_stringer_input(year2022::day24::solve, &input),
        (2022, 25) => to_stringer_input(year2022::day25::solve, &input),
        (2023, 1) => to_stringer_input(year2023::day01::solve, &input),
        (2023, 2) => to_stringer_input(year2023::day02::solve, &input),
        (2023, 3) => to_stringer_input(year2023::day03::solve, &input),
        (2023, 4) => to_stringer_input(year2023::day04::solve, &input),
        (2023, 5) => to_stringer_input(year2023::day05::solve, &input),
        (2023, 6) => to_stringer_input(year2023::day06::solve, &input),
        (2023, 7) => to_stringer_input(year2023::day07::solve, &input),
        (2023, 8) => to_stringer_input(year2023::day08::solve, &input),
        (2023, 9) => to_stringer_input(year2023::day09::solve, &input),
        (2023, 10) => to_stringer_input(year2023::day10::solve, &input),
        (2023, 11) => to_stringer_input(year2023::day11::solve, &input),
        (2023, 12) => to_stringer_input(year2023::day12::solve, &input),
        (2023, 13) => to_stringer_input(year2023::day13::solve, &input),
        (2023, 14) => to_stringer_input(year2023::day14::solve, &input),
        (2023, 15) => to_stringer_input(year2023::day15::solve, &input),
        (2023, 16) => to_stringer_input(year2023::day16::solve, &input),
        (2023, 17) => to_stringer_input(year2023::day17::solve, &input),
        (2023, 18) => to_stringer_input(year2023::day18::solve, &input),
        (2023, 19) => to_stringer_input(year2023::day19::solve, &input),
        (2023, 20) => to_stringer_input(year2023::day20::solve, &input),
        (2023, 21) => to_stringer_input(year2023::day21::solve, &input),
        (2023, 22) => to_stringer_input(year2023::day22::solve, &input),
        (2023, 23) => to_stringer_input(year2023::day23::solve, &input),
        (2023, 24) => to_stringer_input(year2023::day24::solve, &input),
        (2023, 25) => to_stringer_input(year2023::day25::solve, &input),
        (2024, 1) => to_stringer_input(year2024::day01::solve, &input),
        (2024, 2) => to_stringer_input(year2024::day02::solve, &input),
        (2024, 3) => to_stringer_input(year2024::day03::solve, &input),
        (2024, 4) => to_stringer_input(year2024::day04::solve, &input),
        (2024, 5) => to_stringer_input(year2024::day05::solve, &input),
        (2024, 6) => to_stringer_input(year2024::day06::solve, &input),
        (2024, 7) => to_stringer_input(year2024::day07::solve, &input),
        (2024, 8) => to_stringer_input(year2024::day08::solve, &input),
        (2024, 9) => to_stringer_input(year2024::day09::solve, &input),
        (2024, 10) => to_stringer_input(year2024::day10::solve, &input),
        (2024, 11) => to_stringer_input(year2024::day11::solve, &input),
        (2024, 12) => to_stringer_input(year2024::day12::solve, &input),
        (2024, 13) => to_stringer_input(year2024::day13::solve, &input),
        (2024, 14) => to_stringer_input(year2024::day14::solve, &input),
        (2024, 15) => to_stringer_input(year2024::day15::solve, &input),
        (2024, 16) => to_stringer_input(year2024::day16::solve, &input),
        (2024, 17) => to_stringer_input(year2024::day17::solve, &input),
        (2024, 18) => to_stringer_input(year2024::day18::solve, &input),
        (2024, 19) => to_stringer_input(year2024::day19::solve, &input),
        (2024, 20) => to_stringer_input(year2024::day20::solve, &input),
        (2024, 21) => to_stringer_input(year2024::day21::solve, &input),
        (2024, 22) => to_stringer_input(year2024::day22::solve, &input),
        (2024, 23) => to_stringer_input(year2024::day23::solve, &input),
        (2024, 24) => to_stringer_input(year2024::day24::solve, &input),
        (2024, 25) => to_stringer_input(year2024::day25::solve, &input),
        (2025, 1) => to_stringer_input(year2025::day01::solve, &input),
        (2025, 2) => to_stringer_input(year2025::day02::solve, &input),
        (2025, 3) => to_stringer_input(year2025::day03::solve, &input),
        (2025, 4) => to_stringer_input(year2025::day04::solve, &input),
        (2025, 5) => to_stringer_input(year2025::day05::solve, &input),
        (2025, 6) => to_stringer_input(year2025::day06::solve, &input),
        (2025, 7) => to_stringer_input(year2025::day07::solve, &input),
        (2025, 8) => to_stringer_input(year2025::day08::solve, &input),
        (2025, 9) => to_stringer_input(year2025::day09::solve, &input),
        (2025, 10) => to_stringer_input(year2025::day10::solve, &input),
        (2025, 11) => to_stringer_input(year2025::day11::solve, &input),
        (2025, 12) => to_stringer_input(year2025::day12::solve, &input),
        _ => Err(format!("Unsupported year={year}, day={day}, part={part}")),
    };

    #[cfg(feature = "visualization")]
    return result.map(|_| input.visualization.take());

    #[cfg(not(feature = "visualization"))]
    result
}

/// A version of [solve](fn.solve.html) that takes strings as arguments and parses them to the required types.
pub fn solve_raw(year: &str, day: &str, part: &str, input: &str) -> Result<ResultType, String> {
    let year = year.parse::<u16>().map_err(|_| "Invalid year")?;
    let day = day.parse::<u8>().map_err(|_| "Invalid day")?;
    let part = part.parse::<u8>().map_err(|_| "Invalid part")?;
    solve(year, day, part, input)
}
