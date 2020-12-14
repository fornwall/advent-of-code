use crate::input::Input;
use std::collections::HashMap;

type Memory = HashMap<u64, u64>;

trait BitMask {
    fn new() -> Self;
    fn parse(&mut self, input: &str);
    fn apply(&self, memory: &mut Memory, address: u64, value: u64) -> Result<(), String>;
    fn too_slow(&self) -> bool {
        false
    }
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

    fn apply(&self, memory: &mut Memory, address: u64, value: u64) -> Result<(), String> {
        let new_value = (value & self.zeroes) | self.ones;
        memory.insert(address, new_value);
        Ok(())
    }
}

struct BitMaskV2 {
    /// Bitmask with bits being 1 if they should always be set.
    /// Example: "10XX110" causes `ones` to be 0b1000110.
    ones: u64,
    /// Bitmask with bits being 0 if they are undecided.
    /// Example: "10XX110" causes `floating_bitmask` to be 0b1100111.
    floating_bitmask: u64,
}

impl BitMask for BitMaskV2 {
    fn new() -> Self {
        Self {
            ones: 0,
            floating_bitmask: 0,
        }
    }

    fn parse(&mut self, input: &str) {
        self.floating_bitmask = u64::MAX;
        self.ones = 0;
        for (offset, c) in input.bytes().rev().enumerate() {
            match c {
                b'1' => self.ones |= 1 << offset,
                b'X' => {
                    self.floating_bitmask &= !(1 << offset);
                }
                _ => {}
            }
        }
    }

    fn apply(&self, memory: &mut Memory, address: u64, value: u64) -> Result<(), String> {
        const MEMORY_LIMIT: usize = 100_000;
        const ALL_36_BITS_SET: u64 = 0b1111_1111_1111_1111_1111_1111_1111_1111_1111;

        if memory.len() >= MEMORY_LIMIT {
            return Err(format!(
                "Aborting due to memory usage (refusing to go above {} stored addresses)",
                MEMORY_LIMIT
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
            memory.insert(floating_address, value);

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
            // "overflow" when lower set bits are carried right by addition.
            //
            // Thanks to svetlin_zarev on reddit who came up with and shared this!
            floating_counter += 1;
            floating_counter |= self.floating_bitmask;
        }
        Ok(())
    }

    fn too_slow(&self) -> bool {
        self.floating_bitmask.count_zeros() >= 10
    }
}

fn solve_with_bit_mask<T: BitMask>(
    input_string: &str,
    initial_capacity: usize,
) -> Result<u64, String> {
    let mut bit_mask = T::new();
    let mut memory = Memory::with_capacity(initial_capacity);

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
            if bit_mask.too_slow() {
                return Err(format!("Line {}: Bit mask would be too slow", line_idx + 1));
            }
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
            bit_mask.apply(&mut memory, address, value)?;
        } else {
            return Err(on_error());
        }
    }

    Ok(memory.values().sum())
}

pub fn solve(input: &mut Input) -> Result<u64, String> {
    if input.is_part_one() {
        solve_with_bit_mask::<BitMaskV1>(input.text, 1000)
    } else {
        solve_with_bit_mask::<BitMaskV2>(input.text, 100_000)
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
