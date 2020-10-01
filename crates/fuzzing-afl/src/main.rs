#[macro_use]
extern crate afl;
use advent_of_code::solve;

fn main() {
    fuzz!(|data: &[u8]| {
        //for year in 2018..=2018 {
        //for day in 11..=11 {
        //for part in 1..=1 {
        if let Ok(input_string) = std::str::from_utf8(data) {
            let year = 2018;
            let day = 11;
            let part = 1;
            let _ = solve(year, day, part, input_string);
        }
        //}
        //}
        //}
    });
}
