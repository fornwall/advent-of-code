#!/usr/bin/env node

const fs = require('fs');
const solve = require('advent_of_code_rs').solve;

const day = parseInt(process.argv[2]);
const part = parseInt(process.argv[3]);

if (!(day >= 1 && day <= 24)) {
    console.log('Invalid day - must be integer between 1 and 24');
    process.exit(1);
} else if (!(part >= 1 && part <= 2)) {
    console.log('Invalid part - must be 1 or 2');
    process.exit(1);
}

const input = fs.readFileSync(0, 'utf8');

console.log(solve(day, part, input));
