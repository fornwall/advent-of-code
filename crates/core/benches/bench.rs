#![feature(test, concat_idents)]
#![allow(clippy::zero_prefixed_literal)]
extern crate test;

use advent_of_code::solve;
use paste::paste;
use std::fs::read_to_string;
use test::Bencher;

fn solve_parts(b: &mut Bencher, year: u16, day: u8, part: u8) {
    #![allow(clippy::unwrap_used)]
    let input_path = format!("src/year{}/day{:02}_input.txt", year, day);
    let input = read_to_string(input_path).unwrap();
    b.iter(|| {
        solve(year, day, part, &input).unwrap();
    });
}

macro_rules! run_bench {
    ($year: literal, $day: literal) => {
        paste! {
           #[bench]
            fn [<problem_ $year _ $day _ part1>](b: &mut Bencher) {
                solve_parts(b, $year, $day, 1);
            }
           #[bench]
            fn [<problem_ $year _ $day _ part2>](b: &mut Bencher) {
                solve_parts(b, $year, $day, 2);
            }
        }
    };
}

run_bench!(2019, 01);
run_bench!(2019, 02);
run_bench!(2019, 03);
run_bench!(2019, 04);
run_bench!(2019, 05);
