extern crate js_sys;
extern crate wasm_bindgen;
use advent_of_code::solve_raw;
#[cfg(feature = "visualization")]
use advent_of_code_painter::drawer::CommandBufferPainter;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

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
    input: String,
) -> Result<String, JsValue> {
    #[cfg(feature = "console-panic-hook")]
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    #[cfg(feature = "visualization")]
    let painter = Box::new(CommandBufferPainter::new());

    let year = as_string(year);
    let day = as_string(day);
    let part = as_string(part);
    solve_raw(
        &year,
        &day,
        &part,
        &input,
        #[cfg(feature = "visualization")]
        painter,
    )
    .map_err(|error| JsValue::from(js_sys::Error::new(&error)))
}
