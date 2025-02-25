use crate::common::array_stack::ArrayStack;
use crate::input::{Input, on_error};

pub fn solve(input: &Input) -> Result<i32, String> {
    let mut stack = ArrayStack::<512, i32>::new();
    let mut sum = 0;

    for line in input.text.lines() {
        stack.clear();
        for num in line.split_ascii_whitespace() {
            stack.push(num.parse().map_err(|_| on_error())?)?;
        }

        if input.is_part_two() {
            stack.slice_mut().reverse();
        }

        while stack.slice().iter().any(|i| *i != 0) {
            for i in 0..stack.len() - 1 {
                stack.elements[i] = stack.elements[i + 1] - stack.elements[i];
            }
            sum += stack.pop_unwrap();
        }
    }

    Ok(sum)
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one_no_allocations, test_part_two_no_allocations};

    let test_input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
    test_part_one_no_allocations!(test_input => 114);
    test_part_two_no_allocations!(test_input => 2);

    let real_input = include_str!("day09_input.txt");
    test_part_one_no_allocations!(real_input => 2_005_352_194);
    test_part_two_no_allocations!(real_input => 1077);
}
