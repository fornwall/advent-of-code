"use strict";
self.importScripts("/generated/advent_of_code_wasm.js");

self.onmessage = async (message) => {
  const { year, day, part, input, wasm } = message.data;

  const startTime = performance.now();

  try {
    if (wasm) {
      await self.wasmReadyPromise;

      if (!self.wasmWorking) {
        postMessage({ output: "Wasm is not working", isError: true });
        return;
      }
      const output = wasm_bindgen.solve(year, day, part, input);
      const executionTime = performance.now() - startTime;
      postMessage({ output, isError: false, wasm, executionTime });
    } else {
      const response = await fetch(
        `https://advent.fly.dev/solve/${year}/${day}/${part}`,
        {
          method: "POST",
          headers: { "content-type": "text/plain" },
          body: input,
        }
      );
      const responseText = await response.text();
      const executionTime = performance.now() - startTime;
      postMessage({
        output: responseText,
        isError: !response.ok,
        wasm,
        executionTime,
      });
    }
  } catch (error) {
    console.error(error);
    const executionTime = performance.now() - startTime;
    postMessage({ output: error.message, isError: true, wasm, executionTime });
  }
};

self.wasmReadyPromise = (async () => {
  try {
    await wasm_bindgen("/generated/advent_of_code_wasm_bg.wasm");
    self.wasmWorking = true;
  } catch (e) {
    console.warn("WebAssembly not working", e);
    self.wasmWorking = false;
  }
  self.postMessage({ wasmWorking: self.wasmWorking });
})();

(async () => {
  await self.wasmReadyPromise;
})();
