use std::collections::HashMap;

pub fn part1(input_string: &str) -> Result<i64, String> {
    let picks = input_string.lines().fold((0, 0), |state, line| {
        let mut occurrences = HashMap::new();

        line.chars()
            .for_each(|c| *occurrences.entry(c).or_insert(0) += 1);

        let has_occurrence = |count| occurrences.iter().any(|(_key, &value)| value == count);

        (
            state.0 + has_occurrence(2) as i64,
            state.1 + has_occurrence(3) as i64,
        )
    });

    Ok(picks.0 * picks.1)
}

pub fn part2(input_string: &str) -> Result<String, String> {
    fn common_chars<'a>(s1: &'a str, s2: &'a str) -> impl Iterator<Item = char> + 'a {
        s1.chars()
            .zip(s2.chars())
            .filter_map(|(c1, c2)| if c1 == c2 { Some(c1) } else { None })
    }

    let input: Vec<&str> = input_string.lines().collect();

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

#[test]
fn tests_part1() {
    assert_eq!(
        Ok(12),
        part1(
            "abcdef
bababc
abbcde
abcccd
aabcdd
abcdee
ababab
"
        )
    );

    assert_eq!(Ok(6972), part1(include_str!("day02_input.txt")));
}

#[test]
fn tests_part2() {
    assert_eq!(
        Ok("fgij".to_string()),
        part2(
            "abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz
"
        )
    );

    assert_eq!(
        Ok("aixwcbzrmdvpsjfgllthdyoqe".to_string()),
        part2(include_str!("day02_input.txt"))
    );
}
