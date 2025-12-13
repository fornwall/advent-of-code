use crate::input::Input;

pub fn solve(input: &Input) -> Result<u64, String> {
    let words = input.text.split(' ').collect::<Vec<_>>();
    if words.len() != 19 || words[16].is_empty() || words[18].is_empty() {
        return Err("Invalid input - not expected structure".to_string());
    }

    let wanted_row = words[16][0..(words[16].len() - 1)]
        .parse::<u32>()
        .map_err(|_| "Invalid input")?;
    let wanted_col = words[18][0..(words[18].len() - 1)]
        .parse::<u32>()
        .map_err(|_| "Invalid input")?;

    let mut current_code = 20_151_125;
    let mut current_row = 1;
    let mut current_col = 1;
    while (current_row, current_col) != (wanted_row, wanted_col) {
        if current_row == 1 {
            current_row = 1 + current_col;
            current_col = 1;
        } else {
            current_col += 1;
            current_row -= 1;
        }
        current_code = (current_code * 252_533) % 33_554_393;
    }
    Ok(current_code)
}

#[test]
pub fn tests() {
    let real_input = include_str!("day25_input.txt");
    test_part_one!(real_input => 2_650_453);
}
