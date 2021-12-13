use crate::input::Input;

pub fn solve(input: &mut Input) -> Result<u64, String> {
    let mut syntax_error_score = 0;
    let mut autocomplete_scores = Vec::new();

    'next_line: for line in input.text.lines() {
        let mut expecting = Vec::new();
        for c in line.bytes() {
            let closing = match c {
                b'(' => b')',
                b'[' => b']',
                b'{' => b'}',
                b'<' => b'>',
                b')' | b']' | b'}' | b'>' => {
                    if let Some(expected_closer) = expecting.pop() {
                        if expected_closer != c {
                            syntax_error_score += score_syntax_error(c);
                            continue 'next_line;
                        }
                        continue;
                    } else {
                        return Err("Too many chunk closing characters in line".to_string());
                    }
                }
                _ => {
                    return Err(format!("Invalid character in line: '{}'", c as char));
                }
            };
            expecting.push(closing);
        }
        if input.is_part_two() && !expecting.is_empty() {
            autocomplete_scores.push(score_autocomplete(&expecting));
        }
    }

    if input.is_part_one() {
        Ok(syntax_error_score)
    } else {
        autocomplete_scores.sort_unstable();
        Ok(autocomplete_scores[autocomplete_scores.len() / 2])
    }
}

const fn score_syntax_error(character: u8) -> u64 {
    match character {
        b')' => 3,
        b']' => 57,
        b'}' => 1197,
        _ => 25137,
    }
}

fn score_autocomplete(autocomplete: &[u8]) -> u64 {
    autocomplete.iter().rev().fold(0, |score, c| {
        score * 5
            + match c {
                b')' => 1,
                b']' => 2,
                b'}' => 3,
                _ => 4,
            }
    })
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    let example = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";
    test_part_one!(example => 26_397);
    test_part_two!(example => 288_957);

    let real_input = include_str!("day10_input.txt");
    test_part_one!(real_input => 290_691);
    test_part_two!(real_input => 2_768_166_558);
}
