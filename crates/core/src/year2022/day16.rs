use crate::input::Input;
use std::collections::HashMap;

type ReleasedPressure = i16;

pub fn solve(input: &mut Input) -> Result<ReleasedPressure, String> {
    let (start_valve_idx, valves) =
        Valve::parse(input.text).ok_or_else(|| "Invalid input".to_string())?;
    let mut search_state = SearchState::new(valves, start_valve_idx);

    let available_minutes = input.part_values(30, 26);
    let num_actors = input.part_values(1, 2);

    for actor in 0..num_actors {
        for minute in 0..available_minutes {
            let minutes_left = available_minutes - minute - 1;
            search_state.advance_one_minute(minutes_left);
        }

        if num_actors > 1 && actor == 0 {
            search_state.setup_for_second_actor();
        }
    }

    Ok(search_state.most_pressure_that_can_be_released())
}

struct Valve {
    tunnels: Vec<usize>,
    flow_rate: ReleasedPressure,
    mask: i32,
}

impl Valve {
    fn parse(input: &str) -> Option<(usize, Vec<Self>)> {
        let mut valves = Vec::new();
        let mut tunnel_names = Vec::new();

        let mut name_to_valve_idx = HashMap::new();

        let mut num_working_valves = 0;
        for (line_idx, line) in input.lines().enumerate() {
            if line.len() < 10 {
                return None;
            }
            let name = &line[6..8];
            name_to_valve_idx.insert(name.to_string(), line_idx);

            let flow = line
                .split(['=', ';'])
                .nth(1)?
                .parse::<ReleasedPressure>()
                .ok()?;

            let tunnels_comma_separated = if line.contains("tunnels") {
                line.split("valves ").nth(1)?
            } else {
                line.split("valve ").nth(1)?
            };

            let mask = if flow > 0 {
                let result = 1 << num_working_valves;
                num_working_valves += 1;
                result
            } else {
                0
            };

            valves.push(Self {
                tunnels: Vec::new(),
                flow_rate: flow,
                mask,
            });
            tunnel_names.push(
                tunnels_comma_separated
                    .split(", ")
                    .map(str::to_string)
                    .collect::<Vec<_>>(),
            );
        }

        for (valve_idx, names) in tunnel_names.iter().enumerate() {
            valves[valve_idx]
                .tunnels
                .extend(names.iter().filter_map(|name| name_to_valve_idx.get(name)));
        }

        let start_valve_idx = *name_to_valve_idx.get("AA")?;
        Some((start_valve_idx, valves))
    }
}

struct SearchState {
    start_valve_idx: usize,
    num_possible_values_of_bitset: usize,
    valves: Vec<Valve>,
    /// Indexed by (current_position, opened_valves_bitset), where
    ///   current_position is the index of the actor position
    ///   enabled_valves_bitset is a bitset of opened valves (101 means that valve 0 and 2 are opened)
    /// These are encoded as:
    ///   index = current_position * (num_possible_values_of_bitset + 1) + opened_valves_bitset
    /// The value at the index is the highest possible pressure released while being at the state
    /// associated with the index - (current_position, opened_valves_bitset).
    released_pressure: Vec<ReleasedPressure>,
    released_pressure_new: Vec<ReleasedPressure>,
}

impl SearchState {
    const IMPOSSIBLE: ReleasedPressure = -1;

    fn new(valves: Vec<Valve>, start_valve_idx: usize) -> Self {
        let num_working_valves = valves.iter().filter(|valve| valve.flow_rate > 0).count();
        let num_total_valves = valves.len();

        let num_possible_values_of_bitset = 1 << num_working_valves;
        let num_total_states = num_total_valves * num_possible_values_of_bitset;

        let mut released_pressure = vec![Self::IMPOSSIBLE; num_total_states];
        released_pressure[start_valve_idx * num_possible_values_of_bitset] = 0;

        Self {
            start_valve_idx,
            num_possible_values_of_bitset,
            valves,
            released_pressure,
            released_pressure_new: vec![Self::IMPOSSIBLE; num_total_states],
        }
    }

    fn advance_one_minute(&mut self, minutes_left: i32) {
        for (state_idx, &released_pressure) in self.released_pressure.iter().enumerate() {
            if released_pressure == Self::IMPOSSIBLE {
                continue;
            }

            let valve_idx = state_idx / self.num_possible_values_of_bitset;
            let opened_valves_bitset = state_idx % self.num_possible_values_of_bitset;
            let valve = &self.valves[valve_idx];

            for &possible_new_valve_idx in valve.tunnels.iter() {
                Self::set_if_higher(
                    &mut self.released_pressure_new[possible_new_valve_idx
                        * self.num_possible_values_of_bitset
                        + opened_valves_bitset],
                    self.released_pressure
                        [valve_idx * self.num_possible_values_of_bitset + opened_valves_bitset],
                );
            }

            if valve.flow_rate > 0 && (opened_valves_bitset as i32 & valve.mask) == 0 {
                let flow_increase = minutes_left as ReleasedPressure * valve.flow_rate;
                let new_released_pressure = released_pressure + flow_increase;
                let new_opened_valves_bitset =
                    ((opened_valves_bitset as i32) | valve.mask) as usize;
                Self::set_if_higher(
                    &mut self.released_pressure_new
                        [valve_idx * self.num_possible_values_of_bitset + new_opened_valves_bitset],
                    new_released_pressure,
                );
            }
        }
        std::mem::swap(&mut self.released_pressure, &mut self.released_pressure_new);
    }

    fn setup_for_second_actor(&mut self) {
        self.released_pressure_new.fill(Self::IMPOSSIBLE);
        for (state_idx, &released_pressure) in self.released_pressure.iter().enumerate() {
            let opened_valves_bitset = state_idx % self.num_possible_values_of_bitset;
            Self::set_if_higher(
                &mut self.released_pressure_new[self.start_valve_idx
                    * self.num_possible_values_of_bitset
                    + opened_valves_bitset],
                released_pressure,
            );
        }
        std::mem::swap(&mut self.released_pressure, &mut self.released_pressure_new);
        self.released_pressure_new.fill(Self::IMPOSSIBLE);
    }

    fn most_pressure_that_can_be_released(&mut self) -> ReleasedPressure {
        self.released_pressure
            .iter()
            .copied()
            .max()
            .unwrap_or_default()
    }

    fn set_if_higher(current: &mut ReleasedPressure, possibly_higher: ReleasedPressure) {
        if possibly_higher > *current {
            *current = possibly_higher;
        }
    }
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
