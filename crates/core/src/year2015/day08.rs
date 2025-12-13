use crate::input::Input;

pub fn solve(input: &Input) -> Result<usize, String> {
    let mut result = 0;
    for line in input.text.lines() {
        let num_chars = line.len();

        // Strip leading and trailing quotes.
        if !(line.starts_with('"') && line.ends_with('"') && line.len() >= 2) {
            return Err("Invalid input - not surrounded by quotes".to_string());
        }
        let line = &line.as_bytes()[1..line.len() - 1];

        let mut idx = 0;
        // In part 2, 6 is for starting and trailing quote:
        let mut encoded_size = input.part_values(0, 6);
        while idx < line.len() {
            encoded_size += 1;
            if line[idx] == b'\\' {
                if idx + 1 == line.len() {
                    return Err("Invalid input".to_string());
                }
                if line[idx + 1] == b'x' {
                    if input.is_part_two() {
                        encoded_size += 4;
                    }
                    idx += 4;
                } else {
                    if input.is_part_two() {
                        encoded_size += 3;
                    }
                    idx += 2;
                }
            } else {
                idx += 1;
            }
        }
        result += if input.is_part_one() {
            num_chars - encoded_size
        } else {
            encoded_size - num_chars
        };
    }

    Ok(result)
}

#[test]
pub fn tests() {
    test_part_one!("\"\"" => 2);
    test_part_one!("\"abc\"" => 2);
    test_part_one!("\"aaa\\\"aaa\"" => 3);
    test_part_one!("\"\\x27\"" => 5);

    test_part_two!("\"\"" => 4);
    test_part_two!("\"abc\"" => 4);
    test_part_two!("\"aaa\\\"aaa\"" => 6);
    test_part_two!("\"\\x27\"" => 5);

    let real_input = include_str!("day08_input.txt");
    test_part_one!(real_input => 1333);
    test_part_two!(real_input => 2046);
}
