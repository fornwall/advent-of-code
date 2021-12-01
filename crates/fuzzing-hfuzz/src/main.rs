#[macro_use]
extern crate honggfuzz;
use advent_of_code::solve;

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            if data.len() >= 4 {
                if let Ok(input_string) = std::str::from_utf8(&data[3..]) {
                    let year = 2015 + u16::from(data[0] % 7);
                    let day = 1 + data[1] % 25;
                    let part = 1 + data[2] % 2;
                    let _ = solve(year, day, part, input_string);
                }
            }
        });
    }
}
