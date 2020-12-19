use crate::input::Input;

#[derive(Clone)]
enum Rule {
    Character(u8),
    Sequences(Vec<Vec<RuleId>>),
}

impl Rule {
    fn parse(pattern_str: &str) -> Self {
        if pattern_str.as_bytes()[0] == b'"' {
            Self::Character(pattern_str.as_bytes()[1])
        } else {
            Self::Sequences(
                pattern_str
                    .split(" | ")
                    .map(|s| s.split(" ").map(|s| s.parse().unwrap()).collect())
                    .collect(),
            )
        }
    }
}

type RuleId = u8;

struct Rules {
    rules: Vec<Rule>,
}

impl Rules {
    fn parse(rules_str: &str) -> Self {
        let mut rules = Self {
            rules: vec![Rule::Character(0); 255],
        };
        for rule_line in rules_str.lines() {
            rules.add_line(rule_line);
        }
        rules
    }

    fn add_line(&mut self, rule_line: &str) {
        let mut rule_line_parts = rule_line.split(": ");
        let rule_idx_str = rule_line_parts.next().unwrap();
        let pattern_str = rule_line_parts.next().unwrap();

        let rule_idx = rule_idx_str.parse::<RuleId>().unwrap();
        let pattern = Rule::parse(pattern_str);

        self.rules[rule_idx as usize] = pattern;
    }

    fn matches(&self, line: &str) -> bool {
        struct Match<'a> {
            remaining_input: &'a str,
            remaining_sequence: Vec<RuleId>,
        }

        let mut stack = Vec::new();
        stack.push(Match {
            remaining_input: line,
            remaining_sequence: vec![0],
        });

        while let Some(m) = stack.pop() {
            match &self.rules[m.remaining_sequence[0] as usize] {
                &Rule::Character(value) => {
                    if m.remaining_input.as_bytes()[0] == value {
                        let end_of_input = m.remaining_input.len() == 1;
                        let end_of_rule_sequence = m.remaining_sequence.len() == 1;
                        match (end_of_input, end_of_rule_sequence) {
                            (true, true) => {
                                return true;
                            }
                            (false, false) => {
                                stack.push(Match {
                                    remaining_input: &m.remaining_input[1..],
                                    remaining_sequence: m.remaining_sequence[1..].to_vec(),
                                });
                            }
                            _ => {}
                        }
                    }
                }
                Rule::Sequences(choices) => {
                    for chosen_sequence in choices.iter() {
                        let mut remaining_sequence = chosen_sequence.clone();
                        remaining_sequence.extend(&m.remaining_sequence[1..]);
                        stack.push(Match {
                            remaining_input: m.remaining_input,
                            remaining_sequence,
                        });
                    }
                }
            }
        }
        false
    }
}

pub fn solve(input: &mut Input) -> Result<u64, String> {
    let on_error = || "Invalid input".to_string();
    let mut input_parts = input.text.split("\n\n");
    let rules_str = input_parts.next().ok_or_else(on_error)?;
    let messages_str = input_parts.next().ok_or_else(on_error)?;

    let mut rules = Rules::parse(rules_str);

    if input.is_part_two() {
        rules.add_line("8: 42 | 42 8");
        rules.add_line("11: 42 31 | 42 11 31");
    }

    Ok(messages_str
        .lines()
        .filter(|line| rules.matches(line))
        .count() as u64)
}

#[test]
pub fn tests() {
    use crate::{test_part_one, test_part_two};

    let example_part_one = "0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"

ababbb
bababa
abbbab
aaabbb
aaaabbb";
    test_part_one!(example_part_one => 2);
    let example_part_two = "42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: \"a\"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: \"b\"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba";
    test_part_two!(example_part_two => 12);

    let real_input = include_str!("day19_input.txt");
    test_part_one!(real_input => 126);
    test_part_two!(real_input => 282);
}
