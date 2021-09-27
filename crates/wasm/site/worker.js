"use strict";
self.importScripts("/generated/advent_of_code_wasm.js");

async function solveApi(host, year, day, part, input) {
  const startTime = performance.now();
  const response = await fetch(`https://${host}/solve/${year}/${day}/${part}`, {
    method: "POST",
    headers: { "content-type": "text/plain" },
    body: input,
  });
  const responseText = await response.text();
  const executionTime = performance.now() - startTime;
  console.log(
    `API ${year}-${day}-${part} response from ${host}: ${executionTime} ms ${Math.random()}`
  );
  return {
    answer: responseText,
    isError: !response.ok,
    executionTime,
  };
}

async function solveWasm(year, day, part, input) {
  await self.wasmReadyPromise;

  if (!self.wasmWorking) {
    throw new Error("WebAssembly is not working");
  }

  const startTime = performance.now();
  try {
    const answer = wasm_bindgen.solve(year, day, part, input);
    const executionTime = performance.now() - startTime;
    console.log(`Wasm ${year}-${day}-${part} solution in: ${executionTime} ms`);
    return { answer, executionTime, wasm: true };
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

  const apiPromise1 = solveApi("advent.fly.dev", year, day, part, input);
  const apiPromise2 = solveApi(
    "aoc.fornwall.workers.dev",
    year,
    day,
    part,
    input
  );
  const apiPromise3 = solveApi("aoc.fornwall.net", year, day, part, input);
  const wasmPromise = solveWasm(year, day, part, input);
  Promise.any([apiPromise1, apiPromise2, apiPromise3, wasmPromise])
    .then((response) => {
      postMessage({
        output: response.answer,
        isError: response.isError,
        executionTime: response.executionTime,
      });
    })
    .catch((e) => {
      const message =
        "Unable to solve:\n\n" + e.errors.map((e) => `• ${e}`).join("\n");
      postMessage({
        output: message,
        isError: true,
        executionTime: 0,
      });
    });
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
