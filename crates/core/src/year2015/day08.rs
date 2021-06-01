use crate::Input;

pub fn solve(input: &mut Input) -> Result<usize, String> {
    let mut result = 0;
    for line in input.text.lines() {
        let num_chars = line.len();

        // Strip leading and trailing quotes.
        if !(line.starts_with('"') && line.ends_with('"') && line.len() >= 2) {
            return Err("Invalid input - not surrounded by quotes".to_string());
        }
        let line = &line[1..line.len() - 1].as_bytes();

        let mut idx = 0;

        if input.is_part_one() {
            let mut memory_size = 0;
            while idx < line.len() {
                memory_size += 1;
                if line[idx] == b'\\' {
                    if line[idx + 1] == b'x' {
                        idx += 4;
                    } else {
                        idx += 2;
                    }
                } else {
                    idx += 1;
                }
            }
            result += num_chars - memory_size;
        } else {
            let mut encoded_size = 6; // For starting and trailing quote.
            while idx < line.len() {
                encoded_size += 1;
                if line[idx] == b'\\' {
                    if idx + 1 == line.len() {
                        return Err("Invalid input".to_string());
                    }
                    if line[idx + 1] == b'x' {
                        encoded_size += 4;
                        idx += 4;
                    } else {
                        encoded_size += 3;
                        idx += 2;
                    }
                } else {
                    idx += 1;
                }
            }
            result += encoded_size - num_chars;
        }
    }

    Ok(result)
}

#[test]
pub fn tests() {
    use crate::{test_part_one, test_part_two};

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
