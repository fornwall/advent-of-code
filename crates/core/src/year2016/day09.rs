use crate::input::Input;

fn uncompressed_size(text: &[u8], recursive: bool) -> Result<u64, String> {
    let error_mapper_uf8 = |_| "Invalid input";
    let error_mapper_parse = |_| "Invalid input";
    let mut start_parenthesis_idx = None;
    let mut uncompressed_len = 0_u64;

    let mut i = 0;
    while i < text.len() {
        let c = text[i];
        if c == b'(' {
            start_parenthesis_idx = Some(i);
        } else if c == b')' {
            if let Some(from) = start_parenthesis_idx {
                let inside_parenthesis = &text[from + 1..i];
                let parts = inside_parenthesis
                    .split(|&c| c == b'x')
                    .collect::<Vec<&[u8]>>();
                if parts.len() != 2 {
                    return Err("Invalid input".into());
                }
                let chars_to_take = std::str::from_utf8(parts[0])
                    .map_err(error_mapper_uf8)?
                    .parse::<u64>()
                    .map_err(error_mapper_parse)?;
                let repetitions = std::str::from_utf8(parts[1])
                    .map_err(error_mapper_uf8)?
                    .parse::<u64>()
                    .map_err(error_mapper_parse)?;
                uncompressed_len += repetitions
                    * if recursive {
                        uncompressed_size(&text[i + 1..i + 1 + chars_to_take as usize], true)?
                    } else {
                        chars_to_take
                    };
                i += chars_to_take as usize;
                start_parenthesis_idx = None;
            }
        } else if start_parenthesis_idx.is_none() {
            uncompressed_len += 1;
        }
        i += 1;
    }

    Ok(uncompressed_len)
}

pub fn solve(input: &Input) -> Result<u64, String> {
    let text = input.text.as_bytes();
    uncompressed_size(text, input.is_part_two())
}

#[test]
pub fn tests() {
    test_part_one!("ADVENT" => 6);
    test_part_one!("A(1x5)BC" => 7);
    test_part_one!("(3x3)XYZ" => 9);
    test_part_one!("A(2x2)BCD(2x2)EFG" => 11);

    test_part_two!("(3x3)XYZ" => 9);
    test_part_two!("X(8x2)(3x3)ABCY" => 20);

    let real_input = include_str!("day09_input.txt");
    test_part_one!(real_input => 183_269);
    test_part_two!(real_input => 11_317_278_863);
}
