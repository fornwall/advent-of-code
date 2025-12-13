use crate::input::Input;
use std::collections::HashSet;
use std::hash::{BuildHasherDefault, Hasher};

#[derive(Default)]
struct CustomHash {
    hash: u64,
}

impl Hasher for CustomHash {
    fn finish(&self) -> u64 {
        self.hash
    }

    fn write(&mut self, _: &[u8]) {}

    fn write_u64(&mut self, value: u64) {
        // From Fx Hash.
        self.hash = value.wrapping_mul(0x51_7c_c1_b7_27_22_0a_95);
    }
}

type CustomBuildHasher = BuildHasherDefault<CustomHash>;

type Memory = HashSet<u64, CustomBuildHasher>;

trait BitMask {
    fn parse(input: &str) -> Self;
    fn apply(&self, memory: &mut Memory, address: u64, value: u64) -> Result<u64, String>;
}

#[derive(Copy, Clone)]
struct BitMaskV1 {
    zeroes: u64,
    ones: u64,
}

impl BitMask for BitMaskV1 {
    fn parse(input: &str) -> Self {
        let mut zeroes = u64::MAX;
        let mut ones = 0;
        for (offset, c) in input.bytes().rev().enumerate() {
            match c {
                b'1' => ones |= 1 << offset,
                b'0' => zeroes &= !(1 << offset),
                _ => {}
            }
        }
        Self { zeroes, ones }
    }

    fn apply(&self, memory: &mut Memory, address: u64, value: u64) -> Result<u64, String> {
        Ok(if memory.insert(address) {
            (value & self.zeroes) | self.ones
        } else {
            0
        })
    }
}

#[derive(Copy, Clone)]
struct BitMaskV2 {
    /// Bitmask with bits being 1 if they should always be set.
    /// Example: "10XX110" causes `ones` to be 0b1000110.
    ones: u64,
    /// Bitmask with bits being 0 if they are undecided.
    /// Example: "10XX110" causes `floating_bitmask` to be 0b1100111.
    floating_bitmask: u64,
}

impl BitMask for BitMaskV2 {
    fn parse(input: &str) -> Self {
        let mut floating_bitmask = u64::MAX;
        let mut ones = 0;
        for (offset, c) in input.bytes().rev().enumerate() {
            match c {
                b'1' => ones |= 1 << offset,
                b'X' => {
                    floating_bitmask &= !(1 << offset);
                }
                _ => {}
            }
        }
        Self {
            ones,
            floating_bitmask,
        }
    }

    fn apply(&self, memory: &mut Memory, address: u64, value: u64) -> Result<u64, String> {
        const MEMORY_LIMIT: usize = 100_000;
        const ALL_36_BITS_SET: u64 = 0b1111_1111_1111_1111_1111_1111_1111_1111_1111;

        let mut sum = 0;

        if memory.len() >= MEMORY_LIMIT {
            return Err(format!(
                "Aborting due to memory usage (refusing to go above {MEMORY_LIMIT} stored addresses)"
            ));
        }

        // We start with a base address with the ones from the bitmask set
        // and the undecided bits cleared.
        let base_address = (address | self.ones) & self.floating_bitmask;

        // We want to iterate over all possible combinations of undecided bits.
        // Start a counter with undecided bits set to 0 and other bits set to 1.
        let mut floating_counter = self.floating_bitmask;
        loop {
            // Invert the counter to get the bits to set in this iteration:
            let floating_address = base_address | !floating_counter;
            if memory.insert(floating_address) {
                sum += value;
            }

            // If we have all 36 bits set there are no undecided bits left
            // to iterate over and we are done.
            if floating_counter & ALL_36_BITS_SET == ALL_36_BITS_SET {
                break;
            }

            // Increase the counter by 1 to toggle of bits we no longer want to set.
            //
            //  Iteration 1: 0000
            //  Iteration 2: 0001
            //  Iteration 3: 0010
            //  Finally:     1111
            //
            // But, the undecided bits are spread out. If the floating_bitmask is:
            //  floating_bitmask: 0110
            // the first addition will be as desired:
            //  floating_counter: 0111
            // But the next one will result in:
            //  floating_counter: 1000
            // By OR:ing wih the original floating_bitmask we get the desired value:
            //  floating_counter: 1111
            // And that works in general, by bringing back lower bits cleared by the
            // "overflow" when lower set bits are carried left by addition.
            //
            // Thanks to svetlin_zarev on reddit who came up with and shared this!
            floating_counter += 1;
            floating_counter |= self.floating_bitmask;
        }
        Ok(sum)
    }
}

enum Command {
    PopBitMask,
    Set(u64, u64),
}

fn solve_with_bit_mask<T: BitMask + Copy + Clone>(
    input_string: &str,
    initial_capacity: usize,
) -> Result<u64, String> {
    let mut commands = Vec::with_capacity(600);
    let mut bit_mask_stack = Vec::with_capacity(100);

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
            commands.push(Command::PopBitMask);
            bit_mask_stack.push(T::parse(bit_mask_str));
        } else if let Some(remainder) = line.strip_prefix("mem[") {
            let (part1, part2) = remainder.split_once("] = ").ok_or_else(on_error)?;
            let address = part1.parse::<u64>().map_err(|_| on_error())?;
            let value = part2.parse::<u64>().map_err(|_| on_error())?;
            commands.push(Command::Set(address, value));
        } else {
            return Err(on_error());
        }
    }

    let mut current_bit_mask = bit_mask_stack
        .pop()
        .ok_or_else(|| "Internal error".to_string())?;
    let mut memory =
        Memory::with_capacity_and_hasher(initial_capacity, CustomBuildHasher::default());
    let mut sum = 0;
    for command in commands.iter().skip(1).rev() {
        match command {
            Command::PopBitMask => {
                current_bit_mask = bit_mask_stack
                    .pop()
                    .ok_or_else(|| "Internal error".to_string())?;
            }
            &Command::Set(address, value) => {
                sum += current_bit_mask.apply(&mut memory, address, value)?;
            }
        }
    }

    Ok(sum)
}

pub fn solve(input: &Input) -> Result<u64, String> {
    if input.is_part_one() {
        solve_with_bit_mask::<BitMaskV1>(input.text, 1000)
    } else {
        solve_with_bit_mask::<BitMaskV2>(input.text, 100_000)
    }
}

#[test]
pub fn tests() {
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
