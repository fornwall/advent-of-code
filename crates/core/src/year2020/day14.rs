use crate::input::Input;
use std::collections::HashMap;

trait BitMask {
    fn new() -> Self;
    fn parse(&mut self, input: &str);
    fn apply(&self, memory: &mut HashMap<u64, u64>, address: u64, value: u64);
}

struct BitMaskV1 {
    zeroes: u64,
    ones: u64,
}

impl BitMask for BitMaskV1 {
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

    fn apply(&self, memory: &mut HashMap<u64, u64>, address: u64, value: u64) {
        let new_value = (value & self.zeroes) | self.ones;
        memory.insert(address, new_value);
    }
}

struct BitMaskV2 {
    ones: u64,
    floating_bitmask: u64,
    floating_offsets: Vec<u8>,
}

impl BitMask for BitMaskV2 {
    fn new() -> Self {
        Self {
            ones: 0,
            floating_bitmask: 0,
            floating_offsets: Vec::with_capacity(36),
        }
    }

    fn parse(&mut self, input: &str) {
        self.floating_offsets.clear();
        self.floating_bitmask = u64::MAX;
        self.ones = 0;
        for (offset, c) in input.bytes().rev().enumerate() {
            match c {
                b'1' => self.ones |= 1 << offset,
                b'X' => {
                    self.floating_bitmask &= !(1 << offset);
                    self.floating_offsets.push(offset as u8);
                }
                _ => {}
            }
        }
    }

    fn apply(&self, memory: &mut HashMap<u64, u64>, address: u64, value: u64) {
        let new_address = (address | self.ones) & self.floating_bitmask;
        Self::apply_helper(memory, new_address, value, &self.floating_offsets);
    }
}

impl BitMaskV2 {
    fn apply_helper(
        memory: &mut HashMap<u64, u64>,
        address: u64,
        value: u64,
        remaining_floats: &[u8],
    ) {
        if remaining_floats.is_empty() {
            memory.insert(address, value);
        } else {
            Self::apply_helper(memory, address, value, &remaining_floats[1..]);

            let float_mask = 1 << remaining_floats[0];
            let address_with_float_set = address | float_mask;
            Self::apply_helper(
                memory,
                address_with_float_set,
                value,
                &remaining_floats[1..],
            );
        }
    }
}

fn solve_with_bit_mask<T: BitMask>(input_string: &str) -> Result<u64, String> {
    let mut bit_mask = T::new();
    let mut memory = HashMap::new();

    for (line_idx, line) in input_string.lines().enumerate() {
        let on_error = || format!("Line {}: Invalid format", line_idx + 1);

        if let Some(bit_mask_str) = line.strip_prefix("mask = ") {
            if bit_mask_str.len() != 36
                || bit_mask_str
                    .bytes()
                    .any(|c| !matches!(c, b'X' | b'1' | b'0'))
            {
                return Err(on_error());
            }
            bit_mask.parse(bit_mask_str);
        } else if let Some(remainder) = line.strip_prefix("mem[") {
            let mut parts = remainder.split("] = ");
            let address = parts
                .next()
                .ok_or_else(on_error)?
                .parse::<u64>()
                .map_err(|_| on_error())?;
            let value = parts
                .next()
                .ok_or_else(on_error)?
                .parse::<u64>()
                .map_err(|_| on_error())?;
            bit_mask.apply(&mut memory, address, value);
        } else {
            return Err(on_error());
        }
    }

    Ok(memory.values().sum())
}

pub fn solve(input: &mut Input) -> Result<u64, String> {
    if input.is_part_one() {
        solve_with_bit_mask::<BitMaskV1>(input.text)
    } else {
        solve_with_bit_mask::<BitMaskV2>(input.text)
    }
}

#[test]
pub fn tests() {
    use crate::{test_part_one, test_part_two};

    test_part_one!("mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0" => 165);

    test_part_two!("mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1" => 208);

    let real_input = include_str!("day14_input.txt");
    test_part_one!(real_input => 9_628_746_976_360);
    test_part_two!(real_input => 4_574_598_714_592);
}
