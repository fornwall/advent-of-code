/*!
This crates provides solutions for Advent of Code problems.
*/
#![crate_name = "advent_of_code"]

mod year2017;
mod year2018;
mod year2019;

fn to_stringer<T: ToString>(
    function: fn(&str) -> Result<T, String>,
    input: &str,
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
pub fn solve(year: u16, day: u8, part: u8, input: &str) -> Result<String, String> {
    if input.is_empty() {
        return Err("Empty input".to_string());
    }

    match (year, day, part) {
        (2017, 1, 1) => to_stringer(year2017::day01::part1, input),
        (2017, 1, 2) => to_stringer(year2017::day01::part2, input),
        (2017, 2, 1) => to_stringer(year2017::day02::part1, input),
        (2017, 2, 2) => to_stringer(year2017::day02::part2, input),
        (2017, 3, 1) => to_stringer(year2017::day03::part1, input),
        (2017, 3, 2) => to_stringer(year2017::day03::part2, input),
        (2017, 4, 1) => to_stringer(year2017::day04::part1, input),
        (2017, 4, 2) => to_stringer(year2017::day04::part2, input),
        (2017, 5, 1) => to_stringer(year2017::day05::part1, input),
        (2017, 5, 2) => to_stringer(year2017::day05::part2, input),
        (2017, 6, 1) => to_stringer(year2017::day06::part1, input),
        (2017, 6, 2) => to_stringer(year2017::day06::part2, input),
        (2017, 7, 1) => to_stringer(year2017::day07::part1, input),
        (2017, 7, 2) => to_stringer(year2017::day07::part2, input),
        (2017, 8, 1) => to_stringer(year2017::day08::part1, input),
        (2017, 8, 2) => to_stringer(year2017::day08::part2, input),
        (2017, 9, 1) => to_stringer(year2017::day09::part1, input),
        (2017, 9, 2) => to_stringer(year2017::day09::part2, input),
        (2017, 10, 1) => to_stringer(year2017::day10::part1, input),
        (2017, 10, 2) => to_stringer(year2017::day10::part2, input),
        (2018, 1, 1) => to_stringer(year2018::day01::part1, input),
        (2018, 1, 2) => to_stringer(year2018::day01::part2, input),
        (2018, 2, 1) => to_stringer(year2018::day02::part1, input),
        (2018, 2, 2) => to_stringer(year2018::day02::part2, input),
        (2018, 3, 1) => to_stringer(year2018::day03::part1, input),
        (2018, 3, 2) => to_stringer(year2018::day03::part2, input),
        (2018, 4, 1) => to_stringer(year2018::day04::part1, input),
        (2018, 4, 2) => to_stringer(year2018::day04::part2, input),
        (2018, 5, 1) => to_stringer(year2018::day05::part1, input),
        (2018, 5, 2) => to_stringer(year2018::day05::part2, input),
        (2018, 6, 1) => to_stringer(year2018::day06::part1, input),
        (2018, 6, 2) => to_stringer(year2018::day06::part2, input),
        (2018, 7, 1) => to_stringer(year2018::day07::part1, input),
        (2018, 7, 2) => to_stringer(year2018::day07::part2, input),
        (2018, 8, 1) => to_stringer(year2018::day08::part1, input),
        (2018, 8, 2) => to_stringer(year2018::day08::part2, input),
        (2018, 9, 1) => to_stringer(year2018::day09::part1, input),
        (2018, 9, 2) => to_stringer(year2018::day09::part2, input),
        (2018, 10, 1) => to_stringer(year2018::day10::part1, input),
        (2018, 10, 2) => to_stringer(year2018::day10::part2, input),
        (2018, 11, 1) => to_stringer(year2018::day11::part1, input),
        (2018, 11, 2) => to_stringer(year2018::day11::part2, input),
        (2018, 12, 1) => to_stringer(year2018::day12::part1, input),
        (2018, 12, 2) => to_stringer(year2018::day12::part2, input),
        (2018, 13, 1) => to_stringer(year2018::day13::part1, input),
        (2018, 13, 2) => to_stringer(year2018::day13::part2, input),
        (2018, 14, 1) => to_stringer(year2018::day14::part1, input),
        (2018, 14, 2) => to_stringer(year2018::day14::part2, input),
        (2018, 15, 1) => to_stringer(year2018::day15::part1, input),
        (2018, 15, 2) => to_stringer(year2018::day15::part2, input),
        (2018, 16, 1) => to_stringer(year2018::day16::part1, input),
        (2018, 16, 2) => to_stringer(year2018::day16::part2, input),
        (2018, 17, 1) => to_stringer(year2018::day17::part1, input),
        (2018, 17, 2) => to_stringer(year2018::day17::part2, input),
        (2018, 18, 1) => to_stringer(year2018::day18::part1, input),
        (2018, 18, 2) => to_stringer(year2018::day18::part2, input),
        (2018, 19, 1) => to_stringer(year2018::day19::part1, input),
        (2018, 19, 2) => to_stringer(year2018::day19::part2, input),
        (2018, 20, 1) => to_stringer(year2018::day20::part1, input),
        (2018, 20, 2) => to_stringer(year2018::day20::part2, input),
        (2018, 21, 1) => to_stringer(year2018::day21::part1, input),
        (2018, 21, 2) => to_stringer(year2018::day21::part2, input),
        (2018, 22, 1) => to_stringer(year2018::day22::part1, input),
        (2018, 22, 2) => to_stringer(year2018::day22::part2, input),
        (2018, 23, 1) => to_stringer(year2018::day23::part1, input),
        (2018, 23, 2) => to_stringer(year2018::day23::part2, input),
        (2018, 24, 1) => to_stringer(year2018::day24::part1, input),
        (2018, 24, 2) => to_stringer(year2018::day24::part2, input),
        (2018, 25, 1) => to_stringer(year2018::day25::part1, input),
        (2018, 25, 2) => to_stringer(year2018::day25::part2, input),
        (2019, 1, 1) => to_stringer(year2019::day01::part1, input),
        (2019, 1, 2) => to_stringer(year2019::day01::part2, input),
        (2019, 2, 1) => to_stringer(year2019::day02::part1, input),
        (2019, 2, 2) => to_stringer(year2019::day02::part2, input),
        (2019, 3, 1) => to_stringer(year2019::day03::part1, input),
        (2019, 3, 2) => to_stringer(year2019::day03::part2, input),
        (2019, 4, 1) => to_stringer(year2019::day04::part1, input),
        (2019, 4, 2) => to_stringer(year2019::day04::part2, input),
        (2019, 5, 1) => to_stringer(year2019::day05::part1, input),
        (2019, 5, 2) => to_stringer(year2019::day05::part2, input),
        (2019, 6, 1) => to_stringer(year2019::day06::part1, input),
        (2019, 6, 2) => to_stringer(year2019::day06::part2, input),
        (2019, 7, 1) => to_stringer(year2019::day07::part1, input),
        (2019, 7, 2) => to_stringer(year2019::day07::part2, input),
        (2019, 8, 1) => to_stringer(year2019::day08::part1, input),
        (2019, 8, 2) => to_stringer(year2019::day08::part2, input),
        (2019, 9, 1) => to_stringer(year2019::day09::part1, input),
        (2019, 9, 2) => to_stringer(year2019::day09::part2, input),
        (2019, 10, 1) => to_stringer(year2019::day10::part1, input),
        (2019, 10, 2) => to_stringer(year2019::day10::part2, input),
        (2019, 11, 1) => to_stringer(year2019::day11::part1, input),
        (2019, 11, 2) => to_stringer(year2019::day11::part2, input),
        (2019, 12, 1) => to_stringer(year2019::day12::part1, input),
        (2019, 12, 2) => to_stringer(year2019::day12::part2, input),
        (2019, 13, 1) => to_stringer(year2019::day13::part1, input),
        (2019, 13, 2) => to_stringer(year2019::day13::part2, input),
        (2019, 14, 1) => to_stringer(year2019::day14::part1, input),
        (2019, 14, 2) => to_stringer(year2019::day14::part2, input),
        (2019, 15, 1) => to_stringer(year2019::day15::part1, input),
        (2019, 15, 2) => to_stringer(year2019::day15::part2, input),
        (2019, 16, 1) => to_stringer(year2019::day16::part1, input),
        (2019, 16, 2) => to_stringer(year2019::day16::part2, input),
        (2019, 17, 1) => to_stringer(year2019::day17::part1, input),
        (2019, 17, 2) => to_stringer(year2019::day17::part2, input),
        (2019, 18, 1) => to_stringer(year2019::day18::part1, input),
        (2019, 18, 2) => to_stringer(year2019::day18::part2, input),
        (2019, 19, 1) => to_stringer(year2019::day19::part1, input),
        (2019, 19, 2) => to_stringer(year2019::day19::part2, input),
        (2019, 20, 1) => to_stringer(year2019::day20::part1, input),
        (2019, 20, 2) => to_stringer(year2019::day20::part2, input),
        (2019, 21, 1) => to_stringer(year2019::day21::part1, input),
        (2019, 21, 2) => to_stringer(year2019::day21::part2, input),
        (2019, 22, 1) => to_stringer(year2019::day22::part1, input),
        (2019, 22, 2) => to_stringer(year2019::day22::part2, input),
        (2019, 23, 1) => to_stringer(year2019::day23::part1, input),
        (2019, 23, 2) => to_stringer(year2019::day23::part2, input),
        (2019, 24, 1) => to_stringer(year2019::day24::part1, input),
        (2019, 24, 2) => to_stringer(year2019::day24::part2, input),
        (2019, 25, 1) => to_stringer(year2019::day25::part1, input),
        (2019, 25, 2) => to_stringer(year2019::day25::part2, input),
        _ => Err(format!(
            "Unsupported year={}, day={}, part={}",
            year, day, part
        )),
    }
}

/// A version of [solve](fn.solve.html) that takes strings as arguments and parses them to the required types.
pub fn solve_raw(
    year_string: &str,
    day_string: &str,
    part_string: &str,
    input: &str,
) -> Result<String, String> {
    let year = year_string.parse::<u16>().map_err(|_| "Invalid year")?;
    let day = day_string.parse::<u8>().map_err(|_| "Invalid day")?;
    let part = part_string.parse::<u8>().map_err(|_| "Invalid part")?;
    solve(year, day, part, input)
}
