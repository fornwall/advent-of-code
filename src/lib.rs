#[cfg(target_arch = "wasm32")]
extern crate wasm_bindgen;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

pub fn get_problem_set(day: u8, part: u8) -> Option<fn(&str) -> String> {
    struct Solutions(fn(&str) -> String, fn(&str) -> String);

    let parts: Solutions = match day {
        1 => Solutions(day01::part1, day01::part2),
        2 => Solutions(day02::part1, day02::part2),
        3 => Solutions(day03::part1, day03::part2),
        4 => Solutions(day04::part1, day04::part2),
        5 => Solutions(day05::part1, day05::part2),
        6 => Solutions(day06::part1, day06::part2),
        7 => Solutions(day07::part1, day07::part2),
        8 => Solutions(day08::part1, day08::part2),
        9 => Solutions(day09::part1, day09::part2),
        10 => Solutions(day10::part1, day10::part2),
        11 => Solutions(day11::part1, day11::part2),
        12 => Solutions(day12::part1, day12::part2),
        13 => Solutions(day13::part1, day13::part2),
        14 => Solutions(day14::part1, day14::part2),
        15 => Solutions(day15::part1, day15::part2),
        16 => Solutions(day16::part1, day16::part2),
        17 => Solutions(day17::part1, day17::part2),
        18 => Solutions(day18::part1, day18::part2),
        19 => Solutions(day19::part1, day19::part2),
        20 => Solutions(day20::part1, day20::part2),
        21 => Solutions(day21::part1, day21::part2),
        22 => Solutions(day22::part1, day22::part2),
        23 => Solutions(day23::part1, day23::part2),
        24 => Solutions(day24::part1, day24::part2),
        25 => Solutions(day25::part1, day25::part2),
        _ => {
            return None;
        }
    };
    match part {
        1 => Some(parts.0),
        2 => Some(parts.1),
        _ => None,
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn solve(day: u8, part: u8, input: String) -> String {
    if let Some(solver) = get_problem_set(day, part) {
        solver(&input)
    } else {
        format!("Solution for day {} part {} not implemented yet", day, part)
    }
}
