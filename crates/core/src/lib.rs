#![forbid(unsafe_code)]
/*!
This crates provides solutions for Advent of Code problems.
*/
#![crate_name = "advent_of_code"]

mod input;
mod mod_exp;
#[cfg(feature = "visualization")]
pub mod painter;
mod year2015;
mod year2016;
mod year2017;
mod year2018;
mod year2019;
mod year2020;

use input::{Input, Part};
#[cfg(feature = "visualization")]
use painter::PainterRef;

fn to_stringer<T: ToString>(
    function: fn(&str) -> Result<T, String>,
    input: &str,
) -> Result<String, String> {
    function(input).map(|value| value.to_string())
}

fn to_stringer_input<T: ToString>(
    function: fn(&mut Input) -> Result<T, String>,
    input: &mut Input,
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
/// # Examples
/// ```
/// use advent_of_code::solve;
/// let solution = solve(2019, 1, 1, "14");
/// assert_eq!(solution, Ok("2".to_string()));
/// ```
pub fn solve(
    year: u16,
    day: u8,
    part: u8,
    input_string: &str,
    #[cfg(feature = "visualization")] painter: PainterRef,
) -> Result<String, String> {
    #![allow(clippy::let_and_return)]

    if input_string.is_empty() {
        return Err("Empty input".to_string());
    } else if !input_string.is_ascii() {
        return Err("Non-ASCII input".to_string());
    } else if !matches!(day, 0..=25) {
        return Err(format!("Invalid day: {} - must be 1-25", day));
    } else if !matches!(part, 1 | 2) {
        return Err(format!("Invalid part: {} - must be 1-2", part));
    }

    let mut input = Input {
        part: if part == 1 { Part::One } else { Part::Two },
        text: input_string,
        #[cfg(feature = "visualization")]
        painter,
    };

    let result = match (year, day, part) {
        (2015, 1, _) => to_stringer_input(year2015::day01::solve, &mut input),
        (2015, 2, _) => to_stringer_input(year2015::day02::solve, &mut input),
        (2015, 3, _) => to_stringer_input(year2015::day03::solve, &mut input),
        (2015, 4, _) => to_stringer_input(year2015::day04::solve, &mut input),
        (2015, 5, _) => to_stringer_input(year2015::day05::solve, &mut input),
        (2015, 6, _) => to_stringer_input(year2015::day06::solve, &mut input),
        (2015, 7, _) => to_stringer_input(year2015::day07::solve, &mut input),
        (2015, 8, _) => to_stringer_input(year2015::day08::solve, &mut input),
        (2015, 9, _) => to_stringer_input(year2015::day09::solve, &mut input),
        (2015, 10, _) => to_stringer_input(year2015::day10::solve, &mut input),
        (2015, 11, _) => to_stringer_input(year2015::day11::solve, &mut input),
        (2015, 12, _) => to_stringer_input(year2015::day12::solve, &mut input),
        (2015, 13, _) => to_stringer_input(year2015::day13::solve, &mut input),
        (2015, 14, _) => to_stringer_input(year2015::day14::solve, &mut input),
        (2015, 15, _) => to_stringer_input(year2015::day15::solve, &mut input),
        (2015, 16, _) => to_stringer_input(year2015::day16::solve, &mut input),
        (2015, 17, _) => to_stringer_input(year2015::day17::solve, &mut input),
        (2015, 18, _) => to_stringer_input(year2015::day18::solve, &mut input),
        (2015, 19, _) => to_stringer_input(year2015::day19::solve, &mut input),
        (2015, 20, _) => to_stringer_input(year2015::day20::solve, &mut input),
        (2015, 21, _) => to_stringer_input(year2015::day21::solve, &mut input),
        (2015, 22, _) => to_stringer_input(year2015::day22::solve, &mut input),
        (2015, 23, _) => to_stringer_input(year2015::day23::solve, &mut input),
        (2015, 24, _) => to_stringer_input(year2015::day24::solve, &mut input),
        (2015, 25, _) => to_stringer_input(year2015::day25::solve, &mut input),
        (2016, 1, _) => to_stringer_input(year2016::day01::solve, &mut input),
        (2016, 2, _) => to_stringer_input(year2016::day02::solve, &mut input),
        (2016, 3, _) => to_stringer_input(year2016::day03::solve, &mut input),
        (2016, 4, _) => to_stringer_input(year2016::day04::solve, &mut input),
        (2016, 5, _) => to_stringer_input(year2016::day05::solve, &mut input),
        (2016, 6, _) => to_stringer_input(year2016::day06::solve, &mut input),
        (2016, 7, _) => to_stringer_input(year2016::day07::solve, &mut input),
        (2016, 8, _) => to_stringer_input(year2016::day08::solve, &mut input),
        (2016, 9, _) => to_stringer_input(year2016::day09::solve, &mut input),
        (2016, 10, _) => to_stringer_input(year2016::day10::solve, &mut input),
        (2016, 11, _) => to_stringer_input(year2016::day11::solve, &mut input),
        (2016, 12, _) => to_stringer_input(year2016::day12::solve, &mut input),
        (2016, 13, _) => to_stringer_input(year2016::day13::solve, &mut input),
        (2016, 14, _) => to_stringer_input(year2016::day14::solve, &mut input),
        (2016, 15, _) => to_stringer_input(year2016::day15::solve, &mut input),
        (2016, 16, _) => to_stringer_input(year2016::day16::solve, &mut input),
        (2016, 17, _) => to_stringer_input(year2016::day17::solve, &mut input),
        (2016, 18, _) => to_stringer_input(year2016::day18::solve, &mut input),
        (2016, 19, _) => to_stringer_input(year2016::day19::solve, &mut input),
        (2016, 20, _) => to_stringer_input(year2016::day20::solve, &mut input),
        (2016, 21, _) => to_stringer_input(year2016::day21::solve, &mut input),
        (2016, 22, _) => to_stringer_input(year2016::day22::solve, &mut input),
        (2016, 23, _) => to_stringer_input(year2016::day23::solve, &mut input),
        (2016, 24, _) => to_stringer_input(year2016::day24::solve, &mut input),
        (2016, 25, _) => to_stringer_input(year2016::day25::solve, &mut input),
        (2017, 1, 1) => to_stringer(year2017::day01::part1, input_string),
        (2017, 1, 2) => to_stringer(year2017::day01::part2, input_string),
        (2017, 2, 1) => to_stringer(year2017::day02::part1, input_string),
        (2017, 2, 2) => to_stringer(year2017::day02::part2, input_string),
        (2017, 3, 1) => to_stringer(year2017::day03::part1, input_string),
        (2017, 3, 2) => to_stringer(year2017::day03::part2, input_string),
        (2017, 4, _) => to_stringer_input(year2017::day04::solve, &mut input),
        (2017, 5, 1) => to_stringer(year2017::day05::part1, input_string),
        (2017, 5, 2) => to_stringer(year2017::day05::part2, input_string),
        (2017, 6, 1) => to_stringer(year2017::day06::part1, input_string),
        (2017, 6, 2) => to_stringer(year2017::day06::part2, input_string),
        (2017, 7, 1) => to_stringer(year2017::day07::part1, input_string),
        (2017, 7, 2) => to_stringer(year2017::day07::part2, input_string),
        (2017, 8, _) => to_stringer_input(year2017::day08::solve, &mut input),
        (2017, 9, 1) => to_stringer(year2017::day09::part1, input_string),
        (2017, 9, 2) => to_stringer(year2017::day09::part2, input_string),
        (2017, 10, 1) => to_stringer(year2017::day10::part1, input_string),
        (2017, 10, 2) => to_stringer(year2017::day10::part2, input_string),
        (2017, 11, 1) => to_stringer(year2017::day11::part1, input_string),
        (2017, 11, 2) => to_stringer(year2017::day11::part2, input_string),
        (2017, 12, _) => to_stringer_input(year2017::day12::solve, &mut input),
        (2017, 13, _) => to_stringer_input(year2017::day13::solve, &mut input),
        (2017, 14, _) => to_stringer_input(year2017::day14::solve, &mut input),
        (2017, 15, _) => to_stringer_input(year2017::day15::solve, &mut input),
        (2017, 16, _) => to_stringer_input(year2017::day16::solve, &mut input),
        (2017, 17, _) => to_stringer_input(year2017::day17::solve, &mut input),
        (2017, 18, _) => to_stringer_input(year2017::day18::solve, &mut input),
        (2017, 19, _) => to_stringer_input(year2017::day19::solve, &mut input),
        (2017, 20, _) => to_stringer_input(year2017::day20::solve, &mut input),
        (2017, 21, _) => to_stringer_input(year2017::day21::solve, &mut input),
        (2017, 22, _) => to_stringer_input(year2017::day22::solve, &mut input),
        (2017, 23, _) => to_stringer_input(year2017::day23::solve, &mut input),
        (2017, 24, _) => to_stringer_input(year2017::day24::solve, &mut input),
        (2017, 25, _) => to_stringer_input(year2017::day25::solve, &mut input),
        (2018, 1, 1) => to_stringer(year2018::day01::part1, input_string),
        (2018, 1, 2) => to_stringer(year2018::day01::part2, input_string),
        (2018, 2, 1) => to_stringer(year2018::day02::part1, input_string),
        (2018, 2, 2) => to_stringer(year2018::day02::part2, input_string),
        (2018, 3, 1) => to_stringer(year2018::day03::part1, input_string),
        (2018, 3, 2) => to_stringer(year2018::day03::part2, input_string),
        (2018, 4, 1) => to_stringer(year2018::day04::part1, input_string),
        (2018, 4, 2) => to_stringer(year2018::day04::part2, input_string),
        (2018, 5, 1) => to_stringer(year2018::day05::part1, input_string),
        (2018, 5, 2) => to_stringer(year2018::day05::part2, input_string),
        (2018, 6, 1) => to_stringer(year2018::day06::part1, input_string),
        (2018, 6, 2) => to_stringer(year2018::day06::part2, input_string),
        (2018, 7, 1) => to_stringer(year2018::day07::part1, input_string),
        (2018, 7, 2) => to_stringer(year2018::day07::part2, input_string),
        (2018, 8, 1) => to_stringer(year2018::day08::part1, input_string),
        (2018, 8, 2) => to_stringer(year2018::day08::part2, input_string),
        (2018, 9, 1) => to_stringer(year2018::day09::part1, input_string),
        (2018, 9, 2) => to_stringer(year2018::day09::part2, input_string),
        (2018, 10, 1) => to_stringer(year2018::day10::part1, input_string),
        (2018, 10, 2) => to_stringer(year2018::day10::part2, input_string),
        (2018, 11, 1) => to_stringer(year2018::day11::part1, input_string),
        (2018, 11, 2) => to_stringer(year2018::day11::part2, input_string),
        (2018, 12, 1) => to_stringer(year2018::day12::part1, input_string),
        (2018, 12, 2) => to_stringer(year2018::day12::part2, input_string),
        (2018, 13, 1) => to_stringer(year2018::day13::part1, input_string),
        (2018, 13, 2) => to_stringer(year2018::day13::part2, input_string),
        (2018, 14, 1) => to_stringer(year2018::day14::part1, input_string),
        (2018, 14, 2) => to_stringer(year2018::day14::part2, input_string),
        (2018, 15, 1) => to_stringer(year2018::day15::part1, input_string),
        (2018, 15, 2) => to_stringer(year2018::day15::part2, input_string),
        (2018, 16, 1) => to_stringer(year2018::day16::part1, input_string),
        (2018, 16, 2) => to_stringer(year2018::day16::part2, input_string),
        (2018, 17, 1) => to_stringer(year2018::day17::part1, input_string),
        (2018, 17, 2) => to_stringer(year2018::day17::part2, input_string),
        (2018, 18, 1) => to_stringer(year2018::day18::part1, input_string),
        (2018, 18, 2) => to_stringer(year2018::day18::part2, input_string),
        (2018, 19, 1) => to_stringer(year2018::day19::part1, input_string),
        (2018, 19, 2) => to_stringer(year2018::day19::part2, input_string),
        (2018, 20, 1) => to_stringer(year2018::day20::part1, input_string),
        (2018, 20, 2) => to_stringer(year2018::day20::part2, input_string),
        (2018, 21, 1) => to_stringer(year2018::day21::part1, input_string),
        (2018, 21, 2) => to_stringer(year2018::day21::part2, input_string),
        (2018, 22, 1) => to_stringer(year2018::day22::part1, input_string),
        (2018, 22, 2) => to_stringer(year2018::day22::part2, input_string),
        (2018, 23, 1) => to_stringer(year2018::day23::part1, input_string),
        (2018, 23, 2) => to_stringer(year2018::day23::part2, input_string),
        (2018, 24, 1) => to_stringer(year2018::day24::part1, input_string),
        (2018, 24, 2) => to_stringer(year2018::day24::part2, input_string),
        (2018, 25, 1) => to_stringer(year2018::day25::part1, input_string),
        (2018, 25, 2) => to_stringer(year2018::day25::part2, input_string),
        (2019, 1, 1) => to_stringer(year2019::day01::part1, input_string),
        (2019, 1, 2) => to_stringer(year2019::day01::part2, input_string),
        (2019, 2, 1) => to_stringer(year2019::day02::part1, input_string),
        (2019, 2, 2) => to_stringer(year2019::day02::part2, input_string),
        (2019, 3, _) => to_stringer_input(year2019::day03::solve, &mut input),
        (2019, 4, 1) => to_stringer(year2019::day04::part1, input_string),
        (2019, 4, 2) => to_stringer(year2019::day04::part2, input_string),
        (2019, 5, 1) => to_stringer(year2019::day05::part1, input_string),
        (2019, 5, 2) => to_stringer(year2019::day05::part2, input_string),
        (2019, 6, 1) => to_stringer(year2019::day06::part1, input_string),
        (2019, 6, 2) => to_stringer(year2019::day06::part2, input_string),
        (2019, 7, _) => to_stringer_input(year2019::day07::solve, &mut input),
        (2019, 8, 1) => to_stringer(year2019::day08::part1, input_string),
        (2019, 8, 2) => to_stringer(year2019::day08::part2, input_string),
        (2019, 9, 1) => to_stringer(year2019::day09::part1, input_string),
        (2019, 9, 2) => to_stringer(year2019::day09::part2, input_string),
        (2019, 10, 1) => to_stringer(year2019::day10::part1, input_string),
        (2019, 10, 2) => to_stringer(year2019::day10::part2, input_string),
        (2019, 11, 1) => to_stringer(year2019::day11::part1, input_string),
        (2019, 11, 2) => to_stringer(year2019::day11::part2, input_string),
        (2019, 12, 1) => to_stringer(year2019::day12::part1, input_string),
        (2019, 12, 2) => to_stringer(year2019::day12::part2, input_string),
        (2019, 13, _) => to_stringer_input(year2019::day13::solve, &mut input),
        (2019, 14, 1) => to_stringer(year2019::day14::part1, input_string),
        (2019, 14, 2) => to_stringer(year2019::day14::part2, input_string),
        (2019, 15, 1) => to_stringer(year2019::day15::part1, input_string),
        (2019, 15, 2) => to_stringer(year2019::day15::part2, input_string),
        (2019, 16, 1) => to_stringer(year2019::day16::part1, input_string),
        (2019, 16, 2) => to_stringer(year2019::day16::part2, input_string),
        (2019, 17, 1) => to_stringer(year2019::day17::part1, input_string),
        (2019, 17, 2) => to_stringer(year2019::day17::part2, input_string),
        (2019, 18, _) => to_stringer_input(year2019::day18::solve, &mut input),
        (2019, 19, 1) => to_stringer(year2019::day19::part1, input_string),
        (2019, 19, 2) => to_stringer(year2019::day19::part2, input_string),
        (2019, 20, 1) => to_stringer(year2019::day20::part1, input_string),
        (2019, 20, 2) => to_stringer(year2019::day20::part2, input_string),
        (2019, 21, 1) => to_stringer(year2019::day21::part1, input_string),
        (2019, 21, 2) => to_stringer(year2019::day21::part2, input_string),
        (2019, 22, 1) => to_stringer(year2019::day22::part1, input_string),
        (2019, 22, 2) => to_stringer(year2019::day22::part2, input_string),
        (2019, 23, 1) => to_stringer(year2019::day23::part1, input_string),
        (2019, 23, 2) => to_stringer(year2019::day23::part2, input_string),
        (2019, 24, 1) => to_stringer(year2019::day24::part1, input_string),
        (2019, 24, 2) => to_stringer(year2019::day24::part2, input_string),
        (2019, 25, 1) => to_stringer(year2019::day25::part1, input_string),
        (2019, 25, 2) => to_stringer(year2019::day25::part2, input_string),
        (2020, 1, _) => to_stringer_input(year2020::day01::solve, &mut input),
        (2020, 2, _) => to_stringer_input(year2020::day02::solve, &mut input),
        (2020, 3, _) => to_stringer_input(year2020::day03::solve, &mut input),
        (2020, 4, _) => to_stringer_input(year2020::day04::solve, &mut input),
        (2020, 5, _) => to_stringer_input(year2020::day05::solve, &mut input),
        (2020, 6, _) => to_stringer_input(year2020::day06::solve, &mut input),
        (2020, 7, _) => to_stringer_input(year2020::day07::solve, &mut input),
        (2020, 8, _) => to_stringer_input(year2020::day08::solve, &mut input),
        (2020, 9, _) => to_stringer_input(year2020::day09::solve, &mut input),
        (2020, 10, _) => to_stringer_input(year2020::day10::solve, &mut input),
        (2020, 11, _) => to_stringer_input(year2020::day11::solve, &mut input),
        (2020, 12, _) => to_stringer_input(year2020::day12::solve, &mut input),
        (2020, 13, _) => to_stringer_input(year2020::day13::solve, &mut input),
        (2020, 14, _) => to_stringer_input(year2020::day14::solve, &mut input),
        (2020, 15, _) => to_stringer_input(year2020::day15::solve, &mut input),
        (2020, 16, _) => to_stringer_input(year2020::day16::solve, &mut input),
        (2020, 17, _) => to_stringer_input(year2020::day17::solve, &mut input),
        (2020, 18, _) => to_stringer_input(year2020::day18::solve, &mut input),
        (2020, 19, _) => to_stringer_input(year2020::day19::solve, &mut input),
        (2020, 20, _) => to_stringer_input(year2020::day20::solve, &mut input),
        (2020, 21, _) => to_stringer_input(year2020::day21::solve, &mut input),
        (2020, 22, _) => to_stringer_input(year2020::day22::solve, &mut input),
        (2020, 23, _) => to_stringer_input(year2020::day23::solve, &mut input),
        (2020, 24, _) => to_stringer_input(year2020::day24::solve, &mut input),
        (2020, 25, _) => to_stringer_input(year2020::day25::solve, &mut input),
        _ => Err(format!(
            "Unsupported year={}, day={}, part={}",
            year, day, part
        )),
    };

    #[cfg(feature = "visualization")]
    if result.is_err() {
        // TODO: Report error. But perhaps not, return normally, and await ack in wait_forever.
        input.painter.shadow_blur(10);
    }

    result
}

/// A version of [solve](fn.solve.html) that takes strings as arguments and parses them to the required types.
pub fn solve_raw(
    year_string: &str,
    day_string: &str,
    part_string: &str,
    input: &str,
    #[cfg(feature = "visualization")] painter: PainterRef,
) -> Result<String, String> {
    let year = year_string.parse::<u16>().map_err(|_| "Invalid year")?;
    let day = day_string.parse::<u8>().map_err(|_| "Invalid day")?;
    let part = part_string.parse::<u8>().map_err(|_| "Invalid part")?;
    solve(
        year,
        day,
        part,
        input,
        #[cfg(feature = "visualization")]
        painter,
    )
}
