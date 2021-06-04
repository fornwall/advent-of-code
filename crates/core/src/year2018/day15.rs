use crate::input::Input;
use std::collections::VecDeque;
use std::env;

#[derive(Copy, Clone)]
enum MapCell {
    Wall,
    Open,
    Unit {
        hit_points: i32,
        elf: bool,
        even: bool,
    },
}

struct Board {
    width: u32,
    height: u32,
    cells: Vec<MapCell>,
    visited: Vec<bool>,
    round: i32,
    full_round: bool,
    elves_alive: i32,
    elf_died: bool,
    goblins_alive: i32,
    elf_attack_power: i32,
}

impl Board {
    fn parse(input_string: &str, elf_attack_power: i32) -> Result<Self, String> {
        let width = input_string
            .find('\n')
            .ok_or_else(|| "No line in input".to_string())? as u32;

        let mut elves_alive = 0;
        let mut goblins_alive = 0;
        let mut cells = Vec::new();
        let mut height = 0;

        for line in input_string.lines() {
            height += 1;
            if width as usize != line.len() {
                return Err("Not all lines are of equal length".to_string());
            }
            for c in line.chars() {
                cells.push(match c {
                    '#' => MapCell::Wall,
                    '.' => MapCell::Open,
                    'G' | 'E' => {
                        let elf = c == 'E';
                        if elf {
                            elves_alive += 1;
                        } else {
                            goblins_alive += 1;
                        }
                        MapCell::Unit {
                            hit_points: 200,
                            elf,
                            even: false,
                        }
                    }
                    _ => {
                        return Err(format!("Unrecognized cell: {}", c));
                    }
                });
            }
        }

        Ok(Self {
            width,
            height,
            cells,
            visited: vec![false; (width * height) as usize],
            round: 0,
            full_round: false,
            elves_alive,
            elf_died: false,
            goblins_alive,
            elf_attack_power,
        })
    }

    fn at(&mut self, x: u32, y: u32) -> &mut MapCell {
        &mut self.cells[(x + self.width * y) as usize]
    }

    fn put(&mut self, x: u32, y: u32, value: MapCell) {
        self.cells[(x + self.width * y) as usize] = value;
    }

    fn calculate_outcome(&self) -> Option<i32> {
        if self.elves_alive != 0 && self.goblins_alive != 0 {
            return None;
        }

        let hit_point_sum = self.cells.iter().fold(0, |acc, cell| {
            acc + if let MapCell::Unit { hit_points, .. } = *cell {
                hit_points
            } else {
                0
            }
        });
        let round_for_score = self.round - if self.full_round { 0 } else { 1 };
        Some(hit_point_sum * round_for_score)
    }

    fn perform_round(&mut self) {
        self.round += 1;
        self.full_round = true;
        let even_round = self.round % 2 == 0;

        for y in 0..self.height {
            for x in 0..self.width {
                if let MapCell::Unit { even, elf, .. } = *self.at(x, y) {
                    if even == even_round {
                        self.attack_or_move_towards(x, y, !elf);
                    }
                }
            }
        }
    }

    fn attack(&mut self, x: u32, y: u32, elf_target: bool) -> bool {
        let mut lowest_hit_points = std::i32::MAX;
        let mut target_position = (0, 0);

        for (dx, dy) in [(0, -1_i32), (-1_i32, 0), (1, 0), (0, 1)].iter() {
            let (target_x, target_y) = (x as i32 + *dx, y as i32 + *dy);
            if let MapCell::Unit {
                hit_points, elf, ..
            } = self.at(target_x as u32, target_y as u32)
            {
                if *elf == elf_target && *hit_points < lowest_hit_points {
                    lowest_hit_points = *hit_points;
                    target_position = (target_x, target_y);
                }
            }
        }

        if lowest_hit_points != std::i32::MAX {
            let attack_damage = if elf_target { 3 } else { self.elf_attack_power };

            if let MapCell::Unit { hit_points, .. } =
                self.at(target_position.0 as u32, target_position.1 as u32)
            {
                *hit_points -= attack_damage;
                if *hit_points <= 0 {
                    self.put(
                        target_position.0 as u32,
                        target_position.1 as u32,
                        MapCell::Open,
                    );
                    if elf_target {
                        self.elves_alive -= 1;
                        self.elf_died = true;
                    } else {
                        self.goblins_alive -= 1;
                    }
                }
                return true;
            }
        }

        false
    }

    fn attack_or_move_towards(&mut self, x: u32, y: u32, elf_target: bool) {
        if self.elves_alive == 0 || self.goblins_alive == 0 {
            self.full_round = false;
            return;
        }

        if let MapCell::Unit { ref mut even, .. } = self.at(x, y) {
            *even = !*even;
        }

        // Attack.
        if self.attack(x, y, elf_target) {
            return;
        }

        // Move.
        if let Some((nx, ny)) = self.shortest_distance(x, y, elf_target) {
            let cell_value = *self.at(x, y);
            self.put(nx, ny, cell_value);
            self.put(x, y, MapCell::Open);
            // Attack from new position if possible.
            self.attack(nx, ny, elf_target);
            return;
        }
    }

    fn shortest_distance(&mut self, sx: u32, sy: u32, elf_target: bool) -> Option<(u32, u32)> {
        let mut to_visit = VecDeque::new();
        to_visit.push_back((0_i32, sx, sy, 0, 0));

        self.visited.iter_mut().for_each(|element| *element = false);
        self.visited[(sx + self.width * sy) as usize] = true;

        let mut found: Vec<(i32, u32, u32, u32, u32)> = Vec::new();
        let mut found_cost = -1;

        while let Some(visiting) = to_visit.pop_front() {
            let (cost, visiting_x, visiting_y) =
                (visiting.0 + 1, visiting.1 as u32, visiting.2 as u32);

            if found_cost != -1 && found_cost != cost {
                break;
            }

            for (nx, ny) in [(0, -1_i32), (-1_i32, 0), (1, 0), (0, 1)].iter() {
                let x = (visiting_x as i32 + *nx) as u32;
                let y = (visiting_y as i32 + *ny) as u32;

                match self.at(x, y) {
                    MapCell::Unit { elf, .. } if *elf == elf_target => {
                        found.push((cost, visiting_x, visiting_y, visiting.3, visiting.4));
                        found_cost = cost;
                    }
                    MapCell::Open => {
                        if !self.visited[(x + y * self.height) as usize] {
                            self.visited[(x + y * self.height) as usize] = true;
                            let first_x: u32;
                            let first_y: u32;
                            if visiting_x == sx && visiting_y == sy {
                                // Initial step.
                                first_x = x;
                                first_y = y;
                            } else {
                                // Propagate initial step.
                                first_x = visiting.3;
                                first_y = visiting.4;
                            };

                            to_visit.push_back((cost, x, y, first_x, first_y));
                        };
                    }
                    _ => {}
                }
            }
        }

        if found.is_empty() {
            None
        } else {
            found.sort_by(|a, b| {
                a.0.cmp(&b.0)
                    .then(a.2.cmp(&b.2))
                    .then(a.1.cmp(&b.1))
                    .then(a.4.cmp(&b.4).then(a.3.cmp(&b.3)))
            });
            Some((found[0].3, found[0].4))
        }
    }

    fn print(&mut self) {
        if env::var("ADVENT_DEBUG").is_err() {
            return;
        }

        println!("Round {}", self.round);
        for y in 0..self.height {
            for x in 0..self.width {
                let cell = self.at(x, y);
                let c = match *cell {
                    MapCell::Open => ".",
                    MapCell::Wall => "#",
                    MapCell::Unit { elf: false, .. } => "G",
                    MapCell::Unit { elf: true, .. } => "E",
                };
                print!("{}", c);
            }
            print!("   ");
            for x in 0..self.width {
                let cell = self.at(x, y);
                if let MapCell::Unit {
                    hit_points,
                    elf: true,
                    ..
                } = cell
                {
                    print!("E({}), ", hit_points);
                } else if let MapCell::Unit {
                    hit_points,
                    elf: false,
                    ..
                } = cell
                {
                    print!("G({}), ", hit_points);
                }
            }
            println!();
        }
        println!();
    }
}

pub fn solve(input: &mut Input) -> Result<i32, String> {
    let mut attack_strength = input.part_values(3, 4);
    loop {
        let mut board = Board::parse(input.text, attack_strength)?;
        board.print();
        loop {
            if board.round > 500 {
                return Err("No solution found".to_string());
            }
            board.perform_round();
            board.print();
            if input.is_part_two() && board.elf_died {
                break;
            } else if let Some(outcome) = board.calculate_outcome() {
                return Ok(outcome);
            }
        }

        attack_strength += 1;
    }
}

#[test]
fn tests() {
    use crate::{test_part_one, test_part_two};

    test_part_one!(
            "#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######" => 27730
    );

    test_part_one!(
            "#######
#G..#E#
#E#E.E#
#G.##.#
#...#E#
#...E.#
#######" => 36334
    );

    test_part_one!(
            "#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######" => 39514
    );

    test_part_one!(
            "#######
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######"=> 27755
    );

    test_part_one!(
            "#######
#.E...#
#.#..G#
#.###.#
#E#G#G#
#...#G#
#######"
        =>28944
    );

    test_part_one!(
            "#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########"
        =>18740
    );

    let input = include_str!("day15_input.txt");
    test_part_one!(input => 207_059);

    test_part_two!(
            "#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######" =>4988
    );

    test_part_two!(
            "#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######" =>31284
    );

    test_part_two!(input => 49120);
}
