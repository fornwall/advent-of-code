use std::collections::VecDeque;
use std::env;

#[derive(Debug, Copy, Clone)]
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
        let width = match input_string.find('\n') {
            Some(len) => len as u32,
            None => {
                return Err("No line in input".to_string());
            }
        };

        let mut elves_alive = 0;
        let mut goblins_alive = 0;
        let mut cells = Vec::new();
        let mut height = 0;
        for line in input_string.lines() {
            height += 1;
            assert_eq!(width, line.len() as u32);
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
        if self.round > 200 {
            return Some(1337);
        }
        let mut elf_alive = false;
        let mut goblin_alive = false;
        let mut hit_point_sum = 0;

        for cell in self.cells.iter() {
            if let MapCell::Unit {
                hit_points, elf, ..
            } = *cell
            {
                hit_point_sum += hit_points;
                if elf {
                    elf_alive = true;
                } else {
                    goblin_alive = true;
                }
            }
        }
        if goblin_alive && elf_alive {
            None
        } else {
            let round_for_score = self.round - if self.full_round { 0 } else { 1 };
            Some(hit_point_sum * round_for_score)
        }
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
        let (closest_distance, nx, ny) = self.shortest_distance(x, y, elf_target);

        if closest_distance != std::u32::MAX {
            let cell_value = *self.at(x, y);
            self.put(nx, ny, cell_value);
            self.put(x, y, MapCell::Open);
            // Attack from new position if possible.
            self.attack(nx, ny, elf_target);
            return;
        }
    }

    fn shortest_distance(&mut self, sx: u32, sy: u32, elf_target: bool) -> (u32, u32, u32) {
        let mut to_visit = VecDeque::new();
        to_visit.push_back((0_i32, sx, sy, 0, 0));

        self.visited.iter_mut().for_each(|element| *element = false);
        self.visited[(sx + self.width * sy) as usize] = true;

        while let Some(visiting) = to_visit.pop_front() {
            let (cost, visiting_x, visiting_y) =
                (visiting.0 + 1, visiting.1 as u32, visiting.2 as u32);

            for (nx, ny) in [(0, -1_i32), (-1_i32, 0), (1, 0), (0, 1)].iter() {
                let x = (visiting_x as i32 + *nx) as u32;
                let y = (visiting_y as i32 + *ny) as u32;

                match self.at(x, y) {
                    MapCell::Unit { elf, .. } if *elf == elf_target => {
                        return (cost as u32, visiting.3, visiting.4);
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

        (std::u32::MAX, 0, 0)
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

pub fn part1(input_string: &str) -> Result<i32, String> {
    let mut board = Board::parse(input_string, 3)?;
    board.print();
    loop {
        board.perform_round();
        board.print();
        if let Some(outcome) = board.calculate_outcome() {
            return Ok(outcome);
        }
    }
}

pub fn part2(input_string: &str) -> Result<i32, String> {
    let mut attack_strength = 4;
    loop {
        let mut board = Board::parse(input_string, attack_strength)?;
        board.print();
        loop {
            board.perform_round();
            board.print();
            if board.elf_died {
                break;
            } else if board.goblins_alive == 0 {
                return Ok(board.calculate_outcome().unwrap());
            }
        }

        attack_strength += 1;
    }
}

#[test]
fn tests_part1() {
    assert_eq!(
        Ok(27730),
        part1(
            "#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######"
        )
    );

    assert_eq!(
        Ok(36334),
        part1(
            "#######
#G..#E#
#E#E.E#
#G.##.#
#...#E#
#...E.#
#######"
        )
    );

    assert_eq!(
        Ok(39514),
        part1(
            "#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######"
        )
    );

    assert_eq!(
        Ok(27755),
        part1(
            "#######
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######"
        )
    );

    assert_eq!(
        Ok(28944),
        part1(
            "#######
#.E...#
#.#..G#
#.###.#
#E#G#G#
#...#G#
#######"
        )
    );

    assert_eq!(
        Ok(18740),
        part1(
            "#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########"
        )
    );

    assert_eq!(Ok(207_059), part1(include_str!("day15_input.txt")));
}

#[test]
fn tests_part2() {
    assert_eq!(
        Ok(4988),
        part2(
            "#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######"
        )
    );

    assert_eq!(
        Ok(31284),
        part2(
            "#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######"
        )
    );

    assert_eq!(Ok(49120), part2(include_str!("day15_input.txt")));
}
