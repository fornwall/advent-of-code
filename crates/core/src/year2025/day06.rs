use crate::{
    common::array_stack::ArrayStack,
    input::{Input, on_error},
};

pub fn solve(input: &Input) -> Result<u64, String> {
    const MAX_COLUMNS: usize = 6400;
    let mut operators = ArrayStack::<MAX_COLUMNS, u8>::new();
    let mut column_values = ArrayStack::<MAX_COLUMNS, u64>::new();

    let mut column_widths = ArrayStack::<MAX_COLUMNS, usize>::new();
    let mut column_offsets = ArrayStack::<MAX_COLUMNS, usize>::new();

    for (line_idx, line) in input.text.lines().rev().enumerate() {
        if line_idx == 0 {
            // First line: determine column widths and offsets
            let mut last_offset = 0;
            for (offset, b) in line.bytes().enumerate() {
                match b {
                    b'+' | b'*' => {
                        let first_operator = operators.len() == 0;

                        operators.push(b)?;
                        column_offsets.push(offset)?;

                        if !first_operator {
                            column_widths.push(offset - last_offset - 1)?;
                            last_offset = offset;
                        }
                    }
                    b' ' => {}
                    _ => {
                        return Err(format!("Invalid character in operator line: {}", b as char));
                    }
                }
            }
            column_widths.push(line.len() - last_offset)?;
        } else {
            if input.is_part_two() {
                break;
            }
            for (part_idx, part) in line.split_whitespace().enumerate() {
                let new_value = part.trim().parse::<u64>().map_err(|e| e.to_string())?;
                if line_idx == 1 {
                    column_values.push(new_value)?;
                } else {
                    let current_value = column_values.elements[part_idx];
                    let column_operator = operators.elements[part_idx];
                    let updated_value = match column_operator {
                        b'+' => current_value + new_value,
                        b'*' => current_value * new_value,
                        _ => return Err(format!("Unknown operator: {}", column_operator as char)),
                    };
                    column_values.elements[part_idx] = updated_value;
                }
            }
        }
    }
    if input.is_part_one() {
        Ok(column_values.slice().iter().sum())
    } else {
        let mut total_value = 0;
        let num_lines = input.text.lines().count() - 1;
        let line_width = input.text.lines().next().ok_or_else(on_error)?.len() + 1; // +1 for newline
        let grid = input.text.as_bytes();
        for (col_idx, col_width) in column_widths.slice().iter().enumerate() {
            let col_offset = column_offsets.elements[col_idx];
            let mut col_value = 0;
            let operator = operators.elements[col_idx];

            let mut first = true;
            for byte_offset in (col_offset..(col_offset + col_width)).rev() {
                let mut multiplier = 1;
                let mut byte_offset_value: u64 = 0;

                for line_idx in (0..num_lines).rev() {
                    let total_offset = line_idx * line_width + byte_offset;
                    let char = grid[total_offset];
                    let digit = match char {
                        b' ' => continue,
                        _ if char.is_ascii_digit() => char - b'0',
                        _ => return Err(format!("Invalid digit character: {}", char as char)),
                    };
                    let digit_value = (digit as u64) * multiplier;
                    multiplier *= 10;
                    byte_offset_value += digit_value;
                }

                if first {
                    col_value = byte_offset_value;
                    first = false;
                } else {
                    col_value = match operator {
                        b'+' => byte_offset_value + col_value,
                        b'*' => byte_offset_value * col_value,
                        _ => return Err(format!("Unknown operator: {}", operator as char)),
                    };
                }
            }
            total_value += col_value;
        }
        Ok(total_value)
    }
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one_no_allocations, test_part_two_no_allocations};

    let test_input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";
    test_part_one_no_allocations!(test_input => 4_277_556);
    test_part_two_no_allocations!(test_input => 3_263_827);

    let real_input = include_str!("day06_input.txt");
    test_part_one_no_allocations!(real_input => 6_343_365_546_996);
    test_part_two_no_allocations!(real_input => 11_136_895_955_912);
}
