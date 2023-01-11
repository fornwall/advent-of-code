use core::iter::Peekable;
use std::cmp::Ordering;

use crate::input::Input;

pub fn solve(input: &Input) -> Result<usize, String> {
    if input.is_part_one() {
        Ok(input
            .text
            .split("\n\n")
            .enumerate()
            .filter_map(|(zero_based_index, packet_pair)| {
                is_packet_pair_correctly_ordered(packet_pair).then_some(zero_based_index + 1)
            })
            .sum())
    } else {
        let (packets_1_idx, packets_2_idx) = input
            .text
            .lines()
            .filter(|line| !line.is_empty())
            // "2": Add 1 since indexing in the problem is one based.
            // "6": Add 2 - once from one based indexing, and one from "[[6]]" being after "[[2]]" in the list.
            .fold((1, 2), |acc, packet| {
                (
                    acc.0 + usize::from(is_correctly_ordered(packet, "2")),
                    acc.1 + usize::from(is_correctly_ordered(packet, "6")),
                )
            });
        Ok(packets_1_idx * packets_2_idx)
    }
}

fn is_packet_pair_correctly_ordered(packet_pair: &str) -> bool {
    let mut lines = packet_pair.lines();
    if let (Some(first_line), Some(second_line)) = (lines.next(), lines.next()) {
        is_correctly_ordered(first_line, second_line)
    } else {
        false
    }
}

fn is_correctly_ordered(packet_1: &str, packet_2: &str) -> bool {
    let mut line_1 = packet_1.bytes().peekable();
    let mut line_2 = packet_2.bytes().peekable();

    let mut coerced_to_list_1 = 0;
    let mut coerced_to_list_2 = 0;

    let mut value_1 = next_token(&mut line_1, &mut coerced_to_list_1);
    let mut value_2 = next_token(&mut line_2, &mut coerced_to_list_2);

    loop {
        match (value_1, value_2) {
            (Token::Number(n1), Token::Number(n2)) => {
                match n1.cmp(&n2) {
                    Ordering::Less => {
                        // "If the left integer is lower than the right integer, the inputs are in the right order":
                        return true;
                    }
                    Ordering::Greater => {
                        // "If the left integer is higher than the right integer, the inputs are not in the right order":
                        return false;
                    }
                    Ordering::Equal => {
                        // "Otherwise, the inputs are the same integer; continue checking the next part of the input":
                        value_1 = next_token(&mut line_1, &mut coerced_to_list_1);
                        value_2 = next_token(&mut line_2, &mut coerced_to_list_2);
                    }
                }
            }
            (Token::Number(_), Token::ListStart) => {
                // "If exactly one value is an integer, convert the integer to a list which contains that integer as its
                // only value, then retry the comparison. For example, if comparing [0,0,0] and 2, convert the right value
                // to [2] (a list containing 2); the result is then found by instead comparing [0,0,0] and [2]":
                coerced_to_list_1 += 1;
                value_2 = next_token(&mut line_2, &mut coerced_to_list_2);
            }
            (Token::ListStart, Token::Number(_)) => {
                coerced_to_list_2 += 1;
                value_1 = next_token(&mut line_1, &mut coerced_to_list_1);
            }
            (Token::ListEnd, Token::Number(_)) | (Token::ListEnd, Token::ListStart) => {
                // "If the left list runs out of items first, the inputs are in the right order":
                return true;
            }
            (Token::Number(_), Token::ListEnd) | (Token::ListStart, Token::ListEnd) => {
                // "If the right list runs out of items first, the inputs are not in the right order":
                return false;
            }
            (Token::ListEnd, Token::ListEnd) | (Token::ListStart, Token::ListStart) => {
                // "If both values are lists, compare the first value of each list, then the second value,
                // and so on. If the lists are the same length and no comparison makes a decision about the order,
                // continue checking the next part of the input":
                value_1 = next_token(&mut line_1, &mut coerced_to_list_1);
                value_2 = next_token(&mut line_2, &mut coerced_to_list_2);
            }
        }
    }
}

#[derive(Copy, Clone)]
enum Token {
    ListStart,
    ListEnd,
    Number(u8),
}

fn next_token<I: Iterator<Item = u8>>(it: &mut Peekable<I>, coerced_to_list: &mut i32) -> Token {
    if *coerced_to_list > 0 {
        *coerced_to_list -= 1;
        return Token::ListEnd;
    }

    let mut number = 0;
    let mut parsing_number = false;

    while let Some(b) = it.peek() {
        match b {
            b'[' => {
                it.next();
                return Token::ListStart;
            }
            b']' => {
                if parsing_number {
                    return Token::Number(number);
                }
                it.next();
                return Token::ListEnd;
            }
            digit @ b'0'..=b'9' => {
                parsing_number = true;
                number = number * 10 + (digit - b'0');
                it.next();
            }
            b',' => {
                it.next();
                if parsing_number {
                    return Token::Number(number);
                }
            }
            _ => {
                return Token::ListEnd;
            }
        }
    }

    it.next();
    if parsing_number {
        return Token::Number(number);
    }
    Token::ListEnd
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    let test_input = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";
    test_part_one!(test_input => 13);
    test_part_two!(test_input => 140);

    let real_input = include_str!("day13_input.txt");
    test_part_one!(real_input => 4821);
    test_part_two!(real_input => 21_890);
}

#[test]
pub fn troublesome_packet() {
    let p1 = "[[[9,[9,[0,2]],10,[[0,7,9,4,2],2,[6,7,4,3],[7]],4],[],[[],10,4,5]]]";
    let p2 = "[[[[[9],[7],2,1,8],[[7,5,9],10],[],[]]]]";
    assert!(is_correctly_ordered(p1, p2));
}

#[cfg(feature = "count-allocations")]
#[test]
pub fn no_memory_allocations() {
    use crate::input::{test_part_one, test_part_two};
    let real_input = include_str!("day13_input.txt");
    let allocations = allocation_counter::count(|| {
        test_part_one!(real_input => 4821);
        test_part_two!(real_input => 21_890);
    });
    assert_eq!(allocations, 0);
}
