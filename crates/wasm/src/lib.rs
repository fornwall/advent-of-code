#![allow(clippy::unused_unit)]
extern crate js_sys;
extern crate wasm_bindgen;

use advent_of_code::solve_raw;
use wasm_bindgen::prelude::*;

fn as_string(value: &JsValue) -> String {
    value.as_string().unwrap_or_else(|| {
        value
            .as_f64()
            .map_or_else(|| "".to_string(), |number| number.to_string())
    })
}

#[wasm_bindgen]
pub fn solve(
    year: &JsValue,
    day: &JsValue,
    part: &JsValue,
    input: &str,
) -> Result<advent_of_code::ResultType, JsValue> {
    #[cfg(feature = "console-panic-hook")]
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    let year = as_string(year);
    let day = as_string(day);
    let part = as_string(part);
    solve_raw(&year, &day, &part, input).map_err(|error| JsValue::from(js_sys::Error::new(&error)))
}
