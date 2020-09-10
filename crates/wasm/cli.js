#!/usr/bin/env node

const fs = require('fs');
const solve = require('advent-of-code-wasm').solve;

const year = parseInt(process.argv[2]);
const day = parseInt(process.argv[3]);
const part = parseInt(process.argv[4]);

if (!(year >= 2018 && year <= 2019)) {
    console.error('Invalid year - must be integer between 2018 and 2019');
    process.exit(1);
} else if (!(day >= 1 && day <= 25)) {
    console.error('Invalid day - must be integer between 1 and 25');
    process.exit(1);
} else if (!(part >= 1 && part <= 2)) {
    console.error('Invalid part - must be 1 or 2');
    process.exit(1);
}

const input = fs.readFileSync(0, 'utf8');

try {
    const output = solve(day, part, input);
    console.log(output);
} catch (e) {
    console.error("ERROR: Invalid input");
}

