use std::collections::hash_map::Entry;
use std::collections::HashMap;

fn solution(input_string: &str, part1: bool) -> Result<u32, String> {
    const MAX_ITERATIONS: u32 = 1_000_000;

    let mut memory_banks: Vec<u32> = input_string
        .split_ascii_whitespace()
        .enumerate()
        .map(|(index, word)| {
            word.parse::<u32>().map_err(|error| {
                format!("Invalid input at word {}: {}", index + 1, error.to_string())
            })
        })
        .collect::<Result<_, _>>()?;

    if memory_banks.is_empty() {
        return Err("Invalid empty input".to_string());
    }

    let mut seen_before = HashMap::new();

    for current_step in 0..MAX_ITERATIONS {
        match seen_before.entry(memory_banks.clone()) {
            Entry::Occupied(value) => {
                return Ok(current_step - if part1 { 0_u32 } else { *value.get() });
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

pub fn part1(input_string: &str) -> Result<u32, String> {
    solution(input_string, true)
}

pub fn part2(input_string: &str) -> Result<u32, String> {
    solution(input_string, false)
}

#[test]
fn test_part1() {
    assert_eq!(Ok(12841), part1(include_str!("day06_input.txt")));
    assert_eq!(
        Err("Invalid input at word 3: invalid digit found in string".to_string()),
        part1("12 12 hi")
    );
}

#[test]
fn test_part2() {
    assert_eq!(Ok(8038), part2(include_str!("day06_input.txt")));
}
