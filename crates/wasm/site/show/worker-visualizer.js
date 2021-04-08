"use strict";
self.importScripts("generated/advent_of_code_wasm.js");

self.onmessage = async (message) => {
  try {
    const { year, day, part, input } = message.data;

    await self.wasmReadyPromise;
    // const wasm = await self.wasmReadyPromise;
    // If building with atomics, "memory" gets mangled to "__wbindgen_export_0":
    // https://github.com/rustwasm/wasm-bindgen/issues/2114
    // const wasmMemory = wasm.memory ? wasm.memory : wasm.__wbindgen_export_0;
    // console.log('memory', wasmMemory);
    // const memoryArray = new Int32Array(wasmMemory.buffer);
    // console.log('memory len', memoryArray.length);
    // self.do_wait = (offset, value) => {
    //   console.log('do_wait', offset, value);
    //   Atomics.wait(memoryArray, offset, value);
    // }

    const answer = wasm_bindgen.solve(year, day, part, input);
    self.postMessage({ done: true, answer });
  } catch (e) {
    console.log(e);
    self.postMessage({ errorMessage: e.message });
  }
};

self.wasmReadyPromise = (async () => {
  try {
    return await wasm_bindgen("generated/advent_of_code_wasm_bg.wasm");
  } catch (e) {
    throw new Error("WebAssembly not working - " + e.message);
  }
  return result;
})();

(async () => {
  await self.wasmReadyPromise;
})();
