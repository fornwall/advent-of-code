const isDeno = typeof Deno === 'object';

function exitProcess(message) {
	console.log(message);
	if (isDeno) {
		Deno.exit(1);
	} else {
		process.exit(1);
	}
}

function getArgv() {
	if (isDeno) {
		const args = Deno.args.slice();
		args.unshift('deno');
		return args;
	} else {
		return process.argv;
	}
}

function readStdin() {
	if (isDeno) {
		return Deno.readAllSync(Deno.stdin);
	} else {
		const fs = require('fs');
		return fs.readFileSync(0, 'utf8');
	}
}

const argv = getArgv();
const day = parseInt(argv[2]);
const part = parseInt(argv[3]);
if (!(day >= 1 && day <= 25)) {
    exitProcess('Invalid day - must be integer between 1 and 25');
} else if (!(part >= 1 && part <= 2)) {
    exitProcess('Invalid part - must be 1 or 2');
}

const input = readStdin();

function solve(wasmCodeBuffer, day, part, inputBuffer) {
    const wasmModule = new WebAssembly.Module(wasmCodeBuffer);
    const wasmInstance = new WebAssembly.Instance(wasmModule);
    const wasm = wasmInstance.exports;

    const outputPointer = 8;
    const inputLength = inputBuffer.length;
    const inputPointer = wasm.__wbindgen_malloc(inputLength);
    if (isDeno) {
        // TODO?
        new Deno.Buffer(wasm.memory.buffer).write(inputBuffer, inputPointer, inputLength);
    } else {
        Buffer.from(wasm.memory.buffer).write(inputBuffer, inputPointer, inputLength);
    }
    wasm.solve(outputPointer, day, part, inputPointer, inputLength);

    const memi32 = new Int32Array(wasm.memory.buffer);
    const ptr = memi32[outputPointer / 4 + 0];
    const len = memi32[outputPointer / 4 + 1];
    const outputString = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true })
        .decode(new Uint8Array(wasm.memory.buffer).subarray(ptr, ptr + len))
        .slice();
    // wasm.__wbindgen_free(memi32[outputPointer / 4 + 0], memi32[outputPointer / 4 + 1] * 1);

    console.log(outputString);
}

const hostname = 'fornwall.net';
const path = '/advent-of-code-2019/solution.wasm';

if (isDeno) {
    const wasmUrl = 'https://' + hostname + path;
    fetch(wasmUrl).then(response =>
        response.arrayBuffer()
    ).then(wasmCode =>
        solve(wasmCode, day, part, input)
    );
} else {
    const https = require('https')
    const req = https.request({
        hostname,
        port: 443,
        path,
        method: 'GET'
    }, res => {
        const data = [];
        res.on('data', chunk => {
            data.push(chunk);
        }).on('end', function() {
            const wasmCode = Buffer.concat(data);
            solve(wasmCode, day, part, input);
        });
    });
    req.end();
}
