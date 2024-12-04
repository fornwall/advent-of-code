use crate::input::{on_error, Input};

pub fn solve(input: &Input) -> Result<u32, String> {
    let width = input
        .text
        .lines()
        .next()
        .map(|line| line.len())
        .ok_or_else(on_error)? as i32;

    let letter_board = LetterBoard {
        s: input.text.as_bytes(),
        width,
    };

    if letter_board.s.len() != ((letter_board.width + 1) * letter_board.width - 1) as usize {
        return Err("Invalid input - not a rectangle".to_string());
    }

    let mut num_xmas = 0;

    if input.is_part_one() {
        for x in 0..letter_board.width {
            for y in 0..letter_board.width {
                if letter_board.at((x, y)) == b'X' {
                    for (p1, p2, p3) in [
                        ((1, 0), (2, 0), (3, 0)),
                        ((0, 1), (0, 2), (0, 3)),
                        ((1, 1), (2, 2), (3, 3)),
                        ((1, -1), (2, -2), (3, -3)),
                    ] {
                        for mult in [-1, 1] {
                            let c1 = letter_board.at((x + p1.0 * mult, y + p1.1 * mult));
                            if c1 == b'M' {
                                let c2 = letter_board.at((x + p2.0 * mult, y + p2.1 * mult));
                                if c2 == b'A' {
                                    let c3 = letter_board.at((x + p3.0 * mult, y + p3.1 * mult));
                                    if c3 == b'S' {
                                        num_xmas += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    } else {
        for x in 1..(letter_board.width - 1) {
            for y in 1..(letter_board.width - 1) {
                if letter_board.at((x, y)) == b'A' {
                    let top_left = letter_board.at((x - 1, y - 1));
                    let bottom_left = letter_board.at((x - 1, y + 1));
                    let top_right = letter_board.at((x + 1, y - 1));
                    let bottom_right = letter_board.at((x + 1, y + 1));
                    if matches!(
                        (top_left, bottom_left, top_right, bottom_right),
                        (b'M', b'M', b'S', b'S')
                            | (b'S', b'S', b'M', b'M')
                            | (b'M', b'S', b'M', b'S')
                            | (b'S', b'M', b'S', b'M')
                    ) {
                        num_xmas += 1;
                    }
                }
            }
        }
    }
    Ok(num_xmas)
}

struct LetterBoard<'a> {
    s: &'a [u8],
    width: i32,
}

impl LetterBoard<'_> {
    const fn at(&self, position: (i32, i32)) -> u8 {
        if position.0 < 0 || position.0 >= self.width || position.1 < 0 || position.1 >= self.width
        {
            return b' ';
        }
        self.s[(position.0 + (self.width + 1) * position.1) as usize]
    }
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one_no_allocations, test_part_two_no_allocations};

    let test_input = "X...
.M..
..A.
...S";
    test_part_one_no_allocations!(test_input => 1);
    let test_input = "...S
..A.
.M..
X...";
    test_part_one_no_allocations!(test_input => 1);
    let test_input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
    test_part_one_no_allocations!(test_input => 18);
    test_part_two_no_allocations!(test_input => 9);

    let real_input = include_str!("day04_input.txt");
    test_part_one_no_allocations!(real_input => 2573);
    test_part_two_no_allocations!(real_input => 1850);
}
