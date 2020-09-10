extern crate wasm_bindgen;
use advent_of_code::get_problem_set;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn solve(year: u16, day: u8, part: u8, input: String) -> String {
    if let Some(solver) = get_problem_set(year, day, part) {
        solver(&input)
    } else {
        format!(
            "Day ({}) must be between 1 and 25 and part ({}) either 1 or 2",
            day, part
        )
    }
}
