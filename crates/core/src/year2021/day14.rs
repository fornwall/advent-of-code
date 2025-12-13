use crate::input::Input;
use std::collections::HashMap;

pub fn solve(input: &Input) -> Result<u64, String> {
    let mut lines = input.text.lines();

    let template = lines.next().ok_or("No first line")?.as_bytes();
    if template.len() < 2 {
        return Err("No pairs in the template".to_string());
    }

    lines.next();

    let productions = lines
        .map(|line| {
            let bytes = line.as_bytes();
            if bytes.len() != 7 {
                return Err("Production not in format 'AB -> C'".to_string());
            }
            Ok(((bytes[0], bytes[1]), bytes[6]))
        })
        .collect::<Result<Vec<_>, _>>()?;

    let mut pair_occurrences = HashMap::new();
    for pair in template.windows(2) {
        *pair_occurrences.entry((pair[0], pair[1])).or_default() += 1;
    }

    for _step in 0..input.part_values(10, 40) {
        let mut additions = Vec::new();
        let mut removals = Vec::new();
        for &(producing_pair, inserted) in productions.iter() {
            if let Some(&count) = pair_occurrences.get(&producing_pair) {
                additions.push(((producing_pair.0, inserted), count));
                additions.push(((inserted, producing_pair.1), count));
                removals.push((producing_pair, count));
            }
        }
        for (added, count) in additions {
            *pair_occurrences.entry(added).or_default() += count;
        }
        for (removed, count) in removals {
            *pair_occurrences.entry(removed).or_default() -= count;
        }
    }

    let mut element_count = HashMap::new();
    for (key, count) in pair_occurrences.iter() {
        *element_count.entry(key.0).or_default() += count;
        *element_count.entry(key.1).or_default() += count;
    }

    // The above counts every element twice (as each element in a pair) is counted twice -
    // except the edge ones. So make the edge elements double counted as well:
    *element_count.entry(template[0]).or_default() += 1;
    *element_count
        .entry(template[template.len() - 1])
        .or_default() += 1;

    let most_common_count = element_count.values().max().unwrap_or(&0);
    let least_common_count = element_count.values().min().unwrap_or(&0);
    Ok((most_common_count - least_common_count) / 2)
}

#[test]
pub fn tests() {
    let example = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";
    test_part_one!(example => 1588);
    test_part_two!(example => 2_188_189_693_529);

    let real_input = include_str!("day14_input.txt");
    test_part_one!(real_input => 2975);
    test_part_two!(real_input => 3_015_383_850_689);
}
