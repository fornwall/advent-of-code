use crate::input::Input;
use std::collections::HashMap;

pub fn solve(input: &mut Input) -> Result<String, String> {
    if input.is_part_one() {
        let picks = input.text.lines().fold((0, 0), |state, line| {
            let mut occurrences = HashMap::new();

            line.chars()
                .for_each(|c| *occurrences.entry(c).or_insert(0) += 1);

            let has_occurrence = |count| occurrences.iter().any(|(_key, &value)| value == count);

            (
                state.0 + i64::from(has_occurrence(2)),
                state.1 + i64::from(has_occurrence(3)),
            )
        });

        Ok((picks.0 * picks.1).to_string())
    } else {
        fn common_chars<'a>(s1: &'a str, s2: &'a str) -> impl Iterator<Item = char> + 'a {
            s1.chars()
                .zip(s2.chars())
                .filter_map(|(c1, c2)| if c1 == c2 { Some(c1) } else { None })
        }

        let input: Vec<&str> = input.text.lines().collect();

        for i in 0..input.len() {
            for j in i + 1..input.len() {
                let s1 = input[i];
                let s2 = input[j];

                if common_chars(s1, s2).count() + 1 == s1.len() {
                    return Ok(common_chars(s1, s2).collect::<String>());
                }
            }
        }

        Err("No solution found".to_string())
    }
}

#[test]
fn test() {
    use crate::input::{test_part_one, test_part_two};

    test_part_one!(
                    "abcdef
bababc
abbcde
abcccd
aabcdd
abcdee
ababab
"
 => "12".into());

    let input = include_str!("day02_input.txt");
    test_part_one!(input => "6972".into());

    test_part_two!(
            "abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz
"
        =>"fgij".into()
    );

    test_part_two!(
        input=>
            "aixwcbzrmdvpsjfgllthdyoqe".into()
    );
}
