#[cfg(feature = "visualization")]
use super::day03_renderer::render;
use crate::input::Input;

pub struct Map {
    pub rows: usize,
    pub cols: usize,
    grid: Vec<bool>,
}

impl Map {
    fn parse(input: &str) -> Result<Self, String> {
        let rows = input.lines().count();
        let cols = input.lines().next().ok_or("Empty input")?.len();
        let grid: Vec<bool> = input
            .bytes()
            .filter_map(|c| if c == b'\n' { None } else { Some(c == b'#') })
            .collect();

        if grid.len() != cols * rows {
            return Err("Not all rows have equal length".into());
        } else if rows <= 2 || cols <= 2 {
            return Err("Too small grid".into());
        }

        Ok(Self { rows, cols, grid })
    }

    pub fn tree_at(&self, x: usize, y: usize) -> bool {
        // The map repeats to the right many times:
        let x = x % self.cols;
        self.grid[x + y * self.cols]
    }
}

pub fn solve(input: &mut Input) -> Result<usize, String> {
    let map = Map::parse(input.text)?;
    let slopes = input.part_values(vec![(3, 1)], vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]);

    #[cfg(feature = "visualization")]
    render(&map, &slopes, &mut input.painter);

    Ok(slopes.iter().fold(1, |acc, slope| {
        let initial_position = (0, 0);

        let trees_seen_now = std::iter::successors(Some(initial_position), |pos| {
            let new_pos = (pos.0 + slope.0, pos.1 + slope.1);
            if new_pos.1 < map.rows {
                Some(new_pos)
            } else {
                None
            }
        })
        .map(|(x, y)| map.tree_at(x, y))
        .filter(|tree| *tree)
        .count();

        acc * trees_seen_now
    }))
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_one_error, test_part_two};

    test_part_one!("..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#" => 7);

    test_part_one_error!("#.\n.\n" => "Not all rows have equal length");

    let real_input = include_str!("day03_input.txt");
    test_part_one!(real_input => 286);
    test_part_two!(real_input => 3_638_606_400);

    let real_input = include_str!("day03_input_2.txt");
    test_part_one!(real_input => 169);
    test_part_two!(real_input => 7_560_370_818);
}
