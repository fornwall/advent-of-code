const fs = require('fs');
const https = require('https')

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

function solve(wasmCodeBuffer, day, part, input_buffer) {
    const wasmModule = new WebAssembly.Module(wasmCodeBuffer);
    const wasmInstance = new WebAssembly.Instance(wasmModule);
    const wasm = wasmInstance.exports;

    const retptr = 8;
      const inputLength = Buffer.byteLength(input);
      const inputPointer = wasm.__wbindgen_malloc(inputLength);
      Buffer.from(wasm.memory.buffer).write(input, inputPointer, inputLength);
      const ret = wasm.solve(retptr, day, part, inputPointer, inputLength);

      const memi32 = new Int32Array(wasm.memory.buffer);
      const ptr = memi32[retptr / 4 + 0];
      const len = memi32[retptr / 4 + 1];
      const v0 = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true })
        .decode(new Uint8Array(wasm.memory.buffer).subarray(ptr, ptr + len))
        .slice();
      wasm.__wbindgen_free(memi32[retptr / 4 + 0], memi32[retptr / 4 + 1] * 1);

      console.log(v0);
}


const req = https.request({
  hostname: 'fornwall.net',
  port: 443,
  path: '/advent-of-code-2019/571fb931d982e3ea292d.module.wasm',
  method: 'GET'
}, res => {
  const data = [];
  res.on('data', chunk => {
    data.push(chunk);
  }).on('end', function() {
    const wasmCode = Buffer.concat(data);
    solve(wasmCode, day, part, input);
  });
})

req.end();
