use crate::input::Input;
use std::collections::HashSet;
use std::ops::RangeInclusive;

type CoordinateComponent = i8;

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
struct Coordinate {
    x: CoordinateComponent,
    y: CoordinateComponent,
    z: CoordinateComponent,
    w: CoordinateComponent,
}

struct Grid {
    occupied_coordinates: HashSet<Coordinate>,
    checked: HashSet<Coordinate>,
    to_add: Vec<Coordinate>,
    to_remove: Vec<Coordinate>,
}

impl Grid {
    fn parse(input: &str) -> Result<Self, String> {
        let mut occupied_coordinates = HashSet::new();

        for (row, line) in input.lines().enumerate() {
            for (col, b) in line.bytes().enumerate() {
                if b == b'#' {
                    occupied_coordinates.insert(Coordinate {
                        x: col as CoordinateComponent,
                        y: row as CoordinateComponent,
                        z: 0,
                        w: 0,
                    });
                }
            }
        }

        Ok(Self {
            occupied_coordinates,
            checked: HashSet::new(),
            to_add: Vec::new(),
            to_remove: Vec::new(),
        })
    }

    fn active_next_cycle(
        &self,
        is_occupied: bool,
        coordinate: Coordinate,
        w_range: &RangeInclusive<CoordinateComponent>,
    ) -> bool {
        let mut active_neighbors = 0;

        for dx in -1..=1 {
            for dy in -1..=1 {
                for dz in -1..=1 {
                    for dw in w_range.clone() {
                        if !(dx == 0 && dy == 0 && dz == 0 && dw == 0) {
                            let coordinate = Coordinate {
                                x: coordinate.x + dx,
                                y: coordinate.y + dy,
                                z: coordinate.z + dz,
                                w: coordinate.w + dw,
                            };
                            if self.occupied_coordinates.contains(&coordinate) {
                                active_neighbors += 1;
                            }
                        }
                    }
                }
            }
        }

        if is_occupied {
            // - If a cube is active and exactly 2 or 3 of its neighbors are also active,
            // the cube remains active. Otherwise, the cube becomes inactive.
            active_neighbors == 2 || active_neighbors == 3
        } else {
            // - If a cube is inactive but exactly 3 of its neighbors are active, the cube
            // becomes active. Otherwise, the cube remains inactive.
            active_neighbors == 3
        }
    }

    fn cycle(&mut self, w_range: &RangeInclusive<CoordinateComponent>) {
        self.checked.clear();
        self.to_remove.clear();
        self.to_add.clear();

        for coordinate in self.occupied_coordinates.iter() {
            for dx in -1..=1 {
                for dy in -1..=1 {
                    for dz in -1..=1 {
                        for dw in w_range.clone() {
                            let coordinate = Coordinate {
                                x: coordinate.x + dx,
                                y: coordinate.y + dy,
                                z: coordinate.z + dz,
                                w: coordinate.w + dw,
                            };

                            if self.checked.insert(coordinate) {
                                let is_active = self.occupied_coordinates.contains(&coordinate);
                                let will_be_active =
                                    self.active_next_cycle(is_active, coordinate, w_range);

                                if is_active && !will_be_active {
                                    self.to_remove.push(coordinate);
                                } else if will_be_active && !is_active {
                                    self.to_add.push(coordinate);
                                }
                            }
                        }
                    }
                }
            }
        }

        self.occupied_coordinates.extend(self.to_add.iter());
        for remove in self.to_remove.iter() {
            self.occupied_coordinates.remove(remove);
        }
    }
}

pub fn solve(input: &mut Input) -> Result<u64, String> {
    let mut grid = Grid::parse(input.text)?;
    let w_range = input.part_values(0..=0, -1..=1);

    for _ in 0..6 {
        grid.cycle(&w_range);
    }

    Ok(grid.occupied_coordinates.len() as u64)
}

#[test]
pub fn tests() {
    use crate::{test_part_one, test_part_two};

    let example = ".#.\n..#\n###";
    test_part_one!(example => 112);
    test_part_two!(example => 848);

    let real_input = include_str!("day17_input.txt");
    test_part_one!(real_input => 317);
    test_part_two!(real_input => 1692);
}
