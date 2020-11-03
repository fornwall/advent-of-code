#[macro_use]
extern crate honggfuzz;
use advent_of_code::solve;

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            for year in 2018..=2019 {
                for day in 1..=25 {
                    for part in 1..=2 {
                        if let Ok(input_string) = std::str::from_utf8(data) {
                            let _ = solve(year, day, part, input_string);
                        }
                    }
                }
            }
        });
    }
}
