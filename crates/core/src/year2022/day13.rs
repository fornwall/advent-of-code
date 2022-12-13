use core::iter::Peekable;
use std::cmp::Ordering;

use crate::input::Input;

pub fn solve(input: &mut Input) -> Result<usize, String> {
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
        const MAX_LINES: usize = 510;
        const DIVIDER_PACKET_1: &str = "[[2]]";
        const DIVIDER_PACKET_2: &str = "[[6]]";

        let mut lines = [""; MAX_LINES + 2];
        let mut line_idx = 0;
        for line in input.text.lines() {
            if !line.is_empty() {
                lines[line_idx] = line;
                line_idx += 1;
                if line_idx == MAX_LINES {
                    return Err(format!("Too many lines - max {} supported", MAX_LINES));
                }
            }
        }

        lines[line_idx] = DIVIDER_PACKET_1;
        lines[line_idx + 1] = DIVIDER_PACKET_2;

        let packets = &mut lines[..(line_idx + 2)];

        packets.sort_unstable_by(|a, b| {
            if compare_packets(a, b) {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });

        Ok(packets
            .iter()
            .enumerate()
            .filter_map(|(zero_based_idx, packet)| {
                (packet == &DIVIDER_PACKET_1 || packet == &DIVIDER_PACKET_2)
                    .then_some(zero_based_idx + 1)
            })
            .product())
    }
}

fn is_packet_pair_correctly_ordered(packet_pair: &str) -> bool {
    let mut lines = packet_pair.lines();
    if let (Some(first_line), Some(second_line)) = (lines.next(), lines.next()) {
        compare_packets(first_line, second_line)
    } else {
        false
    }
}

fn compare_packets(packet_1: &str, packet_2: &str) -> bool {
    let mut line_1 = packet_1.bytes().peekable();
    let mut line_2 = packet_2.bytes().peekable();

    let mut value_1 = next_value(&mut line_1);
    let mut value_2 = next_value(&mut line_2);

    let mut coerced_to_list_1 = false;
    let mut coerced_to_list_2 = false;

    loop {
        match (value_1, value_2) {
            (Event::Number(n1), Event::Number(n2)) => {
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
                        if coerced_to_list_1 {
                            coerced_to_list_1 = false;
                            value_1 = Event::ListEnd;
                            value_2 = next_value(&mut line_2);
                            continue;
                        } else if coerced_to_list_2 {
                            coerced_to_list_2 = false;
                            value_2 = Event::ListEnd;
                            value_1 = next_value(&mut line_1);
                            continue;
                        }
                        value_1 = next_value(&mut line_1);
                        value_2 = next_value(&mut line_2);
                        continue;
                    }
                }
            }
            (Event::Number(_), Event::ListStart) => {
                // "If exactly one value is an integer, convert the integer to a list which contains that integer as its
                // only value, then retry the comparison. For example, if comparing [0,0,0] and 2, convert the right value
                // to [2] (a list containing 2); the result is then found by instead comparing [0,0,0] and [2]":
                coerced_to_list_1 = true;
                value_2 = next_value(&mut line_2);
            }
            (Event::ListStart, Event::Number(_)) => {
                coerced_to_list_2 = true;
                value_1 = next_value(&mut line_1);
            }
            (Event::ListEnd, Event::Number(_)) | (Event::ListEnd, Event::ListStart) => {
                // "If the left list runs out of items first, the inputs are in the right order":
                return true;
            }
            (Event::Number(_), Event::ListEnd) | (Event::ListStart, Event::ListEnd) => {
                // "If the right list runs out of items first, the inputs are not in the right order":
                return false;
            }
            (Event::ListEnd, Event::ListEnd) | (Event::ListStart, Event::ListStart) => {
                // "If both values are lists, compare the first value of each list, then the second value,
                // and so on. If the lists are the same length and no comparison makes a decision about the order,
                // continue checking the next part of the input":
                value_1 = next_value(&mut line_1);
                value_2 = next_value(&mut line_2);
                continue;
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Event {
    ListStart,
    ListEnd,
    Number(u8),
}

fn next_value<I: Iterator<Item = u8>>(it: &mut Peekable<I>) -> Event {
    let mut number = 0;
    let mut parsing_number = false;
    while let Some(b) = it.peek() {
        match b {
            b'[' => {
                it.next();
                return Event::ListStart;
            }
            b']' => {
                if parsing_number {
                    return Event::Number(number);
                }
                it.next();
                return Event::ListEnd;
            }
            digit @ b'0'..=b'9' => {
                parsing_number = true;
                number = number * 10 + (digit - b'0');
                it.next();
            }
            b',' => {
                it.next();
                if parsing_number {
                    return Event::Number(number);
                }
            }
            _ => {
                return Event::ListEnd;
            }
        }
    }
    it.next();
    if parsing_number {
        return Event::Number(number);
    }
    Event::ListEnd
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
