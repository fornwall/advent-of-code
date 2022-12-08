use crate::input::Input;

struct Grid {
    grid_size: usize,
    values: [u8; Self::MAX_SIZE * Self::MAX_SIZE],
}

impl Grid {
    const MAX_SIZE: usize = 100;
    const fn new(grid_size: usize) -> Self {
        Self {
            values: [0; Self::MAX_SIZE * Self::MAX_SIZE],
            grid_size,
        }
    }

    fn parse(input: &str) -> Result<Self, String> {
        let mut grid = Self::new(input.lines().count());
        for (row_idx, row) in input.lines().enumerate() {
            for (col_idx, col) in row.bytes().enumerate() {
                grid.set(col_idx, row_idx, col - b'0');
            }
        }
        Ok(grid)
    }

    fn set(&mut self, x: usize, y: usize, value: u8) {
        self.values[y * self.grid_size + x] = value;
    }

    fn get(&mut self, x: usize, y: usize) -> u8 {
        self.values[y * self.grid_size + x]
    }
}

struct GridVisibility {
    grid_size: usize,
    values: [bool; Grid::MAX_SIZE * Grid::MAX_SIZE],
}

impl GridVisibility {
    const fn new(grid_size: usize) -> Self {
        Self {
            values: [false; Grid::MAX_SIZE * Grid::MAX_SIZE],
            grid_size,
        }
    }

    fn mark_visible(&mut self, x: usize, y: usize) {
        self.values[y * self.grid_size + x] = true;
    }

    fn num_visible(&self) -> usize {
        self.values.iter().filter(|&&b| b).count()
    }
}

struct ScenicScores {
    grid_size: usize,
    values: [u32; Grid::MAX_SIZE * Grid::MAX_SIZE],
}

impl ScenicScores {
    const fn new(grid_size: usize) -> Self {
        Self {
            values: [1; Grid::MAX_SIZE * Grid::MAX_SIZE],
            grid_size,
        }
    }

    fn add(&mut self, x: usize, y: usize, val: u16) {
        self.values[y * self.grid_size + x] *= u32::from(val);
    }

    fn highest_score(&self) -> usize {
        self.values.iter().max().copied().unwrap_or_default() as usize
    }
}

pub fn solve(input: &mut Input) -> Result<usize, String> {
    let mut grid = Grid::parse(input.text)?;
    let mut grid_visibility = GridVisibility::new(grid.grid_size);
    let mut scenic_scores = ScenicScores::new(grid.grid_size);

    let edge = (grid.grid_size - 1) as i32;
    let mut seen_higher: Vec<(i32, i32)> = Vec::with_capacity(grid.grid_size);

    for direction in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
        for start in 0..=edge {
            // Go in the direction to determine the
            seen_higher.clear();

            let (mut position, other_end) = match direction {
                (1, 0) => ((0, start), (edge, start)),
                (-1, 0) => ((edge, start), (0, start)),
                (0, 1) => ((start, 0), (start, edge)),
                _ => ((start, edge), (start, 0)),
            };

            while (0..=edge).contains(&position.0) && (0..=edge).contains(&position.1) {
                // Pop while top of stack is not higher than current tree, as they
                // are blocked by this tree when looking from the other direction.
                while seen_higher
                    .last()
                    .map(|&stack_pos| {
                        grid.get(stack_pos.0 as usize, stack_pos.1 as usize)
                            <= grid.get(position.0 as usize, position.1 as usize)
                    })
                    .unwrap_or_default()
                {
                    #[allow(clippy::unwrap_used)]
                    let lower = seen_higher.pop().unwrap();
                    // Here ends the scenic view of the popped tree.
                    scenic_scores.add(
                        lower.0 as usize,
                        lower.1 as usize,
                        lower.0.abs_diff(position.0) as u16 + lower.1.abs_diff(position.1) as u16,
                    );
                }

                seen_higher.push(position);

                position.0 += direction.0 as i32;
                position.1 += direction.1 as i32;
            }

            // The remaining trees in the stack are visible from the other edge:
            for pos in seen_higher.iter() {
                grid_visibility.mark_visible(pos.0 as usize, pos.1 as usize);
                scenic_scores.add(
                    pos.0 as usize,
                    pos.1 as usize,
                    pos.0.abs_diff(other_end.0) as u16 + pos.1.abs_diff(other_end.1) as u16,
                );
            }
        }
    }

    if input.is_part_one() {
        Ok(grid_visibility.num_visible())
    } else {
        Ok(scenic_scores.highest_score())
    }
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    let test_input = "30373
25512
65332
33549
35390";
    test_part_one!(test_input => 21);
    test_part_two!(test_input => 8);

    let real_input = include_str!("day08_input.txt");
    test_part_one!(real_input => 1_672);
    test_part_two!(real_input => 327_180);
}
