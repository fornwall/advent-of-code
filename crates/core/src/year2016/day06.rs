use crate::input::Input;
use std::collections::HashMap;

pub fn solve(input: &Input) -> Result<String, String> {
    let mut counts: [HashMap<u8, u32>; 8] = [
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
    ];

    for line in input.text.lines() {
        for (index, c) in line.bytes().take(counts.len()).enumerate() {
            let count: &mut HashMap<u8, u32> = &mut counts[index];
            *count.entry(c).or_insert(0) += 1;
        }
    }

    Ok(counts
        .iter()
        .map(|count| {
            count
                .iter()
                .max_by(|a, b| {
                    if input.is_part_one() {
                        a.1.cmp(b.1)
                    } else {
                        b.1.cmp(a.1)
                    }
                })
                .map_or('?', |(&key, _value)| key as char)
        })
        .collect())
}

#[test]
pub fn tests() {
    let real_input = include_str!("day06_input.txt");
    test_part_one!(real_input => "qzedlxso".to_string());
    test_part_two!(real_input => "ucmifjae".to_string());
}
