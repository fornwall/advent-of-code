use crate::input::Input;
use regex::Regex;
use std::collections::HashMap;

fn expand_rule(rule_idx: &str, rules: &HashMap<&str, Vec<String>>) -> String {
    let parts = rules.get(rule_idx).unwrap();
    let mut result = String::new();
    for part in parts.iter() {
        if part == "a" || part == "b" || part == "|" {
            result += &part;
        } else {
            result += "(";
            result += &expand_rule(&part, &rules);
            result += ")";
        }
    }
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

    println!("RULE 0: {:?}", rules.get("0"));
    let expanded_rule_zero = expand_rule("0", &rules);
    println!("Expanded 0: {}", expanded_rule_zero);
    let regexp = format!("^({})$", expanded_rule_zero);
    println!("regexp: {}", regexp);

    let re = Regex::new(&regexp).unwrap();
    Ok(messages_part
        .lines()
        .filter(|line| re.is_match(line))
        .count() as u64)
}

#[test]
pub fn tests() {
    use crate::test_part_one; //, test_part_two};

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
    //let example_part_two = "";
    //test_part_two!(example_part_two => 0);

    let real_input = include_str!("day19_input.txt");
    test_part_one!(real_input => 126);
    // test_part_two!(real_input => 0);
}
