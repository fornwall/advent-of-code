# advent_of_code_rs_wasm
Solutions to [Advent of Code 2019](https://adventofcode.com/2019).

The solutions are implemented in Rust which is compiled to WebAssembly.

# Usage
Add dependency:

```sh
npm add advent_of_code_rs_wasm
```

The `advent_of_code_rs_wasm` package exports a single `solve` function with the following signature:

```js
function solve(day, part, input)
```

Examples:

```js
const solve = require('advent_of_code_rs_wasm').solve;

assert.equal(solve(1, 1, '14'), '2');
assert.equal(solve(3, 2, "R8,U5,L5,D3\nU7,R6,D4,L4"), '30');
```
