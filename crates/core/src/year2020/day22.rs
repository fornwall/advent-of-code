use crate::input::Input;
use std::collections::VecDeque;

fn parse_player(input: &str) -> VecDeque<u8> {
    let mut result = VecDeque::new();
    for line in input.lines().skip(1) {
        result.push_back(line.parse::<u8>().unwrap());
    }
    result
}

pub fn solve(input: &mut Input) -> Result<u64, String> {
    let on_error = || "Invalid input".to_string();
    let mut parts = input.text.splitn(2, "\n\n");
    let mut p1 = parse_player(parts.next().ok_or_else(on_error)?);
    let mut p2 = parse_player(parts.next().ok_or_else(on_error)?);
    assert_eq!(p1.len(), p2.len());

    if input.is_part_one() {
        while p1.len() > 0 && p2.len() > 0 {
            let c1 = p1.pop_front().unwrap();
            let c2 = p2.pop_front().unwrap();
            if c1 > c2 {
                p1.push_back(c1);
                p1.push_back(c2);
            } else {
                p2.push_back(c2);
                p2.push_back(c1);
            }
        }

        let mut winner = if p1.is_empty() { p2 } else { p1 };
        let mut result = 0_u64;
        let mut i = 1_u64;
        while let Some(c) = winner.pop_back() {
            result += i * u64::from(c);
            i += 1;
        }
        Ok(result)
    } else {
        Ok(0)
    }
}

#[test]
pub fn tests() {
    use crate::{test_part_one, test_part_two};

    let example = "Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10
";
    test_part_one!(example => 306);
    test_part_two!(example  => 291);

    let real_input = include_str!("day22_input.txt");
    test_part_one!(real_input => 33098);
    // test_part_two!(real_input => 0);
}
