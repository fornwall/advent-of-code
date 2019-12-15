const https = require('https')
const fs = require('fs');

const encoder = new TextEncoder()

const day = parseInt(process.argv[2]);
const part = parseInt(process.argv[3]);
if (!(day >= 1 && day <= 25)) {
    console.log('Invalid day - must be integer between 1 and 25');
    process.exit(1);
} else if (!(part >= 1 && part <= 2)) {
    console.log('Invalid part - must be 1 or 2');
    process.exit(1);
}

const input = fs.readFileSync(0, 'utf8');

const options = {
  hostname: 'fornwall.net',
  port: 443,
  path: '/advent-of-code-2019/571fb931d982e3ea292d.module.wasm',
  method: 'GET'
}

function getStringFromWasm(wasm, ptr, len) {
    return new TextDecoder('utf-8', { ignoreBOM: true, fatal: true })
        .decode(new Uint8Array(wasm.memory.buffer).subarray(ptr, ptr + len));
}
let cachegetNodeBufferMemory = null;
function getNodeBufferMemory(wasm) {
    if (cachegetNodeBufferMemory === null || cachegetNodeBufferMemory.buffer !== wasm.memory.buffer) {
        cachegetNodeBufferMemory = Buffer.from(wasm.memory.buffer);
    }
    return cachegetNodeBufferMemory;
}
let WASM_VECTOR_LEN = 0;
function passStringToWasm(wasm, arg) {
    const len = Buffer.byteLength(arg);
    const ptr = wasm.__wbindgen_malloc(len);
    getNodeBufferMemory(wasm).write(arg, ptr, len);
    WASM_VECTOR_LEN = len;
    return ptr;
}

var data = [];

const req = https.request(options, res => {
  res.on('data', chunk => {
    data.push(chunk);
  }).on('end', function() {
    const wasmCode = Buffer.concat(data);
    const wasmModule = new WebAssembly.Module(wasmCode);
    const wasmInstance = new WebAssembly.Instance(wasmModule);
    try {
      const wasm = wasmInstance.exports;
      const retptr = 8;
      const ret = wasm.solve(retptr, day, part, passStringToWasm(wasm, input), WASM_VECTOR_LEN);

      const memi32 = new Int32Array(wasm.memory.buffer);
      const v0 = getStringFromWasm(wasm, memi32[retptr / 4 + 0], memi32[retptr / 4 + 1]).slice();
      wasm.__wbindgen_free(memi32[retptr / 4 + 0], memi32[retptr / 4 + 1] * 1);

      console.log(v0);
    } catch (e) {
		console.log(e);
    }
  });
})


req.on("error", (err) => {
  console.log("Request problem", err);
});

req.end();

