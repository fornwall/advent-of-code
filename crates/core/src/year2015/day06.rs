use crate::input::{Input, Part};

#[derive(Clone, Copy)]
enum Action {
    On,
    Off,
    Toggle,
}

pub fn solve(input: &Input) -> Result<usize, String> {
    fn parse_tuple(tuple: &str) -> Option<(u16, u16)> {
        tuple.split_once(',').and_then(|(first, second)| {
            Some((first.parse::<u16>().ok()?, second.parse::<u16>().ok()?))
        })
    }

    let mut grid = vec![0_u8; 1_000_000].into_boxed_slice();
    for line in input.text.lines() {
        let words = line.split(' ').collect::<Vec<&str>>();

        // Resolve the action once per line instead of comparing command
        // strings for every one of the up to a million cells in the region.
        let (action, from, to) = match words.as_slice() {
            ["turn", "on", from, "through", to] => (Action::On, from, to),
            ["turn", "off", from, "through", to] => (Action::Off, from, to),
            ["toggle", from, "through", to] => (Action::Toggle, from, to),
            _ => return Err("Invalid input".to_string()),
        };

        let (from_x, from_y) = parse_tuple(from).ok_or("Invalid input")?;
        let (to_x, to_y) = parse_tuple(to).ok_or("Invalid input")?;

        // Iterate row-major so consecutive indices are contiguous in memory.
        for y in from_y..=to_y {
            let row_offset = y as usize * 1000;
            for x in from_x..=to_x {
                let cell = &mut grid[row_offset + x as usize];
                *cell = match (input.part, action) {
                    (Part::One, Action::On) => 1,
                    (Part::One, Action::Off) => 0,
                    (Part::One, Action::Toggle) => u8::from(*cell == 0),
                    (Part::Two, Action::On) => *cell + 1,
                    (Part::Two, Action::Off) => cell.saturating_sub(1),
                    (Part::Two, Action::Toggle) => *cell + 2,
                };
            }
        }
    }

    Ok(grid.iter().map(|&i| i as usize).sum())
}

#[test]
pub fn tests() {
    let real_input = include_str!("day06_input.txt");
    test_part_one!(real_input => 569_999);
    test_part_two!(real_input => 17_836_115);
}
