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

pub fn solve(input: &Input) -> Result<AnswersBitSet, String> {
    const GROUP_SEPARATOR: &str = "\n\n";

    if !input.text.bytes().all(|b| matches!(b, b'a'..=b'z' | b'\n')) {
        return Err("Invalid input - only a-z, \\n expected".to_string());
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

    computer(input.text)
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one_no_allocations, test_part_two_no_allocations};

    test_part_one_no_allocations!("abc\n\nabc" => 6);

    let real_input = include_str!("day06_input.txt");
    test_part_one_no_allocations!(real_input => 6686);
    test_part_two_no_allocations!(real_input => 3476);
}
