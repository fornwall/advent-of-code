use crate::common::permutation::all_permutations;
use crate::Input;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};

struct Grid {
    cols: usize,
    data: Vec<bool>,
    locations: Vec<(usize, usize)>,
}

impl Grid {
    fn parse(input: &str) -> Result<Self, String> {
        let rows = input.lines().count();
        let cols = input.lines().next().ok_or("Empty input")?.len();
        let mut locations = Vec::new();
        let mut data = vec![true; rows * cols];

        for (y, line) in input.lines().enumerate() {
            if line.len() != cols {
                return Err("Not all rows have equal length".into());
            }
            for (x, &c) in line.as_bytes().iter().enumerate() {
                data[y * cols + x] = match c {
                    b'#' => false,
                    b'.' => true,
                    b'0'..=b'7' => {
                        if x == 0 || y == 0 || x + 1 == cols || y + 1 == rows {
                            return Err("Number at edge".into());
                        }
                        let number = (c - b'0') as usize;
                        if number >= locations.len() {
                            locations.resize(number + 1, (0, 0));
                        }
                        locations[number] = (x, y);
                        true
                    }
                    _ => {
                        return Err(format!("Invalid char in input: '{}'", c as char));
                    }
                };
            }
        }

        Ok(Self {
            cols,
            data,
            locations,
        })
    }

    fn is_open(&self, location: (usize, usize)) -> bool {
        self.data[location.1 * self.cols + location.0]
    }
}

pub fn solve(input: &mut Input) -> Result<usize, String> {
    let grid = Grid::parse(input.text)?;
    let mut distances: HashMap<(usize, usize), usize> = HashMap::new();

    for from in 0..grid.locations.len() {
        'toloop: for to in 0..grid.locations.len() {
            if to <= from {
                continue;
            }

            let starting_location = grid.locations[from];
            let target_location = grid.locations[to];
            if starting_location == (0, 0) || target_location == (0, 0) {
                return Err("Not all digits in grid".into());
            }

            let mut visited = HashSet::new();
            let mut to_visit = BinaryHeap::new();
            to_visit.push(Reverse((0, 0, starting_location)));

            while let Some(Reverse((_heuristic, distance, location))) = to_visit.pop() {
                if location == target_location {
                    distances.insert((from, to), distance);
                    continue 'toloop;
                }

                for diff in &[(1_i32, 0_i32), (-1, 0), (0, 1), (0, -1)] {
                    if diff.0 == -1 && location.0 == 0 || diff.1 == -1 && location.1 == 0 {
                        continue;
                    }
                    let new_location = (
                        (location.0 as i32 + diff.0) as usize,
                        (location.1 as i32 + diff.1) as usize,
                    );
                    if grid.is_open(new_location) && visited.insert(new_location) {
                        let new_distance = distance + 1;
                        let heuristic = (new_location.0 as i32 - target_location.0 as i32).abs()
                            as usize
                            + (new_location.1 as i32 - target_location.1 as i32).abs() as usize;
                        to_visit.push(Reverse((
                            new_distance + heuristic,
                            new_distance,
                            new_location,
                        )));
                    }
                }
            }
        }
    }

    let mut initial_order = (1..grid.locations.len()).collect::<Vec<usize>>();
    let mut answer = usize::MAX;
    all_permutations(
        &mut initial_order,
        &mut |order: &[usize]| -> Result<(), String> {
            let mut current_location = 0_usize;
            let mut total_distance = 0;
            for &n in order.iter() {
                let key = (
                    std::cmp::min(current_location, n),
                    std::cmp::max(current_location, n),
                );
                total_distance += distances.get(&key).unwrap();
                current_location = n;
            }
            if input.is_part_two() {
                total_distance += distances.get(&(0, current_location)).unwrap();
            }
            answer = std::cmp::min(answer, total_distance);
            Ok(())
        },
    )?;

    Ok(answer)
}

#[test]
pub fn tests() {
    use crate::{test_part_one, test_part_two};

    test_part_one!("###########\n#0.1.....2#\n#.#######.#\n#4.......3#\n###########" => 14);

    let real_input = include_str!("day24_input.txt");
    test_part_one!(real_input => 412);
    test_part_two!(real_input => 664);
}
