extern crate wasm_bindgen;
use advent_of_code_rs::get_problem_set;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn solve(day: u8, part: u8, input: String) -> String {
    if let Some(solver) = get_problem_set(day, part) {
        solver(&input)
    } else {
        format!(
            "Day ({}) must be between 1 and 25 and part ({}) either 1 or 2",
            day, part
        )
    }
}
