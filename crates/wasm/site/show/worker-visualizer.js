import init, { solve } from "./generated/advent_of_code_wasm.js";

self.onmessage = async (message) => {
  try {
    const { year, day, part, input } = message.data;
    await self.wasmReadyPromise;
    const answer = solve(year, day, part, input);
    self.postMessage({ done: true, answer });
  } catch (e) {
    console.log(e);
    self.postMessage({ errorMessage: e.message });
  }
};

self.wasmReadyPromise = (async () => {
  try {
    return await init();
  } catch (e) {
    throw new Error("WebAssembly not working - " + e.message);
  }
})();

(async () => {
  await self.wasmReadyPromise;
})();
