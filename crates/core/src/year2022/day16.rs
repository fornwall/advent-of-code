use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

use crate::input::Input;

pub fn solve(input: &mut Input) -> Result<usize, String> {
    let actor_1_remaining_minutes = input.part_values(30, 26);
    let actor_2_remaining_minutes = input.part_values(0, 26);

    let (distances, flows) = parse(input.text).ok_or("Invalid input")?;

    // Compute min distances useful for upper bound calculations.
    let min_distances = (0..=std::cmp::max(actor_1_remaining_minutes, actor_2_remaining_minutes))
        .map(|minute| {
            let mut v = distances
                .iter()
                .map(|distances| {
                    distances
                        .iter()
                        .copied()
                        .filter(|&distance| distance != 0)
                        .min()
                        .unwrap_or(usize::MAX - 1)
                })
                .enumerate()
                .filter(|&(flow_idx, min_distance)| {
                    minute > (min_distance + 1) && flows[flow_idx] != 0
                })
                .collect::<Vec<_>>();
            v.sort_unstable_by_key(|&(flow_idx, min_distance)| {
                Reverse(flows[flow_idx] * (minute - (min_distance + 1)))
            });
            v
        })
        .collect::<Vec<_>>();

    let upper_bound = |state: SearchState| {
        let mut upper_bound = state.released_pressure;
        let mut opened_bitset = state.opened_bitset;
        let mut t1 = state.actor_1_remaining_minutes;
        let mut t2 = state.actor_2_remaining_minutes;
        'outer: loop {
            for &(flow_idx, min_distance) in &min_distances[t1] {
                if opened_bitset & (1 << flow_idx) == 0 {
                    opened_bitset |= 1 << flow_idx;
                    t1 -= min_distance + 1;
                    upper_bound += flows[flow_idx] * t1;
                    (t1, t2) = (t1.max(t2), t1.min(t2));
                    continue 'outer;
                }
            }
            return upper_bound;
        }
    };

    let mut best = 0;
    let mut to_visit = BinaryHeap::from_iter([SearchState {
        upper_bound: usize::MAX,
        released_pressure: 0,
        actor_1_flow_idx: 0,
        actor_2_flow_idx: 0,
        actor_1_remaining_minutes,
        actor_2_remaining_minutes,
        opened_bitset: 1,
    }]);
    let mut visited = HashSet::new();

    while let Some(state) = to_visit.pop() {
        if state.upper_bound <= best {
            break;
        }

        best = best.max(state.released_pressure);

        if !visited.insert((
            state.actor_1_flow_idx,
            state.actor_1_remaining_minutes,
            state.actor_2_flow_idx,
            state.actor_2_remaining_minutes,
            state.opened_bitset,
        )) {
            continue;
        }

        for (flow_idx, &travel_time) in distances[state.actor_1_flow_idx].iter().enumerate() {
            let enough_time_remaining = travel_time < state.actor_1_remaining_minutes;
            let can_be_opened = (state.opened_bitset & (1 << flow_idx)) == 0;

            if enough_time_remaining && can_be_opened {
                let remaining_minutes_after_opening =
                    state.actor_1_remaining_minutes - (travel_time + 1);
                let mut new_state = SearchState {
                    actor_1_flow_idx: flow_idx,
                    actor_1_remaining_minutes: remaining_minutes_after_opening,
                    released_pressure: state.released_pressure
                        + remaining_minutes_after_opening * flows[flow_idx],
                    opened_bitset: state.opened_bitset | (1 << flow_idx),
                    ..state
                };
                if new_state.actor_1_remaining_minutes < new_state.actor_2_remaining_minutes {
                    std::mem::swap(
                        &mut new_state.actor_1_remaining_minutes,
                        &mut new_state.actor_2_remaining_minutes,
                    );
                    std::mem::swap(
                        &mut new_state.actor_1_flow_idx,
                        &mut new_state.actor_2_flow_idx,
                    );
                }
                new_state.upper_bound = upper_bound(new_state);
                if new_state.upper_bound > best {
                    to_visit.push(new_state);
                }
            }
        }

        if state.actor_2_remaining_minutes != 0 {
            // The second actor could proceed even if first actor is stuck.
            let mut new_state = SearchState {
                actor_1_remaining_minutes: state.actor_2_remaining_minutes,
                actor_1_flow_idx: state.actor_2_flow_idx,
                actor_2_remaining_minutes: 0,
                ..state
            };
            new_state.upper_bound = upper_bound(new_state);
            to_visit.push(new_state);
        }
    }

    Ok(best)
}

fn parse(input: &str) -> Option<(Vec<Vec<usize>>, Vec<usize>)> {
    const CAPACITY_ESTIMATE: usize = 64;
    let mut flow_rates = Vec::with_capacity(CAPACITY_ESTIMATE);
    let mut tunnel_names = Vec::with_capacity(CAPACITY_ESTIMATE);
    let mut name_to_valve_idx = HashMap::with_capacity(CAPACITY_ESTIMATE);

    for (line_idx, line) in input.lines().enumerate() {
        if line.len() < 10 {
            return None;
        }
        let (name, rest) = line[6..].split_once(' ')?;
        name_to_valve_idx.insert(name.to_string(), line_idx);
        let (_, rest) = rest.split_once('=')?;
        let (flow_rate_str, rest) = rest.split_once(';')?;
        let flow_rate = flow_rate_str.parse::<u16>().ok()? as usize;
        let (_, rest) = rest.split_once("valve")?;
        let rest = rest.trim_start_matches('s').trim_start();
        let linked_tunnel_names = rest.split(", ").map(str::to_string).collect::<Vec<_>>();
        flow_rates.push(flow_rate);
        tunnel_names.push(linked_tunnel_names);
    }

    let tunnel_ids = tunnel_names
        .iter()
        .map(|names| {
            names
                .iter()
                .filter_map(|name| name_to_valve_idx.get(name).copied())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut flows = vec![0];
    // nonzero[valve_idx]:
    //   - If valve_idx has a zero flow: usize::MAX
    //   - If valve_idx has a non-zero flow: index into flows
    let mut nonzero = vec![usize::MAX; flow_rates.len()];
    nonzero[*name_to_valve_idx.get("AA")?] = 0;

    for (valve_idx, &flow_rate) in flow_rates.iter().enumerate() {
        if flow_rate != 0 {
            nonzero[valve_idx] = flows.len();
            flows.push(flow_rate);
        }
    }

    let mut distances = vec![vec![usize::MAX; flows.len()]; flows.len()];
    let mut visited = vec![false; flow_rates.len()];
    let mut queue = VecDeque::with_capacity(flow_rates.len());

    for (valve_idx, &flow_idx) in nonzero.iter().enumerate() {
        if flow_idx != usize::MAX {
            let distances = &mut distances[flow_idx];

            queue.clear();
            queue.push_back((valve_idx, 0));
            visited.fill(false);

            while let Some((visited_valve_idx, d)) = queue.pop_front() {
                if !visited[visited_valve_idx] {
                    visited[visited_valve_idx] = true;
                    if nonzero[visited_valve_idx] != usize::MAX {
                        distances[nonzero[visited_valve_idx]] = d;
                    }
                    queue.extend(
                        tunnel_ids[visited_valve_idx]
                            .iter()
                            .map(|&next| (next, d + 1)),
                    );
                }
            }
        }
    }

    (flows.len() < 64).then_some((distances, flows))
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct SearchState {
    upper_bound: usize,
    released_pressure: usize,
    actor_1_flow_idx: usize,
    actor_1_remaining_minutes: usize,
    actor_2_flow_idx: usize,
    actor_2_remaining_minutes: usize,
    opened_bitset: u64,
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    let test_input = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";
    test_part_one!(test_input => 1651);
    test_part_two!(test_input => 1707);

    let real_input = include_str!("day16_input.txt");
    test_part_one!(real_input => 2359);
    test_part_two!(real_input => 2999);

    // Additional tests from:
    // https://www.reddit.com/r/adventofcode/comments/znklnh/2022_day_16_some_extra_test_cases_for_day_16/
    // Testcase 1 - A Line, linearly increasing rates:
    let test_input = "Valve AA has flow rate=0; tunnels lead to valves BA
Valve BA has flow rate=2; tunnels lead to valves AA, CA
Valve CA has flow rate=4; tunnels lead to valves BA, DA
Valve DA has flow rate=6; tunnels lead to valves CA, EA
Valve EA has flow rate=8; tunnels lead to valves DA, FA
Valve FA has flow rate=10; tunnels lead to valves EA, GA
Valve GA has flow rate=12; tunnels lead to valves FA, HA
Valve HA has flow rate=14; tunnels lead to valves GA, IA
Valve IA has flow rate=16; tunnels lead to valves HA, JA
Valve JA has flow rate=18; tunnels lead to valves IA, KA
Valve KA has flow rate=20; tunnels lead to valves JA, LA
Valve LA has flow rate=22; tunnels lead to valves KA, MA
Valve MA has flow rate=24; tunnels lead to valves LA, NA
Valve NA has flow rate=26; tunnels lead to valves MA, OA
Valve OA has flow rate=28; tunnels lead to valves NA, PA
Valve PA has flow rate=30; tunnels lead to valves OA";
    test_part_one!(test_input => 2640);
    test_part_two!(test_input => 2670);
    // Testcase 2 - A Line, quadratically increasing rates:
    let test_input = "Valve AA has flow rate=0; tunnels lead to valves BA
Valve BA has flow rate=1; tunnels lead to valves AA, CA
Valve CA has flow rate=4; tunnels lead to valves BA, DA
Valve DA has flow rate=9; tunnels lead to valves CA, EA
Valve EA has flow rate=16; tunnels lead to valves DA, FA
Valve FA has flow rate=25; tunnels lead to valves EA, GA
Valve GA has flow rate=36; tunnels lead to valves FA, HA
Valve HA has flow rate=49; tunnels lead to valves GA, IA
Valve IA has flow rate=64; tunnels lead to valves HA, JA
Valve JA has flow rate=81; tunnels lead to valves IA, KA
Valve KA has flow rate=100; tunnels lead to valves JA, LA
Valve LA has flow rate=121; tunnels lead to valves KA, MA
Valve MA has flow rate=144; tunnels lead to valves LA, NA
Valve NA has flow rate=169; tunnels lead to valves MA, OA
Valve OA has flow rate=196; tunnels lead to valves NA, PA
Valve PA has flow rate=225; tunnels lead to valves OA";
    test_part_one!(test_input => 13468);
    test_part_two!(test_input => 12887);
    // Testcase 3 - A circle:
    let test_input = "Valve BA has flow rate=2; tunnels lead to valves AA, CA
Valve CA has flow rate=10; tunnels lead to valves BA, DA
Valve DA has flow rate=2; tunnels lead to valves CA, EA
Valve EA has flow rate=10; tunnels lead to valves DA, FA
Valve FA has flow rate=2; tunnels lead to valves EA, GA
Valve GA has flow rate=10; tunnels lead to valves FA, HA
Valve HA has flow rate=2; tunnels lead to valves GA, IA
Valve IA has flow rate=10; tunnels lead to valves HA, JA
Valve JA has flow rate=2; tunnels lead to valves IA, KA
Valve KA has flow rate=10; tunnels lead to valves JA, LA
Valve LA has flow rate=2; tunnels lead to valves KA, MA
Valve MA has flow rate=10; tunnels lead to valves LA, NA
Valve NA has flow rate=2; tunnels lead to valves MA, OA
Valve OA has flow rate=10; tunnels lead to valves NA, PA
Valve PA has flow rate=2; tunnels lead to valves OA, AA
Valve AA has flow rate=0; tunnels lead to valves BA, PA";
    test_part_one!(test_input => 1288);
    test_part_two!(test_input => 1484);
    // Testcase 4 - Clusters:
    let test_input = "Valve AA has flow rate=0; tunnels lead to valves AB, BB, CB
Valve AB has flow rate=0; tunnels lead to valves AA, AC
Valve AC has flow rate=0; tunnels lead to valves AB, AD
Valve AD has flow rate=0; tunnels lead to valves AC, AE
Valve AE has flow rate=0; tunnels lead to valves AD, AF
Valve AF has flow rate=0; tunnels lead to valves AE, AG
Valve AG has flow rate=0; tunnels lead to valves AF, AH
Valve AH has flow rate=0; tunnels lead to valves AG, AI
Valve AI has flow rate=0; tunnels lead to valves AH, AJ
Valve AJ has flow rate=0; tunnels lead to valves AI, AK
Valve AK has flow rate=100; tunnels lead to valves AJ, AW, AX, AY, AZ
Valve AW has flow rate=10; tunnels lead to valves AK
Valve AX has flow rate=10; tunnels lead to valves AK
Valve AY has flow rate=10; tunnels lead to valves AK
Valve AZ has flow rate=10; tunnels lead to valves AK
Valve BB has flow rate=0; tunnels lead to valves AA, BC
Valve BC has flow rate=0; tunnels lead to valves BB, BD
Valve BD has flow rate=0; tunnels lead to valves BC, BE
Valve BE has flow rate=0; tunnels lead to valves BD, BF
Valve BF has flow rate=0; tunnels lead to valves BE, BG
Valve BG has flow rate=0; tunnels lead to valves BF, BH
Valve BH has flow rate=0; tunnels lead to valves BG, BI
Valve BI has flow rate=0; tunnels lead to valves BH, BJ
Valve BJ has flow rate=0; tunnels lead to valves BI, BK
Valve BK has flow rate=100; tunnels lead to valves BJ, BW, BX, BY, BZ
Valve BW has flow rate=10; tunnels lead to valves BK
Valve BX has flow rate=10; tunnels lead to valves BK
Valve BY has flow rate=10; tunnels lead to valves BK
Valve BZ has flow rate=10; tunnels lead to valves BK
Valve CB has flow rate=0; tunnels lead to valves AA, CC
Valve CC has flow rate=0; tunnels lead to valves CB, CD
Valve CD has flow rate=0; tunnels lead to valves CC, CE
Valve CE has flow rate=0; tunnels lead to valves CD, CF
Valve CF has flow rate=0; tunnels lead to valves CE, CG
Valve CG has flow rate=0; tunnels lead to valves CF, CH
Valve CH has flow rate=0; tunnels lead to valves CG, CI
Valve CI has flow rate=0; tunnels lead to valves CH, CJ
Valve CJ has flow rate=0; tunnels lead to valves CI, CK
Valve CK has flow rate=100; tunnels lead to valves CJ, CW, CX, CY, CZ
Valve CW has flow rate=10; tunnels lead to valves CK
Valve CX has flow rate=10; tunnels lead to valves CK
Valve CY has flow rate=10; tunnels lead to valves CK
Valve CZ has flow rate=10; tunnels lead to valves CK";
    test_part_one!(test_input => 2400);
    test_part_two!(test_input => 3680);
}
