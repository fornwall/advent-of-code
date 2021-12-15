import init, { solve } from "./generated/advent_of_code_wasm.js";

class WasmNotWorkingError extends Error {
  constructor(message) {
    super(message);
    this.name = "WasmNotWorkingError";
  }
}

async function solveWasm(year, day, part, input) {
  await self.wasmReadyPromise;

  if (!self.wasmWorking) {
    throw new WasmNotWorkingError();
  }

  const startTime = performance.now();
  try {
    const answer = solve(year, day, part, input);
    const executionTime = performance.now() - startTime;
    console.log(
      `Wasm ${year}-${day}-${part} solution in: ${executionTime.toFixed(2)} ms`
    );
    return { answer, executionTime };
  } catch (error) {
    console.error(error);
    const executionTime = performance.now() - startTime;
    return {
      answer: error.message,
      isError: true,
      executionTime,
    };
  }
}

self.onmessage = async (message) => {
  const { year, day, part, input } = message.data;

  try {
    const response = await solveWasm(year, day, part, input);
    postMessage({
      worker: "wasm",
      year,
      day,
      part,
      input,
      output: response.answer,
      isError: response.isError,
      executionTime: response.executionTime,
    });
  } catch (e) {
    const message = "• WebAssembly: " + e;
    postMessage({
      worker: "wasm",
      year,
      day,
      part,
      input,
      output: message,
      isError: true,
      isInternalError: true,
      executionTime: 0,
    });
  }
};

self.wasmReadyPromise = (async () => {
  try {
    await init();
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
