use crate::input::Input;
use regex::Regex;
use std::collections::HashMap;

fn expand_rule(rule_idx: &str, rules: &HashMap<&str, Vec<String>>, depth: u32) -> String {
    if depth > 17 {
        return "".to_string();
    }
    let parts = rules.get(rule_idx).unwrap();
    let mut result = String::new();
    result += "(";
    for part in parts.iter() {
        if part == "a" || part == "b" || part == "|" {
            result += &part;
        } else {
            result += &expand_rule(&part, &rules, depth + 1);
        }
    }
    result += ")";
    result
}

pub fn solve(input: &mut Input) -> Result<u64, String> {
    let on_error = || "Invalid input".to_string();
    let mut input_parts = input.text.split("\n\n");
    let rules_part = input_parts.next().ok_or_else(on_error)?;
    let messages_part = input_parts.next().ok_or_else(on_error)?;

    //let mut expanded_rules = HashMap::new();
    let mut rules = HashMap::new();

    for rule_line in rules_part.lines() {
        let mut rule_line_parts = rule_line.split(": ");
        let rule_idx = rule_line_parts.next().ok_or_else(on_error)?;
        let rule_parts: Vec<String> = rule_line_parts
            .next()
            .ok_or_else(on_error)?
            .split(" ")
            .map(|p| p.replace('"', ""))
            .collect();
        rules.insert(rule_idx, rule_parts);
    }

    if !input.is_part_one() {
        rules.insert(
            "8",
            vec![
                "42".to_string(),
                "|".to_string(),
                "42".to_string(),
                "8".to_string(),
            ],
        );
        rules.insert(
            "11",
            vec![
                "42".to_string(),
                "31".to_string(),
                "|".to_string(),
                "42".to_string(),
                "11".to_string(),
                "31".to_string(),
            ],
        );
    }

    println!("RULE 0: {:?}", rules.get("0"));
    let expanded_rule_zero = expand_rule("0", &rules, 0);
    println!("Expanded 0: {}", expanded_rule_zero);
    let regexp = format!("^{}$", expanded_rule_zero);
    println!("regexp: {}", regexp);

    let re = Regex::new(&regexp).unwrap();
    Ok(messages_part
        .lines()
        .filter(|line| re.is_match(line))
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
