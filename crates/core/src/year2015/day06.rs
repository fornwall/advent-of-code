use crate::input::{Input, Part};

pub fn solve(input: &mut Input) -> Result<usize, String> {
    fn parse_tuple(tuple: &str) -> Option<(u16, u16)> {
        tuple.split_once(',').and_then(|(first, second)| {
            Some((first.parse::<u16>().ok()?, second.parse::<u16>().ok()?))
        })
    }

    let mut grid = vec![0_u8; 1_000_000].into_boxed_slice();
    for line in input.text.lines() {
        let words = line.split(' ').collect::<Vec<&str>>();
        let is_toggle = words[0] == "toggle";
        let expected_word_count = if is_toggle { 4 } else { 5 };
        if words.len() != expected_word_count {
            return Err("Invalid input".to_string());
        }

        let (from, to) = if is_toggle {
            (words[1], words[3])
        } else {
            (words[2], words[4])
        };

        let (from_x, from_y) = parse_tuple(from).ok_or("Invalid input")?;
        let (to_x, to_y) = parse_tuple(to).ok_or("Invalid input")?;

        for x in from_x..=to_x {
            for y in from_y..=to_y {
                let index = x as usize + y as usize * 1000;
                grid[index] = match (words[1], input.part) {
                    ("on", Part::One) => 1,
                    ("on", Part::Two) => grid[index] + 1,
                    ("off", Part::One) => 0,
                    ("off", Part::Two) => grid[index] - u8::from(grid[index] != 0),
                    (_, Part::One) => u8::from(grid[index] == 0),
                    (_, Part::Two) => grid[index] + 2,
                };
            }
        }
    }

    Ok(grid.iter().map(|&i| i as usize).sum())
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    let real_input = include_str!("day06_input.txt");
    test_part_one!(real_input => 569_999);
    test_part_two!(real_input => 17_836_115);
}
