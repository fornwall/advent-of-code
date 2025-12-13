use crate::input::Input;

type PolymerUnit = u8;

const fn destroys_each_other(a: PolymerUnit, b: PolymerUnit) -> bool {
    a.eq_ignore_ascii_case(&b) && a != b
}

pub fn solve(input: &Input) -> Result<usize, String> {
    let input_polymer = input.text.as_bytes();
    let mut new_polymer = Vec::<PolymerUnit>::with_capacity(input_polymer.len());

    let candidates_for_removal = input.part_values(0..=0, b'a'..=b'z');

    candidates_for_removal
        .map(|to_remove_lower| {
            new_polymer.clear();

            for &unit in input_polymer
                .iter()
                .filter(|unit| !unit.eq_ignore_ascii_case(&to_remove_lower))
            {
                let unit_reacts_with_last = new_polymer
                    .last()
                    .is_some_and(|&last| destroys_each_other(unit, last));

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

#[test]
fn tests() {
    test_part_one!("aA"=>0);
    test_part_one!("abBA" => 0);
    test_part_one!("abAB" => 4);
    test_part_one!("aabAAB" => 6);

    test_part_two!("dabAcCaCBAcCcaDA" => 4);

    let input = include_str!("day05_input.txt");
    test_part_one!(input => 11252);
    test_part_two!(input => 6118);
}
