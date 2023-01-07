use crate::input::Input;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

pub fn solve(input: &Input) -> Result<u32, String> {
    const MAX_ITERATIONS: u32 = 1_000_000;

    let mut memory_banks: Vec<u32> = input
        .text
        .split_ascii_whitespace()
        .enumerate()
        .map(|(index, word)| {
            word.parse::<u32>()
                .map_err(|error| format!("Invalid input at word {}: {}", index + 1, error))
        })
        .collect::<Result<_, _>>()?;

    if memory_banks.is_empty() {
        return Err("Invalid empty input".to_string());
    }

    let mut seen_before = HashMap::new();

    for current_step in 0..MAX_ITERATIONS {
        match seen_before.entry(memory_banks.clone()) {
            Entry::Occupied(value) => {
                return Ok(current_step - input.part_values(0_u32, *value.get()));
            }
            Entry::Vacant(entry) => {
                entry.insert(current_step);
            }
        }

        let bank_to_redistribute =
            memory_banks
                .iter()
                .enumerate()
                .fold(0, |acc, (index, &blocks)| {
                    if blocks > memory_banks[acc] {
                        index
                    } else {
                        acc
                    }
                });

        let mut blocks_to_distribute = memory_banks[bank_to_redistribute];
        memory_banks[bank_to_redistribute] = 0;
        let mut current_index = bank_to_redistribute;
        while blocks_to_distribute > 0 {
            current_index = (current_index + 1) % memory_banks.len();
            memory_banks[current_index] += 1;
            blocks_to_distribute -= 1;
        }
    }

    Err(format!("Aborting after {} iterations", MAX_ITERATIONS))
}

#[test]
fn tests() {
    use crate::input::{test_part_one, test_part_one_error, test_part_two};
    let real_input = include_str!("day06_input.txt");
    test_part_one!(real_input => 12841);
    test_part_two!(real_input => 8038);
    test_part_one_error!(
        "12 12 hi" =>
        "Invalid input at word 3: invalid digit found in string".to_string()
    );
}
