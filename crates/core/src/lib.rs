use std::fmt::Display;

mod year2018;
mod year2019;

fn to_stringer<T: Display>(
    function: fn(&str) -> Result<T, String>,
    input: &str,
) -> Result<String, String> {
    match function(input) {
        Ok(value) => Ok(value.to_string()),
        Err(error) => Err(error),
    }
}

pub fn solve(year: u16, day: u8, part: u8, input: &str) -> Result<String, String> {
    match (year, day, part) {
        (2019, 1, 1) => to_stringer(year2019::day01::part1, input),
        (2019, 1, 2) => to_stringer(year2019::day01::part2, input),
        (2019, 2, 1) => to_stringer(year2019::day02::part1, input),
        (2019, 2, 2) => to_stringer(year2019::day02::part2, input),
        (2019, 4, 1) => to_stringer(year2019::day04::part1, input),
        (2019, 4, 2) => to_stringer(year2019::day04::part2, input),
        (2019, 5, 1) => to_stringer(year2019::day05::part1, input),
        (2019, 5, 2) => to_stringer(year2019::day05::part2, input),
        (2019, 6, 1) => to_stringer(year2019::day06::part1, input),
        (2019, 6, 2) => to_stringer(year2019::day06::part2, input),
        (2019, 9, 1) => to_stringer(year2019::day09::part1, input),
        (2019, 9, 2) => to_stringer(year2019::day09::part2, input),
        (2019, 11, 1) => to_stringer(year2019::day11::part1, input),
        (2019, 11, 2) => to_stringer(year2019::day11::part2, input),
        (2019, 12, 1) => to_stringer(year2019::day12::part1, input),
        (2019, 12, 2) => to_stringer(year2019::day12::part2, input),
        _ => Err(format!(
            "Unsupported year={}, day={}, part={}",
            year, day, part
        )),
    }

    /*
    let parts: Solutions = if year == 2018 {
        match day {
            1 => Solutions(year2018::day01::part1, year2018::day01::part2),
            2 => Solutions(year2018::day02::part1, year2018::day02::part2),
            3 => Solutions(year2018::day03::part1, year2018::day03::part2),
            4 => Solutions(year2018::day04::part1, year2018::day04::part2),
            5 => Solutions(year2018::day05::part1, year2018::day05::part2),
            6 => Solutions(year2018::day06::part1, year2018::day06::part2),
            7 => Solutions(year2018::day07::part1, year2018::day07::part2),
            8 => Solutions(year2018::day08::part1, year2018::day08::part2),
            9 => Solutions(year2018::day09::part1, year2018::day09::part2),
            10 => Solutions(year2018::day10::part1, year2018::day10::part2),
            11 => Solutions(year2018::day11::part1, year2018::day11::part2),
            12 => Solutions(year2018::day12::part1, year2018::day12::part2),
            13 => Solutions(year2018::day13::part1, year2018::day13::part2),
            14 => Solutions(year2018::day14::part1, year2018::day14::part2),
            15 => Solutions(year2018::day15::part1, year2018::day15::part2),
            16 => Solutions(year2018::day16::part1, year2018::day16::part2),
            17 => Solutions(year2018::day17::part1, year2018::day17::part2),
            18 => Solutions(year2018::day18::part1, year2018::day18::part2),
            19 => Solutions(year2018::day19::part1, year2018::day19::part2),
            20 => Solutions(year2018::day20::part1, year2018::day20::part2),
            21 => Solutions(year2018::day21::part1, year2018::day21::part2),
            22 => Solutions(year2018::day22::part1, year2018::day22::part2),
            23 => Solutions(year2018::day23::part1, year2018::day23::part2),
            24 => Solutions(year2018::day24::part1, year2018::day24::part2),
            25 => Solutions(year2018::day25::part1, year2018::day25::part2),
            _ => {
                return None;
            }
        }
    } else if year == 2019 {
        match (day, part) {
            1 => Solutions(year2019::day01::part1, year2019::day01::part2),
            2 => Solutions(year2019::day02::part1, year2019::day02::part2),
            3 => Solutions(year2019::day03::part1, year2019::day03::part2),
            4 => Solutions(year2019::day04::part1, year2019::day04::part2),
            5 => Solutions(year2019::day05::part1, year2019::day05::part2),
            6 => Solutions(year2019::day06::part1, year2019::day06::part2),
            7 => Solutions(year2019::day07::part1, year2019::day07::part2),
            8 => Solutions(year2019::day08::part1, year2019::day08::part2),
            9 => Solutions(year2019::day09::part1, year2019::day09::part2),
            10 => Solutions(year2019::day10::part1, year2019::day10::part2),
            11 => Solutions(year2019::day11::part1, year2019::day11::part2),
            12 => Solutions(year2019::day12::part1, year2019::day12::part2),
            13 => Solutions(year2019::day13::part1, year2019::day13::part2),
            14 => Solutions(year2019::day14::part1, year2019::day14::part2),
            15 => Solutions(year2019::day15::part1, year2019::day15::part2),
            16 => Solutions(year2019::day16::part1, year2019::day16::part2),
            17 => Solutions(year2019::day17::part1, year2019::day17::part2),
            18 => Solutions(year2019::day18::part1, year2019::day18::part2),
            19 => Solutions(year2019::day19::part1, year2019::day19::part2),
            20 => Solutions(year2019::day20::part1, year2019::day20::part2),
            21 => Solutions(year2019::day21::part1, year2019::day21::part2),
            22 => Solutions(year2019::day22::part1, year2019::day22::part2),
            23 => Solutions(year2019::day23::part1, year2019::day23::part2),
            24 => Solutions(year2019::day24::part1, year2019::day24::part2),
            25 => Solutions(year2019::day25::part1, year2019::day25::part2),
            _ => {
                return None;
            }
        }
    } else {
        return None;
    };

    match part {
        1 => Some(parts.0),
        2 => Some(parts.1),
        _ => None,
    }
    */
}
