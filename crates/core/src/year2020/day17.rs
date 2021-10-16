use crate::input::Input;
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};

type CoordinateComponent = i8;

type Coordinate = (
    CoordinateComponent,
    CoordinateComponent,
    CoordinateComponent,
    CoordinateComponent,
);

struct Grid {
    occupied_coordinates: HashSet<Coordinate>,
    active_neighbors_count: HashMap<Coordinate, u8>,
}

impl Grid {
    fn parse(input: &str) -> Result<Self, String> {
        let mut occupied_coordinates = HashSet::with_capacity(2000);

        for (row, line) in input.lines().enumerate() {
            if line.len() > 8 || row > 7 {
                return Err("Bigger than 8x8 input".into());
            }
            for (col, b) in line.bytes().enumerate() {
                if b == b'#' {
                    occupied_coordinates.insert((
                        col as CoordinateComponent,
                        row as CoordinateComponent,
                        0,
                        0,
                    ));
                }
            }
        }

        Ok(Self {
            occupied_coordinates,
            active_neighbors_count: HashMap::with_capacity(25000),
        })
    }

    fn cycle(&mut self, w_range: CoordinateComponent) {
        self.active_neighbors_count.clear();

        for coordinate in self.occupied_coordinates.iter() {
            for dx in -1..=1 {
                for dy in -1..=1 {
                    for dz in -1..=1 {
                        for dw in -w_range..=w_range {
                            let coordinate = (
                                coordinate.0 + dx,
                                coordinate.1 + dy,
                                coordinate.2 + dz,
                                coordinate.3 + dw,
                            );

                            let value_to_add = (dx | dy | dz | dw).abs() as u8;

                            match self.active_neighbors_count.entry(coordinate) {
                                Entry::Occupied(mut entry) => {
                                    entry.insert(entry.get() + value_to_add);
                                }
                                Entry::Vacant(entry) => {
                                    entry.insert(value_to_add);
                                }
                            }
                        }
                    }
                }
            }
        }

        for (&coordinate, &active_neighbors) in self.active_neighbors_count.iter() {
            let is_active = self.occupied_coordinates.contains(&coordinate);

            let will_be_active = if is_active {
                // "If a cube is active and exactly 2 or 3 of its neighbors are also active,
                // the cube remains active. Otherwise, the cube becomes inactive."
                active_neighbors == 2 || active_neighbors == 3
            } else {
                // "If a cube is inactive but exactly 3 of its neighbors are active, the cube
                // becomes active. Otherwise, the cube remains inactive."
                active_neighbors == 3
            };

            if is_active && !will_be_active {
                self.occupied_coordinates.remove(&coordinate);
            } else if !is_active && will_be_active {
                self.occupied_coordinates.insert(coordinate);
            }
        }
    }
}

pub fn solve(input: &mut Input) -> Result<u64, String> {
    let mut grid = Grid::parse(input.text)?;
    let w_range = input.part_values(0, 1);

    for _ in 0..6 {
        grid.cycle(w_range);
    }

    Ok(grid.occupied_coordinates.len() as u64)
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    let example = ".#.\n..#\n###";
    test_part_one!(example => 112);
    test_part_two!(example => 848);

    let real_input = include_str!("day17_input.txt");
    test_part_one!(real_input => 317);
    test_part_two!(real_input => 1692);
}
