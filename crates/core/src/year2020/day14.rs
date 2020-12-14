use crate::input::Input;
use std::collections::HashMap;

#[derive(Copy, Clone)]
struct BitMask {
    zeroes: u64,
    ones: u64,
}

impl BitMask {
    fn new() -> Self {
        Self { zeroes: 0, ones: 0 }
    }

    fn parse(&mut self, input: &str) {
        self.zeroes = u64::MAX;
        self.ones = 0;
        for (offset, c) in input.bytes().rev().enumerate() {
            match c {
                b'1' => self.ones |= 1 << offset,
                b'0' => self.zeroes &= !(1 << offset),
                _ => {}
            }
        }
    }

    fn apply(&self, value: u64) -> u64 {
        (value & self.zeroes) | self.ones
    }
}

pub fn solve(input: &mut Input) -> Result<u64, String> {
    let mut bit_mask = BitMask::new();
    let mut memory = HashMap::new();

    for (line_idx, line) in input.text.lines().enumerate() {
        let on_error = || format!("Line {}: Invalid format", line_idx + 1);

        if let Some(bit_mask_str) = line.strip_prefix("mask = ") {
            bit_mask.parse(bit_mask_str);
        } else if let Some(remainder) = line.strip_prefix("mem[") {
            let mut parts = remainder.split("] = ");
            let address = parts.next().unwrap().parse::<u64>().unwrap();
            let value = parts.next().unwrap().parse::<u64>().unwrap();
            memory.insert(address, bit_mask.apply(value));
        } else {
            return Err(on_error());
        }
    }

    Ok(memory.values().sum())
}

#[test]
pub fn tests() {
    use crate::{test_part_one, test_part_two};

    let example = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";
    test_part_one!(example => 165);
    // test_part_two!(example => 1_068_781);

    let real_input = include_str!("day14_input.txt");
    test_part_one!(real_input => 9628746976360);
    // test_part_two!(real_input => 825_305_207_525_452);
}
