'use strict';
self.importScripts("advent_of_code_wasm.js");

self.onmessage = async (message) => {
    await wasm_bindgen("advent_of_code_wasm_bg.wasm");

    const { year, day, part, input } = message.data;

    wasm_bindgen.solve(year, day, part, input);
}
