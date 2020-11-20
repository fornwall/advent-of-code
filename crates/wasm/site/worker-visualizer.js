'use strict';
self.importScripts("/show/advent_of_code_wasm.js");

self.onmessage = async (message) => {
    await wasm_bindgen("/show/advent_of_code_wasm_bg.wasm");

    const { year, day, part, input } = message.data;

    wasm_bindgen.solve(year, day, part, input);
}
