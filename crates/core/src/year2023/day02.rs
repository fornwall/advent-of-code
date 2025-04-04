use crate::input::{Input, on_error};

pub fn solve(input: &Input) -> Result<u32, String> {
    input
        .text
        .lines()
        .map(|game_str| score_game(game_str, input.is_part_one()))
        .sum::<Result<_, _>>()
}

fn score_game(game_str: &str, part1: bool) -> Result<u32, String> {
    let (game_declaration_str, draws_str) = game_str.split_once(": ").ok_or_else(on_error)?;
    let mut max_shown = [0; 3];

    for draw_str in draws_str.split("; ") {
        for color_reveal_str in draw_str.split(", ") {
            let (num_revealed_cubes, color_str) =
                color_reveal_str.split_once(' ').ok_or_else(on_error)?;

            let color_idx = match color_str {
                "red" => 0,
                "green" => 1,
                "blue" => 2,
                _ => return Err(format!("Invalid color: {color_reveal_str}")),
            };

            let num_revealed_cubes =
                u32::from(num_revealed_cubes.parse::<u8>().map_err(|_| on_error())?);
            max_shown[color_idx] = max_shown[color_idx].max(num_revealed_cubes);
        }
    }

    Ok(if part1 {
        u32::from(max_shown[0] <= 12 && max_shown[1] <= 13 && max_shown[2] <= 14)
            * u32::from(
                game_declaration_str
                    .split(' ')
                    .nth(1)
                    .ok_or_else(on_error)?
                    .parse::<u8>()
                    .map_err(|_| on_error())?,
            )
    } else {
        max_shown[0] * max_shown[1] * max_shown[2]
    })
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one_no_allocations, test_part_two_no_allocations};

    let real_input = include_str!("day02_input.txt");
    test_part_one_no_allocations!(real_input => 3035);
    test_part_two_no_allocations!(real_input => 66027);
}
