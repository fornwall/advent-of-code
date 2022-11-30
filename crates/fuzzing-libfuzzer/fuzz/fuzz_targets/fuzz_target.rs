#![no_main]
use libfuzzer_sys::fuzz_target;

use advent_of_code::solve;

fuzz_target!(|data: &[u8]| {
    if data.len() >= 4 {
        if let Ok(input_string) = std::str::from_utf8(&data[3..]) {
            let year = 2015 + (data[0] % 8) as u16;
            let day = 1 + data[1] % 25;
            let part = 1 + data[2] % 2;
            let _ = solve(year, day, part, input_string);
        }
    }
});
