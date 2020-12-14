#!/usr/bin/env node

const fs = require("fs");
const solve = require("advent-of-code-wasm").solve;

const year = parseInt(process.argv[2]);
const day = parseInt(process.argv[3]);
const part = parseInt(process.argv[4]);

const input = fs.readFileSync(0, "utf8");

try {
  const output = solve(year, day, part, input);
  console.log(output);
} catch (e) {
  console.error(e.message);
  process.exit(1);
}
