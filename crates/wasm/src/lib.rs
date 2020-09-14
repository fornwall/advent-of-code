extern crate wasm_bindgen;
use advent_of_code::solve as core_solve;
use js_sys;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn solve(year: u16, day: u8, part: u8, input: String) -> Result<String, JsValue> {
    match core_solve(year, day, part, &input) {
        Ok(value) => Ok(value),
        Err(error) => Err(js_sys::Error::new(&error).into()),
    }
}
