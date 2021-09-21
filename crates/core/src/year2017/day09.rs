use crate::input::Input;

pub fn solve(input: &mut Input) -> Result<u32, String> {
    let mut result = 0;
    let mut stack = Vec::new();
    let mut ignore_next = false;
    let mut inside_garbage = false;

    for char in input.text.bytes() {
        if ignore_next {
            ignore_next = false;
            continue;
        }

        match (inside_garbage, char) {
            (false, b'<') => {
                inside_garbage = true;
            }
            (false, b'{') => {
                if input.is_part_one() {
                    let this_score = 1 + stack.last().unwrap_or(&0);
                    stack.push(this_score);
                    result += this_score;
                }
            }
            (false, b'}') => {
                stack.pop();
            }
            (true, b'!') => {
                ignore_next = true;
            }
            (true, b'>') => {
                inside_garbage = false;
            }
            (true, _) => {
                if input.is_part_two() {
                    result += 1;
                }
            }
            _ => {}
        }
    }
    Ok(result)
}

#[test]
fn test() {
    use crate::input::{test_part_one, test_part_two};
    let real_input = include_str!("day09_input.txt");
    test_part_one!(real_input => 11089);
    test_part_two!(real_input => 5288);
}
