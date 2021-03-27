fn solution(input_string: &str, part1: bool) -> u32 {
    let mut result = 0;
    let mut stack = Vec::new();
    let mut ignore_next = false;
    let mut inside_garbage = false;

    for char in input_string.bytes() {
        if ignore_next {
            ignore_next = false;
            continue;
        }

        match (inside_garbage, char) {
            (false, b'<') => {
                inside_garbage = true;
            }
            (false, b'{') => {
                if part1 {
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
                if !part1 {
                    result += 1;
                }
            }
            _ => {}
        }
    }
    result
}

pub fn part1(input_string: &str) -> Result<u32, String> {
    Ok(solution(input_string, true))
}

pub fn part2(input_string: &str) -> Result<u32, String> {
    Ok(solution(input_string, false))
}

#[test]
fn test_part1() {
    assert_eq!(Ok(11089), part1(include_str!("day09_input.txt")));
}

#[test]
fn test_part2() {
    assert_eq!(Ok(5288), part2(include_str!("day09_input.txt")));
}
