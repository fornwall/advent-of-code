type PolymerUnit = u8;

const fn destroys_each_other(a: PolymerUnit, b: PolymerUnit) -> bool {
    a.eq_ignore_ascii_case(&b) && a != b
}

fn solution(input: &str, part1: bool) -> Result<usize, String> {
    let input_polymer = input.as_bytes();
    let mut new_polymer = Vec::<PolymerUnit>::with_capacity(input_polymer.len());

    let candidates_for_removal = if part1 { 0..1 } else { b'a'..b'z' };

    candidates_for_removal
        .map(|to_remove_lower| {
            new_polymer.clear();

            for &unit in input_polymer
                .iter()
                .filter(|unit| !unit.eq_ignore_ascii_case(&to_remove_lower))
            {
                let unit_reacts_with_last = new_polymer
                    .last()
                    .map(|&last| destroys_each_other(unit, last))
                    .unwrap_or(false);

                if unit_reacts_with_last {
                    new_polymer.pop();
                } else {
                    new_polymer.push(unit);
                }
            }

            new_polymer.len()
        })
        .min()
        .ok_or_else(|| "Internal error".to_string())
}

pub fn part1(input: &str) -> Result<usize, String> {
    solution(input, true)
}

pub fn part2(input: &str) -> Result<usize, String> {
    solution(input, false)
}

#[test]
fn tests_part1() {
    assert_eq!(Ok(0), part1("aA"));
    assert_eq!(Ok(0), part1("abBA"));
    assert_eq!(Ok(4), part1("abAB"));
    assert_eq!(Ok(6), part1("aabAAB"));

    assert_eq!(Ok(11252), part1(include_str!("day05_input.txt")));
}

#[test]
fn tests_part2() {
    assert_eq!(Ok(4), part2("dabAcCaCBAcCcaDA"));

    assert_eq!(Ok(6118), part2(include_str!("day05_input.txt")));
}
