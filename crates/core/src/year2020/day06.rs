use crate::input::Input;
use std::ops::{BitAnd, BitOr};

type AnswersBitSet = u32;

/// Convert the yes answers of a person, as a string of a-z characters identifying
/// the questions the person answered yes to, to a bit set where:
/// - First bit is set if question 'a' has been answered with yes.
/// - Second bit is set if question 'b' has been answered with yes.
/// - ...
fn person_answers_to_bit_set(answers: &str) -> AnswersBitSet {
    answers
        .bytes()
        .map(|question_identifier| 1 << (question_identifier - b'a'))
        .sum::<AnswersBitSet>()
}

pub fn solve(input: &mut Input) -> Result<AnswersBitSet, String> {
    const GROUP_SEPARATOR: &str = "\n\n";

    if !input
        .text
        .bytes()
        .all(|b| matches!(b, b'a'..=b'z' | b'\r' | b'\n'))
    {
        return Err("Invalid input - only a-z, \r and \n expected".to_string());
    }

    let initial_bit_set = input.part_values(0, AnswersBitSet::MAX);

    let bit_set_merger = if input.is_part_one() {
        BitOr::bitor
    } else {
        BitAnd::bitand
    };

    let computer = |text: &str| {
        Ok(text
            .split(GROUP_SEPARATOR)
            .map(|group_answers| {
                group_answers
                    .lines()
                    .map(person_answers_to_bit_set)
                    .fold(initial_bit_set, bit_set_merger)
                    .count_ones()
            })
            .sum())
    };

    if input.text.contains('\r') {
        // Only call replace() (which results in memory allocation) if necessary.
        computer(&input.text.replace('\r', ""))
    } else {
        computer(input.text)
    }
}

#[test]
pub fn tests() {
    use crate::{test_part_one, test_part_two};

    test_part_one!("abc\n\nabc" => 6);
    test_part_one!("abc\r\n\r\nabc" => 6);

    let real_input = include_str!("day06_input.txt");
    test_part_one!(real_input => 6686);
    test_part_two!(real_input => 3476);
}
