use crate::input::Input;
use std::collections::{HashMap, HashSet};

pub fn solve(input: &Input) -> Result<u32, String> {
    let mut mappings = HashMap::new();
    let mut molecule = String::new();

    for line in input.text.lines() {
        if line.is_empty() {
            // Blank line before last.
        } else if let Some((part1, part2)) = line.split_once(" => ") {
            mappings.entry(part1).or_insert_with(Vec::new).push(part2);
        } else {
            molecule = line.to_string();
        }
    }

    if input.is_part_one() {
        let mut distinct_molecules = HashSet::new();
        for (&key, values) in mappings.iter() {
            for (start_idx, _) in molecule.match_indices(key) {
                for &value in values.iter() {
                    let new_molecule = format!(
                        "{}{}{}",
                        &molecule[..start_idx],
                        value,
                        &molecule[start_idx + key.len()..]
                    );
                    distinct_molecules.insert(new_molecule);
                }
            }
        }
        Ok(distinct_molecules.len() as u32)
    } else {
        let mut count = 0;
        let mut reversed_molecule = molecule.chars().rev().collect::<String>();
        let reversed_mappings: Vec<(String, String)> = mappings
            .into_iter()
            .flat_map(|(key, values)| {
                let reversed_key: String = key.chars().rev().collect();
                values.into_iter().map(move |value| {
                    let reversed_value = value.chars().rev().collect::<String>();
                    (reversed_value, reversed_key.clone())
                })
            })
            .collect();

        while reversed_molecule.len() != 1 {
            if let Some((idx, key, value)) = reversed_mappings
                .iter()
                .filter_map(|(key, value)| reversed_molecule.find(key).map(|idx| (idx, key, value)))
                .min()
            {
                reversed_molecule = format!(
                    "{}{}{}",
                    &reversed_molecule[..idx],
                    value,
                    &reversed_molecule[(idx + key.len())..]
                );
                count += 1;
            } else {
                return Err(format!("Stuck after {} steps", count));
            }
        }

        Ok(count)
    }
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    let real_input = include_str!("day19_input.txt");
    test_part_one!(real_input => 509);
    test_part_two!(real_input => 195);
}
