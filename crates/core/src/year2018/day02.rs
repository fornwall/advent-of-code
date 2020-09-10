use std::collections::HashMap;

pub fn part1(input_string: &str) -> String {
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

    (picks.0 * picks.1).to_string()
}

pub fn part2(input_string: &str) -> String {
    let input: Vec<&str> = input_string.lines().collect();

    for i in 0..input.len() {
        for j in i + 1..input.len() {
            let s1 = input[i];
            let s2 = input[j];

            let diffs = s1
                .chars()
                .zip(s2.chars())
                .filter(|(c1, c2)| c1 != c2)
                .count();

            if diffs == 1 {
                return s1
                    .chars()
                    .zip(s2.chars())
                    .filter(|pair| pair.0 == pair.1)
                    .map(|pair| pair.0)
                    .collect::<String>();
            }
        }
    }
    panic!("No solution found");
}

#[test]
fn tests_part1() {
    assert_eq!(
        "12",
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

    assert_eq!("6972", part1(include_str!("day02_input.txt")));
}

#[test]
fn tests_part2() {
    assert_eq!(
        "fgij",
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
        "aixwcbzrmdvpsjfgllthdyoqe",
        part2(include_str!("day02_input.txt"))
    );
}
