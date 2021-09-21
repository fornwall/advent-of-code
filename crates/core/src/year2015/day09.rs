use crate::common::permutation::all_permutations;
use crate::Input;
use std::collections::{HashMap, HashSet};

pub fn solve(input: &mut Input) -> Result<u32, String> {
    let mut places = HashSet::new();
    let mut distances = HashMap::new();
    for line in input.text.lines() {
        // "Faerun to Tristram = 58"
        let parts = line.split(' ').collect::<Vec<_>>();
        if parts.len() != 5 {
            return Err("Invalid input - line not having 5 words".to_string());
        }
        let from = parts[0];
        let to = parts[2];
        let distance = parts[4].parse::<u32>().map_err(|_| "Invalid input")?;

        places.insert(from);
        places.insert(to);
        distances.insert((from, to), distance);
        distances.insert((to, from), distance);
    }

    let mut best_distance = input.part_values(u32::MAX, u32::MIN);
    let mut places = places.into_iter().collect::<Vec<_>>();

    all_permutations(&mut places, &mut |ordering| {
        let mut this_distance = 0;
        for pair in ordering.windows(2) {
            this_distance += distances.get(&(pair[0], pair[1])).ok_or_else(|| {
                "Distances between every pair of locations not specified".to_string()
            })?
        }
        best_distance = if input.is_part_one() {
            std::cmp::min(best_distance, this_distance)
        } else {
            std::cmp::max(best_distance, this_distance)
        };
        Ok(())
    })?;

    Ok(best_distance)
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    let example_input = "London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141";
    test_part_one!(example_input => 605);
    test_part_two!(example_input => 982);

    let real_input = include_str!("day09_input.txt");
    test_part_one!(real_input => 207);
    test_part_two!(real_input => 804);
}
