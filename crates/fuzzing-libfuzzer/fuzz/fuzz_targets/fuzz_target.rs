#![no_main]
use libfuzzer_sys::fuzz_target;

use advent_of_code::solve;

fuzz_target!(|input_string: String| {
    for year in 2018..=2019 {
        for day in 1..=25 {
            for part in 1..=2 {
                let _ = solve(year, day, part, &input_string);
            }
        }
    }
});
