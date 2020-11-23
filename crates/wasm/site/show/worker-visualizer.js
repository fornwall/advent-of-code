'use strict';
self.importScripts("advent_of_code_wasm.js");

self.onmessage = async (message) => {
    const { year, day, part, input } = message.data;
    const wasm = await wasm_bindgen("advent_of_code_wasm_bg.wasm");

    // If building with atomics, "memory" gets mangled to "__wbindgen_export_0":
    // https://github.com/rustwasm/wasm-bindgen/issues/2114
    // const wasm_memory = wasm.memory ? wasm.memory : wasm.__wbindgen_export_0;
    // const memory_array = new Int32Array(wasm_memory.buffer);
    // self.do_wait = (offset, value) => {
    //    Atomics.wait(memory_array, offset, value);
    //}

    wasm_bindgen.solve(year, day, part, input);
}
