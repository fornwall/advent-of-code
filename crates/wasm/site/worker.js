self.importScripts("/advent_of_code_wasm.js");

async function run() {
    try {
	    await wasm_bindgen("/advent_of_code_wasm_bg.wasm");

        self.onmessage = (e) => {
            const { year, day, part, input } = e.data;
            try {
                const output = wasm_bindgen.solve(year, day, part, input);
                postMessage({output, isError: false});
            } catch (e) {
                console.log(e);
                postMessage({output: e.message, isError: true});
            }
        };
    } catch (e) {
        console.warn('WebAssembly not working - switching to HTTP API fallback', e);
        self.onmessage = async (e) => {
            const { year, day, part, input } = e.data;
            const response = await fetch(`https://aoc.fly.dev/solve/${year}/${day}/${part}/`, {
                method: "POST",
                headers: { "content-type": "text/plain" },
                body: input
            });
            const responseText = await response.text();
            postMessage({output: responseText, isError: !response.ok});
        };
    }
}

run();
