#[cfg(feature = "simd")]
use std::simd::{SimdPartialEq, ToBitMask};

#[cfg(feature = "visualization")]
use svgplot::{Coordinate, SvgImage, SvgRect, SvgScript, SvgStyle};

use crate::input::Input;

pub fn solve(input: &Input) -> Result<usize, String> {
    #[cfg(not(feature = "simd"))]
    const DIRECTIONS: [(i16, i16); 8] = [
        // NW
        (-1, -1),
        // N
        (0, -1),
        // NE
        (1, -1),
        // W
        (-1, 0),
        // E
        (1, 0),
        // SW
        (-1, 1),
        // S
        (0, 1),
        // SE
        (1, 1),
    ];

    #[cfg(feature = "simd")]
    const NEIGHBOR_OFFSETS: [i32; 8] = [
        // NW:
        -(MAX_SIZE as i32) - 1,
        // N:
        -(MAX_SIZE as i32),
        // NE:
        -(MAX_SIZE as i32) + 1,
        // W:
        -1,
        // E:
        1,
        // SW:
        (MAX_SIZE as i32) - 1,
        // S:
        MAX_SIZE as i32,
        // SE:
        (MAX_SIZE as i32) + 1,
    ];

    const RULES: [(u8, (i16, i16)); 4] = [
        // "If there is no elf in the n, ne, or nw adjacent positions, the elf proposes moving north one step"
        (0b0000_0111, (0, -1)),
        // "if there is no elf in the s, se, or sw adjacent positions, the elf proposes moving south one step"
        (0b1110_0000, (0, 1)),
        // "if there is no elf in the w, nw, or sw adjacent positions, the elf proposes moving west one step"
        (0b0010_1001, (-1, 0)),
        // "if there is no elf in the e, ne, or se adjacent positions, the elf proposes moving east one step"
        (0b1001_0100, (1, 0)),
    ];

    #[cfg(feature = "simd")]
    const NEIGHBORS_MASK: [u8; 7] = [
        0b0000_0111,
        0b1110_0000,
        0b0010_1001,
        0b1001_0100,
        0b0000_0111,
        0b1110_0000,
        0b0010_1001,
    ];

    #[cfg(feature = "simd")]
    const MOVEMENTS: [(i16, i16); 7] = [(0, -1), (0, 1), (-1, 0), (1, 0), (0, -1), (0, 1), (-1, 0)];

    #[cfg(feature = "simd")]
    let rules_for_mask: [(std::simd::mask16x8, (i16, i16)); 4] = [
        // "If there is no elf in the n, ne, or nw adjacent positions, the elf proposes moving north one step"
        (std::simd::mask16x8::from_bitmask(0b0000_0111), (0, -1)),
        // "if there is no elf in the s, se, or sw adjacent positions, the elf proposes moving south one step"
        (std::simd::mask16x8::from_bitmask(0b1110_0000), (0, 1)),
        // "if there is no elf in the w, nw, or sw adjacent positions, the elf proposes moving west one step"
        (std::simd::mask16x8::from_bitmask(0b0010_1001), (-1, 0)),
        // "if there is no elf in the e, ne, or se adjacent positions, the elf proposes moving east one step"
        (std::simd::mask16x8::from_bitmask(0b1001_0100), (1, 0)),
    ];

    const MAX_SIZE: usize = 256;
    const MAX_ELVES: usize = 10_000;
    const OFFSET: usize = MAX_SIZE / 2;
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
        .enumerate()
        .flat_map(|(y, line)| {
            line.bytes().enumerate().filter_map(move |(x, c)| {
                (c == b'#').then_some((x as i16 + OFFSET as i16, y as i16 + OFFSET as i16))
            })
        })
        .collect::<Vec<_>>();

    if elves.len() > MAX_ELVES {
        return Err(format!("Too many elves - max {MAX_ELVES} supported"));
    }

    let mut elf_grid = vec![NO_ELF; MAX_SIZE * MAX_SIZE];
    for (elf_idx, &elf) in elves.iter().enumerate() {
        if is_outside_max(elf) {
            return Err(format!("Elf is outside of [0,{MAX_SIZE})"));
        }
        elf_grid[elf.1 as usize * MAX_SIZE + elf.0 as usize] = elf_idx as u16;
    }

    let mut elf_moves = Vec::with_capacity(elves.len());

    #[cfg(feature = "visualization")]
    let mut elf_positions_per_step = vec![elves.clone()];

    #[cfg(feature = "simd")]
    let no_elf_vector = std::simd::u16x8::splat(NO_ELF);

    for round in 0..input.part_values(10, 10000) {
        let mut num_moves = 0;

        #[cfg(feature = "simd")]
        {
            //var directionOffset = (round - 1) % 4;
            //var masksVector = IntVector.fromArray(INT_128_SPECIES, NEIGHBORS_MASK, directionOffset);
            let masks_vector = std::simd::u8x4::from_array([
                NEIGHBORS_MASK[round % 4],
                NEIGHBORS_MASK[(round + 1) % 4],
                NEIGHBORS_MASK[(round + 2) % 4],
                NEIGHBORS_MASK[(round + 3) % 3],
            ]);

            for (elf_idx, elf) in elves.iter_mut().enumerate() {
                let neighbors_indices =
                    std::simd::usizex8::from(NEIGHBOR_OFFSETS.map(|v| {
                        (v + (elf.1 as i32) * (MAX_SIZE as i32) + (elf.0 as i32)) as usize
                    }));
                let neighbors_vector =
                    std::simd::u16x8::gather_or_default(&elf_grid, neighbors_indices);
                let neighbors_mask = neighbors_vector.simd_ne(no_elf_vector).to_bitmask();
                if neighbors_mask != 0 {
                    let neighbors_mask_repated = std::simd::u8x4::splat(neighbors_mask);
                    for (idx, &val) in (neighbors_mask_repated & masks_vector)
                        .as_array()
                        .iter()
                        .enumerate()
                    {
                        if val == 0 {
                            let to_move = MOVEMENTS[match idx {
                                0 => round % 4,
                                1 => (round + 1) % 4,
                                2 => (round + 2) % 4,
                                _ => (round + 3) % 4,
                            }];
                            elf_moves.push((elf_idx, to_move));
                            break;
                        }
                    }
                    /*
                    for rule_offset in 0..RULES.len() {
                        let (check_mask, to_move) = rules_for_mask[(round + rule_offset) % RULES.len()];
                        if !(check_mask & neighbors_mask).any() {
                            elf_moves.push((elf_idx, to_move));
                            break;
                        }
                    }
                     */
                }
            }
        }

        #[cfg(not(feature = "simd"))]
        for (elf_idx, elf) in elves.iter_mut().enumerate() {
            let adjacent_bitmask = DIRECTIONS
                .iter()
                .enumerate()
                .fold(0, |acc, (idx, (dx, dy))| {
                    acc | if elf_grid[(elf.1 + dy) as usize * MAX_SIZE + (elf.0 + dx) as usize]
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
                        elf_moves.push((elf_idx, to_move));
                        break;
                    }
                }
            }
        }

        for &(elf_idx, to_move) in elf_moves.iter() {
            let elf = &mut elves[elf_idx];
            let new_position = (elf.0 + to_move.0, elf.1 + to_move.1);
            if is_outside_max(new_position) {
                return Err(format!(
                    "Elf tried to moved outside of [0,{MAX_SIZE}): {new_position:?}"
                ));
            }

            let elf_idx_at_position =
                elf_grid[new_position.1 as usize * MAX_SIZE + new_position.0 as usize];

            if elf_idx_at_position == NO_ELF {
                elf_grid[elf.1 as usize * MAX_SIZE + elf.0 as usize] = NO_ELF;
                *elf = new_position;
                elf_grid[elf.1 as usize * MAX_SIZE + elf.0 as usize] = elf_idx as u16;
                num_moves += 1;
            } else {
                // Position was occupied - stand still and push other elf (which must be coming from other direction) back:
                elf_grid[new_position.1 as usize * MAX_SIZE + new_position.0 as usize] = NO_ELF;
                let pushed_back_position = (new_position.0 + to_move.0, new_position.1 + to_move.1);
                elves[elf_idx_at_position as usize] = pushed_back_position;
                elf_grid[pushed_back_position.1 as usize * MAX_SIZE
                    + pushed_back_position.0 as usize] = elf_idx_at_position;
                num_moves -= 1;
            }
        }
        elf_moves.clear();

        #[cfg(feature = "visualization")]
        {
            elf_positions_per_step.push(elves.clone());

            if num_moves == 0 || (input.is_part_one() && round == 9) {
                let mut svg = SvgImage::new();
                let mut elf_position_rect_ids = Vec::new();
                for elf in elf_positions_per_step[0].iter() {
                    elf_position_rect_ids.push(
                        svg.add_with_id(
                            SvgRect::default()
                                .x(elf.0 as Coordinate)
                                .y(elf.1 as Coordinate)
                                .width(1)
                                .height(1),
                        ),
                    );
                }

                let (max_coords, min_coords) = elf_positions_per_step.iter().flatten().fold(
                    ((0, 0), (i16::MAX, i16::MAX)),
                    |(max, min), elf| {
                        (
                            (elf.0.max(max.0), elf.1.max(max.1)),
                            (elf.0.min(min.0), elf.1.min(min.1)),
                        )
                    },
                );
                let step_duration_ms = 300;
                let animation_duration_ms = step_duration_ms - 100;
                svg.add(SvgStyle::new(format!("\n\
    rect {{ fill: #00B1D2; transition: x {}ms, y {}ms, fill {}ms; }} rect.moving {{ fill: #FDDB27 !important; }}
", animation_duration_ms, animation_duration_ms, animation_duration_ms)));

                let array_declaration = format!(
                    "const elfPositions = [{}];",
                    elf_positions_per_step
                        .iter()
                        .map(|positions| format!(
                            "[{}]",
                            positions
                                .iter()
                                .map(|p| format!("[{},{}]", p.0, p.1))
                                .collect::<Vec<_>>()
                                .join(",")
                        ))
                        .collect::<Vec<_>>()
                        .join(",")
                );
                svg.add(SvgScript::new(format!("{}{}", array_declaration, format!(
                    "\nconst elfRects = document.querySelectorAll('rect');\n\
window.onNewStep = (step) => {{\n\
        const prevPos = (step == 0) ? null : elfPositions[step-1];\n\
        const pos = elfPositions[step];\n\
        for (let i = 0; i < {}; i++) {{\n\
            const e = elfRects[i];
            e.setAttribute('x', pos[i][0]);\n\
            e.setAttribute('y', pos[i][1]);\n\
            if (prevPos === null || (prevPos[i][0] === pos[i][0] && prevPos[i][1] === pos[i][1])) {{\n\
               e.classList.remove('moving');\n\
            }} else {{\n\
               e.classList.add('moving');\n\
            }}\n\
        }}\n\
}};",
                    elves.len(),
                ))));
                input.rendered_svg.replace(
                    svg.view_box((
                        min_coords.0 as i64,
                        min_coords.1 as i64,
                        (max_coords.0 - min_coords.0) as i64,
                        (max_coords.1 - min_coords.1) as i64,
                    ))
                    .style("background: black;")
                    .data_attribute("steps".to_string(), format!("{}", round + 1))
                    .data_attribute("step-duration".to_string(), format!("{}", step_duration_ms))
                    .to_svg_string(),
                );
            }
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
                    acc.0.min(e.0),
                    acc.1.max(e.0),
                    acc.2.min(e.1),
                    acc.3.max(e.1),
                )
            });
    let rectangle_size = ((max_x + 1 - min_x) * (max_y + 1 - min_y)) as usize;
    Ok(rectangle_size - elves.len())
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
