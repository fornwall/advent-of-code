self.importScripts("/advent_of_code_wasm.js");

async function run() {
  let wasmWorking;
  try {
    await wasm_bindgen("/advent_of_code_wasm_bg.wasm");
    wasmWorking = true;
  } catch (e) {
    console.warn('WebAssembly not working', e);
    wasmWorking = false;
  }

  self.postMessage({wasmWorking});

  self.onmessage = async (message) => {
    const { year, day, part, input, wasm } = message.data;
    const startTime = performance.now();

    try {
      if (wasm) {
        if (!wasmWorking) {
          postMessage({output: "Wasm is not working", isError: true});
          return;
        }
        const output = wasm_bindgen.solve(year, day, part, input);
        const executionTime = performance.now() - startTime;
        postMessage({output, isError: false, wasm, executionTime});
      } else {
        const response = await fetch(`https://aoc.fly.dev/solve/${year}/${day}/${part}/`, {
          method: "POST",
          headers: { "content-type": "text/plain" },
          body: input
        });
        const responseText = await response.text();
        const executionTime = performance.now() - startTime;
        postMessage({output: responseText, isError: !response.ok, wasm, executionTime});
      }
    } catch (error) {
      console.error(error);
      const executionTime = performance.now() - startTime;
      postMessage({output: error.message, isError: true, wasm, executionTime});
    }
  }
}

run();
