use std::array;

use crate::input::Input;

pub fn solve(input: &Input) -> Result<u32, String> {
    let minutes = input.part_values(24, 32);
    let max_blueprints = input.part_values(64, 3);

    let blueprints = parse_blueprints(input.text);

    let max_geodes = blueprints
        .take(max_blueprints)
        .map(|blueprint| most_geodes_opened(&blueprint, minutes));

    Ok(if input.is_part_one() {
        max_geodes
            .enumerate()
            .map(|(offset, geodes)| (offset + 1) as u32 * geodes)
            .sum()
    } else {
        max_geodes.product()
    })
}

type Blueprint = [[u32; 4]; 4];

fn parse_blueprints(input: &str) -> impl Iterator<Item = Blueprint> + '_ {
    input.lines().filter_map(|line| {
        let mut words = line.split(' ');
        Some([
            [words.nth(6)?.parse::<u32>().ok()?, 0, 0, 0],
            [words.nth(5)?.parse::<u32>().ok()?, 0, 0, 0],
            [
                words.nth(5)?.parse::<u32>().ok()?,
                words.nth(2)?.parse::<u32>().ok()?,
                0,
                0,
            ],
            [
                words.nth(5)?.parse::<u32>().ok()?,
                0,
                words.nth(2)?.parse::<u32>().ok()?,
                0,
            ],
        ])
    })
}

#[derive(Clone, Copy)]
struct State {
    upper_bound: u32,
    minutes_remaining: u32,
    ores: [u32; 4],
    robots: [u32; 4],
}

fn most_geodes_opened(blueprint: &Blueprint, minutes: u32) -> u32 {
    let max_costs: [_; 4] = array::from_fn(|resource_idx| {
        if resource_idx == 3 {
            u32::MAX
        } else {
            blueprint
                .iter()
                .map(|costs| costs[resource_idx])
                .max()
                .unwrap_or_default()
        }
    });

    let initial_state = State {
        upper_bound: 1,
        minutes_remaining: minutes,
        ores: [0, 0, 0, 0],
        robots: [1, 0, 0, 0],
    };

    let mut to_visit = Vec::from([initial_state]);
    let mut most_geodes_produced = 0;

    while let Some(state) = to_visit.pop() {
        if state.upper_bound <= most_geodes_produced {
            continue;
        }

        most_geodes_produced = std::cmp::max(
            most_geodes_produced,
            state.ores[3] + state.robots[3] * state.minutes_remaining,
        );

        for robot_to_build_idx in 0..4 {
            if state.robots[robot_to_build_idx] == max_costs[robot_to_build_idx] {
                // No point in creating more robots of this type, as we are producing
                // enough each minute to fulfill all possible needs.
                continue;
            }

            let minutes_before_resources_obtained = (0..3)
                .map(|resource_idx| {
                    let resource_cost = blueprint[robot_to_build_idx][resource_idx];
                    if resource_cost <= state.ores[resource_idx] {
                        // We have enough resources already.
                        0
                    } else if state.robots[resource_idx] == 0 {
                        // We are not producing a resource required for this robot - abort.
                        state.minutes_remaining
                    } else {
                        1 + (resource_cost - state.ores[resource_idx] - 1)
                            / state.robots[resource_idx]
                    }
                })
                .max()
                .unwrap_or_default();

            if minutes_before_resources_obtained >= state.minutes_remaining {
                // There is not enough time to await constructing this robot.
                continue;
            }

            let mut new_state = state;
            for (j, ore) in new_state.ores.iter_mut().enumerate() {
                *ore = *ore + state.robots[j] * (minutes_before_resources_obtained + 1)
                    - blueprint[robot_to_build_idx][j];
            }
            new_state.minutes_remaining -= minutes_before_resources_obtained + 1;
            new_state.robots[robot_to_build_idx] += 1;
            new_state.upper_bound = {
                let mut ores = [new_state.ores; 4];
                let mut robots = new_state.robots;

                for _ in 0..new_state.minutes_remaining {
                    let can_build_robot: [u32; 4] = array::from_fn(|robot_idx| {
                        u32::from((0..3).all(|resource_idx| {
                            ores[robot_idx][resource_idx] >= blueprint[robot_idx][resource_idx]
                        }))
                    });
                    let new_ores: [[u32; 4]; 4] = array::from_fn(|robot_idx| {
                        array::from_fn(|resource_idx| {
                            ores[robot_idx][resource_idx] + robots[resource_idx]
                                - (can_build_robot[robot_idx] * blueprint[robot_idx][resource_idx])
                        })
                    });
                    for robot_idx in 0..4 {
                        robots[robot_idx] += can_build_robot[robot_idx];
                    }
                    ores = new_ores;
                }

                ores[0][3]
            };

            if new_state.upper_bound > most_geodes_produced {
                to_visit.push(new_state);
            }
        }
    }

    most_geodes_produced
}

#[test]
pub fn tests() {
    let test_input = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";
    test_part_one!(test_input => 33);
    test_part_two!(test_input => 3472);

    let real_input = include_str!("day19_input.txt");
    test_part_one!(real_input => 1834);
    test_part_two!(real_input => 2240);
}
