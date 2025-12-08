use crate::input::{Input, on_error};

pub fn solve(input: &Input) -> Result<u64, String> {
    const MAX_WIDTH: usize = 150;
    let grid_width = input.text.lines().next().ok_or_else(on_error)?.len();
    if grid_width > MAX_WIDTH {
        return Err("Input too large".to_string());
    }

    let mut num_timelines_at_x = [0_u64; MAX_WIDTH];
    let beam_start_x = input
        .text
        .bytes()
        .position(|b| b == b'S')
        .ok_or_else(on_error)?;
    num_timelines_at_x[beam_start_x] = 1;

    let mut num_splits = 0;
    for line in input.text.lines().skip(1) {
        for (x, _) in line.bytes().enumerate().filter(|(_, c)| *c == b'^') {
            let num_timelines_here = num_timelines_at_x[x];
            if num_timelines_here > 0 {
                num_splits += 1;
                num_timelines_at_x[x - 1] += num_timelines_here;
                num_timelines_at_x[x] = 0;
                num_timelines_at_x[x + 1] += num_timelines_here;
            }
        }
    }

    Ok(if input.is_part_one() {
        num_splits
    } else {
        num_timelines_at_x.iter().sum()
    })
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one_no_allocations, test_part_two_no_allocations};

    let test_input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
    test_part_one_no_allocations!(test_input => 21);
    test_part_two_no_allocations!(test_input => 40);

    let real_input = include_str!("day07_input.txt");
    test_part_one_no_allocations!(real_input => 1638);
    test_part_two_no_allocations!(real_input => 7_759_107_121_385);
}
