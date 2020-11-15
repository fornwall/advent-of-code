use crate::year2017::day10::part2 as knot_hash;

pub fn part1(input_string: &str) -> Result<u32, String> {
    let mut used_count = 0;
    for row in 0..=127 {
        let hash_input = format!("{}-{}", input_string, row);
        let hash = knot_hash(&hash_input)?;
        used_count += hash
            .chars()
            .map(|b| {
                u32::from_str_radix(&b.to_string(), 16)
                    .expect("ok")
                    .count_ones()
            })
            .sum::<u32>();
        println!("{} -> {}", hash_input, hash);
    }
    Ok(used_count)
}

pub fn part2(_input_string: &str) -> Result<u32, String> {
    Err("Not yet implemented".to_string())
}

#[test]
fn test_part1() {
    assert_eq!(Ok(8108), part1("flqrgnkx"));
    assert_eq!(Ok(8222), part1(include_str!("day14_input.txt")));
}

#[test]
fn test_part2() {
    assert_eq!(Ok(1220), part2(include_str!("day01_input.txt")));
}
