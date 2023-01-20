#[cfg(feature = "visualization")]
use svgplot::{SvgColor, SvgImage, SvgPath, SvgScript};

use crate::input::Input;

pub fn solve(input: &Input) -> Result<usize, String> {
    const DIRECTIONS: [(i16, i16); 8] = [
        // NW, N, NE
        (-1, 1),
        (0, 1),
        (1, 1),
        // E, SE
        (1, 0),
        (1, -1),
        // S, SW
        (0, -1),
        (-1, -1),
        // W
        (-1, 0),
    ];

    const RULES: [(i16, (i16, i16)); 4] = [
        // "If there is no elf in the n, ne, or nw adjacent positions, the elf proposes moving north one step."
        (0b0000_0111, (0, 1)),
        // "if there is no elf in the s, se, or sw adjacent positions, the elf proposes moving south one step."
        (0b0111_0000, (0, -1)),
        // "if there is no elf in the w, nw, or sw adjacent positions, the elf proposes moving west one step."
        (0b1100_0001, (-1, 0)),
        // "if there is no elf in the e, ne, or se adjacent positions, the elf proposes moving east one step."
        (0b0001_1100, (1, 0)),
    ];

    const MAX_SIZE: usize = 384;
    const MAX_ELVES: usize = 10_000;
    const OFFSET: usize = MAX_SIZE / 2;
    const NO_CHOICE: (i16, i16) = (i16::MAX, i16::MAX);
    const NO_ELF: u16 = u16::MAX;

    let is_outside_max = |position: (i16, i16)| {
        position.0 < 0
            || position.1 < 0
            || position.0 >= MAX_SIZE as i16
            || position.1 >= MAX_SIZE as i16
    };

    let mut elves = input
        .text
        .lines()
        .rev()
        .enumerate()
        .flat_map(|(y, line)| {
            line.bytes().enumerate().filter_map(move |(x, c)| {
                (c == b'#').then_some(Elf {
                    position: (x as i16 + OFFSET as i16, y as i16 + OFFSET as i16),
                    to_move_choice: NO_CHOICE,
                })
            })
        })
        .collect::<Vec<_>>();

    if elves.len() > MAX_ELVES {
        return Err(format!("Too many elves - max {MAX_ELVES} supported"));
    }

    let mut elf_grid = vec![NO_ELF; MAX_SIZE * MAX_SIZE];
    for (elf_idx, elf) in elves.iter().enumerate() {
        if is_outside_max(elf.position) {
            return Err(format!("Elf is outside of [0,{MAX_SIZE})"));
        }
        elf_grid[elf.position.1 as usize * MAX_SIZE + elf.position.0 as usize] = elf_idx as u16;
    }

    #[cfg(feature = "visualization")]
    let mut max_coords = (0, 0);
    #[cfg(feature = "visualization")]
    let mut min_coords = (i16::MAX, i16::MAX);
    #[cfg(feature = "visualization")]
    let mut svg = SvgImage::new();
    #[cfg(feature = "visualization")]
        let mut stable_elf_positions = String::from("const stableElfPositions = [");
    #[cfg(feature = "visualization")]
    let mut elf_positions = String::from("const elfPositions = [");
    #[cfg(feature = "visualization")]
    {
        elf_positions.push('[');
        for (idx, elf) in elves.iter().enumerate() {
            if idx > 0 {
                elf_positions.push(',');
            }
            min_coords.0 = elf.position.0.min(min_coords.0);
            min_coords.1 = elf.position.0.min(min_coords.1);
            max_coords.0 = elf.position.0.max(max_coords.0);
            max_coords.1 = elf.position.0.max(max_coords.1);
            elf_positions.push_str(&format!("[{},{}]", elf.position.0, elf.position.1));
        }
        elf_positions.push(']');
        stable_elf_positions.push_str("[]");
    }

    for round in 0..input.part_values(10, 10000) {
        let mut num_moves = 0;

        #[cfg(feature = "visualization")]
        {
            stable_elf_positions.push_str(",[");
            elf_positions.push_str(",[");
        }

        for elf in elves.iter_mut() {
            let adjacent_bitmask = DIRECTIONS
                .iter()
                .enumerate()
                .fold(0, |acc, (idx, (dx, dy))| {
                    acc | if elf_grid
                        [(elf.position.1 + dy) as usize * MAX_SIZE + (elf.position.0 + dx) as usize]
                        == NO_ELF
                    {
                        0
                    } else {
                        1 << idx
                    }
                });

            // "During the first half of each round, each Elf considers the eight positions adjacent to themself.
            // If no other Elves are in one of those eight positions, the Elf does not do anything during this round."
            if adjacent_bitmask != 0 {
                for rule_offset in 0..RULES.len() {
                    let (check_mask, to_move) = RULES[(round + rule_offset) % RULES.len()];
                    if (check_mask & adjacent_bitmask) == 0 {
                        elf.to_move_choice = to_move;
                        break;
                    }
                }
            }
        }

        #[cfg(feature = "visualization")]
        let mut first_elf = true;

        for elf_idx in 0..elves.len() {
            let elf = &mut elves[elf_idx];
            if elf.to_move_choice != NO_CHOICE {
                let to_move = elf.to_move_choice;
                elf.to_move_choice = NO_CHOICE;

                let new_position = (elf.position.0 + to_move.0, elf.position.1 + to_move.1);
                if is_outside_max(new_position) {
                    return Err(format!(
                        "Elf tried to moved outside of [0,{}): {:?}",
                        MAX_SIZE, new_position
                    ));
                }

                let elf_idx_at_position =
                    elf_grid[new_position.1 as usize * MAX_SIZE + new_position.0 as usize];

                if elf_idx_at_position == NO_ELF {
                    elf_grid[elf.position.1 as usize * MAX_SIZE + elf.position.0 as usize] = NO_ELF;
                    elf.position = new_position;
                    elf_grid[elf.position.1 as usize * MAX_SIZE + elf.position.0 as usize] =
                        elf_idx as u16;
                    num_moves += 1;
                } else {
                    // Position was occupied - stand still and push other elf (which must be coming from other direction) back:
                    elf_grid[new_position.1 as usize * MAX_SIZE + new_position.0 as usize] = NO_ELF;
                    let pushed_back_position =
                        (new_position.0 + to_move.0, new_position.1 + to_move.1);
                    elves[elf_idx_at_position as usize].position = pushed_back_position;
                    elf_grid[pushed_back_position.1 as usize * MAX_SIZE
                        + pushed_back_position.0 as usize] = elf_idx_at_position;
                    num_moves -= 1;
                }
            }
            #[cfg(feature = "visualization")]
            {
                if first_elf {
                    first_elf = false;
                } else {
                    elf_positions.push(',');
                }
                let elf = elves[elf_idx];
                min_coords.0 = elf.position.0.min(min_coords.0);
                min_coords.1 = elf.position.1.min(min_coords.1);
                max_coords.0 = elf.position.0.max(max_coords.0);
                max_coords.1 = elf.position.1.max(max_coords.1);
                elf_positions.push_str(&format!("[{},{}]", elf.position.0, elf.position.1));
            }
        }

        #[cfg(feature = "visualization")]
        {
            elf_positions.push(']');
            stable_elf_positions.push(']');
        }

        #[cfg(feature = "visualization")]
        if num_moves == 0 || (input.is_part_one() && round == 9) {
            let new_elves_id = svg.add_with_id(SvgPath::default().fill(SvgColor::Rgb(0xff, 0, 0)));
            elf_positions.push_str("];");
            stable_elf_positions.push_str("];");
            elf_positions.push_str(&format!(
                "window.onNewStep = (step) => {{\n\
                        const pathData = elfPositions[step].map((e) => \n\
                               `M ${{e[0]}} ${{e[1]}} l 1 0 l 0 1 l-1 0 Z`\
                            ).join(' ');\n\
                        document.getElementById(\"{}\").setAttribute('d', pathData);\n\
                }};",
                new_elves_id
            ));
            svg.add(SvgScript::new(elf_positions));
            input.rendered_svg.replace(
                svg.view_box((
                    min_coords.0 as i64,
                    min_coords.1 as i64,
                    (max_coords.0 - min_coords.0) as i64,
                    (max_coords.1 - min_coords.1) as i64,
                ))
                .data_attribute("steps".to_string(), format!("{}", round + 1))
                .to_svg_string(),
            );
            return Ok(0);
        }

        if num_moves == 0 {
            return Ok(round + 1);
        }
    }

    let (min_x, max_x, min_y, max_y) =
        elves
            .iter()
            .fold((i16::MAX, i16::MIN, i16::MAX, i16::MIN), |acc, e| {
                (
                    acc.0.min(e.position.0),
                    acc.1.max(e.position.0),
                    acc.2.min(e.position.1),
                    acc.3.max(e.position.1),
                )
            });
    let rectangle_size = ((max_x + 1 - min_x) * (max_y + 1 - min_y)) as usize;
    #[cfg(feature = "visualization")]
    input.rendered_svg.replace(
        svg.data_attribute("steps".to_string(), "10".to_string())
            .to_svg_string(),
    );
    Ok(rectangle_size - elves.len())
}

#[derive(Copy, Clone)]
struct Elf {
    position: (i16, i16),
    to_move_choice: (i16, i16),
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    let test_input = "....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..";
    test_part_one!(test_input => 110);
    test_part_two!(test_input => 20);

    let real_input = include_str!("day23_input.txt");
    test_part_one!(real_input => 3920);
    test_part_two!(real_input => 889);
}
