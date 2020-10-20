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

  self.onmessage = async (e) => {
    const { year, day, part, input, wasm } = e.data;
    if (wasm) {
      if (!wasmWorking) {
        postMessage({output: "WASM is not working", isError: true});
        return;
      }
      try {
        const output = wasm_bindgen.solve(year, day, part, input);
        postMessage({output, isError: false});
      } catch (e) {
        console.log(e);
        postMessage({output: e.message, isError: true, wasm});
      }
    } else {
      const response = await fetch(`https://aoc.fly.dev/solve/${year}/${day}/${part}/`, {
        method: "POST",
        headers: { "content-type": "text/plain" },
        body: input
      });
      const responseText = await response.text();
      postMessage({output: responseText, isError: !response.ok, wasm});
    }
  }
}

run();
