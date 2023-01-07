use crate::input::Input;

//   0:      1:      2:      3:      4:     5:      6:      7:      8:      9:
//  aaaa    ....    aaaa    aaaa    ....    aaaa    aaaa    aaaa    aaaa    aaaa
// b    c  .    c  .    c  .    c  b    c  b    .  b    .  .    c  b    c  b    c
// b    c  .    c  .    c  .    c  b    c  b    .  b    .  .    c  b    c  b    c
//  ....    ....    dddd    dddd    dddd    dddd    dddd    ....    dddd    dddd
// e    f  .    f  e    .  .    f  .    f  .    f  e    f  .    f  e    f  .    f
// e    f  .    f  e    .  .    f  .    f  .    f  e    f  .    f  e    f  .    f
//  gggg    ....    gggg    gggg    ....   gggg     gggg    ....    gggg    gggg
//
// Digits:
// 0: 6 segments
// 1: 2 segments (unique) - c,f
// 2: 5 segments
// 3: 5 segments
// 4: 4 segments (unique) - b,c,d,f
// 5: 5 segments
// 6: 6 segments
// 7: 3 segments (unique) - a,c,f
// 8: 7 segments (unique) - a,b,c,d,e,f (all)
// 9: 6 segments

fn pattern_as_bitset(pattern: &str) -> u8 {
    pattern.bytes().map(|b| 1 << (b - b'a')).sum()
}

pub fn solve(input: &Input) -> Result<u32, String> {
    let mut result = 0;
    for line in input.text.lines() {
        if let Some((from, to)) = line.split_once(" | ") {
            if input.is_part_one() {
                result += to
                    .split(' ')
                    .filter(|s| matches!(s.len(), 2 | 3 | 4 | 7))
                    .count() as u32;
            } else {
                let signal_patterns = from.split(' ').collect::<Vec<&str>>();
                if signal_patterns.len() != 10 {
                    return Err("Not 10 unique signal patterns to left of '|'".to_string());
                }

                let mut number_to_pattern = [0_u8; 10];
                for pattern in signal_patterns.iter() {
                    let identified_digit = match pattern.len() {
                        2 => 1,
                        3 => 7,
                        4 => 4,
                        7 => 8,
                        _ => {
                            continue;
                        }
                    };
                    number_to_pattern[identified_digit] = pattern_as_bitset(pattern);
                }

                for pattern in signal_patterns {
                    // Use our above identified numbers to identify
                    // - The three digits (2, 3 and 5) with 5 segments,
                    // - The three digits (0, 6 and 9) with 6 segments.
                    let pattern_bitset = pattern_as_bitset(pattern);
                    let found_digit = match (
                        pattern.len(),
                        (pattern_bitset & number_to_pattern[1]).count_ones(),
                        (pattern_bitset & number_to_pattern[4]).count_ones(),
                    ) {
                        (5, 1, 2) => 2,
                        (5, 2, _) => 3,
                        (5, 1, _) => 5,
                        (6, 2, 3) => 0,
                        (6, 1, _) => 6,
                        (6, 2, 4) => 9,
                        _ => {
                            continue;
                        }
                    };
                    number_to_pattern[found_digit] = pattern_bitset;
                }

                result += to
                    .split(' ')
                    .rev()
                    .enumerate()
                    .map(|(digit_idx, digit_pattern)| {
                        let digit_bitset = pattern_as_bitset(digit_pattern);
                        for (index, &index_pattern) in number_to_pattern.iter().enumerate() {
                            if index_pattern == digit_bitset {
                                return index as u32 * u32::pow(10, digit_idx as u32);
                            }
                        }
                        0
                    })
                    .sum::<u32>();
            }
        }
    }
    Ok(result)
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    let example =
        "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
    test_part_two!(example => 5353);

    let example =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";
    test_part_one!(example => 26);
    test_part_two!(example => 61_229);

    let real_input = include_str!("day08_input.txt");
    test_part_one!(real_input => 321);
    test_part_two!(real_input => 1_028_926);
}
