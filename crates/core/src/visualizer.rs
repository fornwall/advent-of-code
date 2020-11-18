use js_sys::Promise;
pub use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::OffscreenCanvas;

#[wasm_bindgen(inline_js = "export function get_canvas() { return self.canvas; }")]
extern "C" {
    fn get_canvas() -> CanvasElement;
}

#[wasm_bindgen(inline_js = "export function get_promise() { return self.promise; }")]
#[cfg_attr(rustfmt, rustfmt_skip)]
extern "C" {
    pub fn get_promise() -> Promise;
}

#[wasm_bindgen]
extern "C" {
    pub type CanvasElement;

    #[wasm_bindgen(structural, method)]
    pub fn getContext(this: &CanvasElement, name: &str) -> CanvasContext;

    #[wasm_bindgen(method, getter, structural)]
    fn width(this: &CanvasElement) -> u32;

    #[wasm_bindgen(method, getter, structural)]
    fn height(this: &CanvasElement) -> u32;
}

#[wasm_bindgen]
extern "C" {
    pub type CanvasContext;

    #[wasm_bindgen(structural, method)]
    pub fn beginPath(this: &CanvasContext);

    #[wasm_bindgen(structural, method)]
    pub fn closePath(this: &CanvasContext);

    #[wasm_bindgen(structural, method)]
    pub fn stroke(this: &CanvasContext);

    #[wasm_bindgen(structural, method)]
    pub fn arc(this: &CanvasContext, a: f64, b: f64, c: f64, d: f64, e: f64);

    #[wasm_bindgen(structural, method)]
    pub fn moveTo(this: &CanvasContext, a: f64, b: f64);

    #[wasm_bindgen(structural, method)]
    pub fn clearRect(this: &CanvasContext, a: u32, b: u32, c: u32, d: u32);

    #[wasm_bindgen(structural, method, getter)]
    pub fn canvas(this: &CanvasContext) -> CanvasElement;
}

pub fn get_drawing_context() -> CanvasContext {
    let canvas = get_canvas();
    let js: JsValue = canvas.width().into();
    //web_sys::console::log_2(&"Hello using web-sys".into(), &js);
    canvas.getContext("2d")
}

pub fn clear(context: &CanvasContext) {
    let canvas = context.canvas();
    context.clearRect(0, 0, canvas.width(), canvas.height());
}
