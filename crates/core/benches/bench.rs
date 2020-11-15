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

run_bench!(2017, 01);
run_bench!(2017, 02);
run_bench!(2017, 03);
run_bench!(2017, 04);
run_bench!(2017, 05);
run_bench!(2017, 06);
run_bench!(2017, 07);
run_bench!(2017, 08);
run_bench!(2017, 09);
run_bench!(2017, 10);
run_bench!(2017, 11);
run_bench!(2017, 12);
run_bench!(2017, 13);

run_bench!(2018, 01);
run_bench!(2018, 02);
run_bench!(2018, 03);
run_bench!(2018, 04);
run_bench!(2018, 05);
run_bench!(2018, 06);
run_bench!(2018, 07);
run_bench!(2018, 08);
run_bench!(2018, 09);
run_bench!(2018, 10);
run_bench!(2018, 11);
run_bench!(2018, 12);
run_bench!(2018, 13);
run_bench!(2018, 14);
run_bench!(2018, 15);
run_bench!(2018, 16);
run_bench!(2018, 17);
run_bench!(2018, 18);
run_bench!(2018, 19);
run_bench!(2018, 20);
run_bench!(2018, 21);
run_bench!(2018, 22);
run_bench!(2018, 23);
run_bench!(2018, 24);
run_bench!(2018, 25);

run_bench!(2019, 01);
run_bench!(2019, 02);
run_bench!(2019, 03);
run_bench!(2019, 04);
run_bench!(2019, 05);
run_bench!(2019, 06);
run_bench!(2019, 07);
run_bench!(2019, 08);
run_bench!(2019, 09);
run_bench!(2019, 10);
run_bench!(2019, 11);
run_bench!(2019, 12);
run_bench!(2019, 13);
run_bench!(2019, 14);
run_bench!(2019, 15);
run_bench!(2019, 16);
run_bench!(2019, 17);
run_bench!(2019, 18);
run_bench!(2019, 19);
run_bench!(2019, 20);
run_bench!(2019, 21);
run_bench!(2019, 22);
run_bench!(2019, 23);
run_bench!(2019, 24);
run_bench!(2019, 25);
