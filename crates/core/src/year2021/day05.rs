use crate::input::Input;

struct Board {
    // Bitsets keeping track of squares with coordinates in the range [0,1000].
    claimed_once: [u64; (1000 * 1000) / 64],
    claimed_multiple: [u64; (1000 * 1000) / 64],
}

impl Board {
    const fn new() -> Self {
        Self {
            claimed_once: [0; (1000 * 1000) / 64],
            claimed_multiple: [0; (1000 * 1000) / 64],
        }
    }

    fn claim_square(&mut self, x: i32, y: i32) {
        let bit_idx = x as usize + y as usize * 1000;
        let array_idx = bit_idx / 64;
        let local_bit = 1 << (bit_idx % 64);

        if self.claimed_once[array_idx] & local_bit == 0 {
            self.claimed_once[array_idx] |= local_bit;
        } else {
            self.claimed_multiple[array_idx] |= local_bit;
        }
    }

    fn claimed_multiple(&self) -> u32 {
        self.claimed_multiple.iter().map(|&i| i.count_ones()).sum()
    }

    fn add_line(&mut self, from_x: u16, from_y: u16, to_x: u16, to_y: u16) {
        let mut current_x = i32::from(from_x);
        let mut current_y = i32::from(from_y);

        let dx = (i32::from(to_x) - current_x).signum();
        let dy = (i32::from(to_y) - current_y).signum();

        loop {
            self.claim_square(current_x, current_y);
            current_x += dx;
            current_y += dy;

            if (current_x, current_y) == (i32::from(to_x), i32::from(to_y)) {
                self.claim_square(current_x, current_y);
                break;
            }
        }
    }
}

fn parse_point(s: &str) -> Option<(u16, u16)> {
    if let Some((Ok(x), Ok(y))) = s
        .split_once(',')
        .map(|(x, y)| (x.parse::<u16>(), y.parse::<u16>()))
    {
        return Some((x, y));
    }
    None
}

pub fn solve(input: &mut Input) -> Result<u32, String> {
    let mut board = Board::new();
    for line in input.text.lines() {
        if let Some((from, to)) = line.split_once(" -> ") {
            if let Some((from_x, from_y)) = parse_point(from) {
                if let Some((to_x, to_y)) = parse_point(to) {
                    if from_x < 1000 && from_y < 1000 && to_x < 1000 && to_y < 1000 {
                        let is_straight_line = (from_x == to_x) || (from_y == to_y);
                        let is_diagonal = (i32::from(from_x) - i32::from(to_x)).abs()
                            == (i32::from(from_y) - i32::from(to_y)).abs();

                        match (is_straight_line, is_diagonal, input.is_part_two()) {
                            (false, false, _) => {
                                return Err(format!(
                                    "Line is neither straight nor diagonal: {},{} -> {},{}",
                                    from_x, from_y, to_x, to_y
                                ));
                            }
                            (true, _, _) | (_, true, true) => {
                                board.add_line(from_x, from_y, to_x, to_y);
                            }
                            (false, true, false) => {}
                        }
                        continue;
                    }
                }
            }
        }
        return Err(
            "Input is not in the format 'x1,y1 -> x2,y2' with values in the range [0,1000]"
                .to_string(),
        );
    }
    Ok(board.claimed_multiple())
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    let example = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";
    test_part_one!(example => 5);
    test_part_two!(example => 12);

    let real_input = include_str!("day05_input.txt");
    test_part_one!(real_input => 7644);
    test_part_two!(real_input => 18627);
}
