[![npm](https://img.shields.io/npm/v/advent-of-code-wasm.svg)](https://www.npmjs.com/package/advent-of-code-wasm)

# advent-of-code-wasm
Solutions to [Advent of Code](https://adventofcode.com/) implemented in Rust and compiled to WebAssembly.

## Running it in your browser
Use https://aoc.fornwall.net to run the solutions in your browser.

## Usage as a Node.js library
Add dependency:

```sh
npm add advent-of-code-wasm
```

The `advent-of-code-wasm` package exports a single `solve` function with the following signature:

```js
function solve(year, day, part, input)
```

Examples:

```js
const assert = require('assert/strict');
const solve = require('advent-of-code-wasm').solve;
  
assert.equal(solve(2019, 1, 1, '14'), '2');
assert.equal(solve(2019, 3, 2, "R8,U5,L5,D3\nU7,R6,D4,L4"), '30');
```

## Usage as a command line tool

```sh
$ npm install -g advent-of-code-wasm
$ echo 14 | advent-of-code-wasm 2019 1 1
2
```

## Deployment to CloudFlare workers
The [worker/](https://github.com/fornwall/advent-of-code/tree/master/crates/wasm/worker) directory contains code and configuration to deploy the WebAssembly with a JS wrapper to [Cloudflare Workers](https://workers.cloudflare.com/).

- Deployment URL: https://aoc.fornwall.workers.dev

It implements the API described at https://aoc.fornwall.net/openapi.json, and that URL can be selected from a [Swagger UI](https://aoc.fornwall.net/api/). Example usage:

```sh
$ curl -d 14 https://aoc.fornwall.workers.dev/solve/2019/1/1
2
```

## Deployment to Netlify Functions
The [functions/](https://github.com/fornwall/advent-of-code/tree/master/crates/wasm/functions) directory contains code and configuration to deploy the WebAssembly with a JS wrapper to [Netlify Functions](https://www.netlify.com/products/functions/).

- Deployment URL: https://aoc.fornwall.workers.dev

It implements the API described at https://aoc.fornwall.net/openapi.json, and that URL can be selected from a [Swagger UI](https://aoc.fornwall.net/api/). Example usage:

```sh
$ curl -d 14 https://aoc.fornwall.net/solve/2019/1/1
2
```

