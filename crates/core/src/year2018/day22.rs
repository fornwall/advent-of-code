use std::collections::hash_map::Entry;
use std::collections::{BinaryHeap, HashMap, HashSet};

enum RegionType {
    Rocky,
    Narrow,
    Wet,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone)]
enum Equipment {
    Torch,
    ClimbingGear,
    Neither,
}

const fn is_compatible(region_type: RegionType, equipment: Equipment) -> bool {
    // In rocky regions, you can use the climbing gear or the torch. You cannot use neither (you'll likely slip and fall).
    // In wet regions, you can use the climbing gear or neither tool. You cannot use the torch (if it gets wet, you won't have a light source).
    // In narrow regions, you can use the torch or neither tool. You cannot use the climbing gear (it's too bulky to fit).
    matches!(
        (region_type, equipment),
        (RegionType::Rocky, Equipment::ClimbingGear)
            | (RegionType::Rocky, Equipment::Torch)
            | (RegionType::Wet, Equipment::ClimbingGear)
            | (RegionType::Wet, Equipment::Neither)
            | (RegionType::Narrow, Equipment::Torch)
            | (RegionType::Narrow, Equipment::Neither)
    )
}

fn other_equipment(region_type: RegionType, equipment: Equipment) -> Result<Equipment, String> {
    // In rocky regions, you can use the climbing gear or the torch. You cannot use neither (you'll likely slip and fall).
    // In wet regions, you can use the climbing gear or neither tool. You cannot use the torch (if it gets wet, you won't have a light source).
    // In narrow regions, you can use the torch or neither tool. You cannot use the climbing gear (it's too bulky to fit).
    Ok(match (region_type, equipment) {
        (RegionType::Rocky, Equipment::ClimbingGear) | (RegionType::Narrow, Equipment::Neither) => {
            Equipment::Torch
        }
        (RegionType::Rocky, Equipment::Torch) | (RegionType::Wet, Equipment::Neither) => {
            Equipment::ClimbingGear
        }
        (RegionType::Wet, Equipment::ClimbingGear) | (RegionType::Narrow, Equipment::Torch) => {
            Equipment::Neither
        }
        _ => return Err("Invalid region type and equipment pair".to_string()),
    })
}

struct Grid {
    cache: HashMap<(usize, usize), usize>,
    depth: usize,
    target_x: usize,
    target_y: usize,
}

impl Grid {
    fn parse(input_string: &str) -> Result<Self, String> {
        let error_message = |_| "Invalid grid format";
        let lines: Vec<&str> = input_string.lines().collect();
        if lines.len() != 2 {
            return Err("Invalid input - expecting 2 lines".to_string());
        } else if lines[0].len() < 8 {
            return Err("Invalid input - first line is too short".to_string());
        } else if lines[1].len() < 9 {
            return Err("Invalid input - second line is too short".to_string());
        }
        let depth = lines[0][7..].parse::<usize>().map_err(error_message)?;

        let parts: Vec<&str> = lines[1][8..].split(',').collect();
        let target_x = parts[0].parse::<usize>().map_err(error_message)?;
        let target_y = parts[1].parse::<usize>().map_err(error_message)?;

        Ok(Self {
            cache: HashMap::new(),
            depth,
            target_x,
            target_y,
        })
    }

    fn geological_index(&mut self, x: usize, y: usize) -> usize {
        if let Entry::Occupied(entry) = self.cache.entry((x, y)) {
            return *entry.get();
        }
        let value = if (x == 0 && y == 0) || (x == self.target_x && y == self.target_y) {
            // The region at 0,0 (the mouth of the cave) has a geologic index of 0:
            // The region at the coordinates of the target has a geologic index of 0:
            0
        } else if y == 0 {
            // If the region's Y coordinate is 0, the geologic index is its X coordinate times 16807:
            x * 16807
        } else if x == 0 {
            // If the region's X coordinate is 0, the geologic index is its Y coordinate times 48271:
            y * 48271
        } else {
            // Otherwise, the region's geologic index is the result of multiplying the erosion levels of the regions at X-1,Y and X,Y-1:
            self.erosion_level(x - 1, y) * self.erosion_level(x, y - 1)
        };
        self.cache.insert((x, y), value);
        value
    }

    /// A region's erosion level is its geologic index plus the cave system's depth, all modulo 20183.
    fn erosion_level(&mut self, x: usize, y: usize) -> usize {
        (self.geological_index(x, y) + self.depth) % 20183
    }

    fn risk_level(&mut self, x: usize, y: usize) -> usize {
        self.erosion_level(x, y) % 3
    }

    fn region_type(&mut self, x: usize, y: usize) -> Result<RegionType, String> {
        Ok(match self.risk_level(x, y) {
            0 => RegionType::Rocky,
            1 => RegionType::Wet,
            2 => RegionType::Narrow,
            other => {
                return Err(format!("Invalid risk level: {}", other));
            }
        })
    }
}

pub fn part1(input_string: &str) -> Result<usize, String> {
    let mut grid = Grid::parse(input_string)?;
    let mut sum = 0;
    for y in 0..=grid.target_y {
        for x in 0..=grid.target_x {
            sum += grid.risk_level(x, y);
        }
    }
    Ok(sum)
}

pub fn part2(input_string: &str) -> Result<i32, String> {
    let mut grid = Grid::parse(input_string)?;
    let mut to_visit = BinaryHeap::new();
    let mut visited = HashSet::new();

    // (-(cost+heuristic), -cost, x, y, equipment)
    to_visit.push((0, 0, 0, 0, Equipment::Torch));

    let heuristic = |x: i32, y: i32, equipment, g: &Grid| -> i32 {
        (x - g.target_x as i32).abs()
            + (y - g.target_y as i32).abs()
            + if equipment == Equipment::Torch { 0 } else { 7 }
    };

    while let Some(visiting) = to_visit.pop() {
        let cost = -visiting.1;
        let visiting_x = visiting.2 as i32;
        let visiting_y = visiting.3 as i32;
        let equipment = visiting.4;

        if !visited.insert((visiting_x, visiting_y, equipment)) {
            continue;
        }

        if visiting_x == grid.target_x as i32
            && visiting_y == grid.target_y as i32
            && equipment == Equipment::Torch
        {
            return Ok(cost);
        }

        let region_type_visiting = grid.region_type(visiting_x as usize, visiting_y as usize)?;

        let other_equipment = other_equipment(region_type_visiting, equipment)?;
        if !visited.contains(&(visiting_y, visiting_y, other_equipment)) {
            let new_cost = cost + 7;
            let new_heuristic = heuristic(visiting_x, visiting_y, other_equipment, &grid);
            to_visit.push((
                -(new_cost + new_heuristic),
                -new_cost,
                visiting_x,
                visiting_y,
                other_equipment,
            ));
        }

        for (nx, ny) in [(0, -1_i32), (-1_i32, 0), (1, 0), (0, 1)].iter() {
            let new_x = (visiting_x as i32 + *nx) as i32;
            let new_y = (visiting_y as i32 + *ny) as i32;
            if new_x < 0 || new_y < 0 {
                continue;
            }

            let region_type_new = grid.region_type(new_x as usize, new_y as usize)?;
            if is_compatible(region_type_new, equipment) {
                if visited.contains(&(new_x, new_y, equipment)) {
                    continue;
                }
                let new_cost = cost + 1;
                let new_heuristic = heuristic(new_x, new_y, equipment, &grid);
                to_visit.push((
                    -(new_cost + new_heuristic),
                    -new_cost,
                    new_x,
                    new_y,
                    equipment,
                ));
            }
        }
    }

    Err("No solution found".to_string())
}

#[test]
fn tests_part1() {
    assert_eq!(
        Ok(114),
        part1(
            "depth: 510
target: 10,10"
        )
    );

    assert_eq!(Ok(11843), part1(include_str!("day22_input.txt")));
}

#[test]
fn tests_part2() {
    assert_eq!(
        Ok(45),
        part2(
            "depth: 510
target: 10,10"
        )
    );

    assert_eq!(Ok(1078), part2(include_str!("day22_input.txt")));
}
