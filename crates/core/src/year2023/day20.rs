use crate::common::array_deque::ArrayDeque;
use crate::common::id_assigner::IdAssigner;
use crate::input::{on_error, Input};

pub fn solve(input: &Input) -> Result<u64, String> {
    const MAX_MODULES: usize = 64;
    const MAX_ITERATIONS: u64 = 10_000;
    const MAX_WORK_QUEUE_SIZE: usize = 128;
    const EXPECTED_NUM_RX_TOGGLES: u32 = 4;

    let mut id_assigner = IdAssigner::<MAX_MODULES, str>::new("");
    let mut is_flip_flop = 0_u64;
    let mut enabled_flip_flops = 0_u64;
    let mut broadcaster_idx = 0_usize;
    let mut conjunctions_memory = [0_u64; MAX_MODULES];
    let mut num_sources = [0_u32; MAX_MODULES];
    let mut destinations = [0_u64; MAX_MODULES];
    let (mut sent_lows, mut sent_highs) = (0, 0);
    let mut cycle_length = 1_u64;
    let mut found_cycles_bitset = 0_u32;
    let mut rx_emitter_idx = usize::MAX;
    let mut rx_emitter_sources = 0_u64;

    for line in input.text.lines() {
        let (src_module_name, destinations_str) = line.split_once(" -> ").ok_or_else(on_error)?;
        let src_module_idx = id_assigner.id_of(&src_module_name[1..])?;

        if src_module_name == "broadcaster" {
            broadcaster_idx = usize::from(src_module_idx);
        } else if src_module_name.as_bytes()[0] == b'%' {
            is_flip_flop |= 1 << src_module_idx;
        };

        for destination_name in destinations_str.split(", ") {
            let destination_idx = id_assigner.id_of(destination_name)?;
            destinations[usize::from(src_module_idx)] |= 1 << u64::from(destination_idx);
            num_sources[usize::from(destination_idx)] += 1;
            if destination_name == "rx" && input.is_part_two() {
                if rx_emitter_idx != usize::MAX {
                    return Err("Multiple emitters to 'rx'".to_string());
                }
                rx_emitter_idx = usize::from(src_module_idx);
            }
        }
    }

    if input.is_part_two() {
        if rx_emitter_idx == usize::MAX {
            return Err("No emitter to 'rx'".to_string());
        } else if is_flip_flop & (1 << rx_emitter_idx) != 0 {
            return Err("The emitter to 'rx' is not a conjunction".to_string());
        }
        for (i, destination) in destinations.iter().enumerate().take(id_assigner.len()) {
            if destination & (1 << rx_emitter_idx) != 0 {
                if is_flip_flop & (1 << i) != 0 {
                    return Err(
                        "Not all input sources to the 'rx' emitter module are conjunctions"
                            .to_string(),
                    );
                }
                rx_emitter_sources |= 1 << i;
            }
        }
        if rx_emitter_sources.count_ones() != EXPECTED_NUM_RX_TOGGLES {
            return Err("Not four emitters to 'rx'".to_string());
        }
    }

    for button_presses in 0..MAX_ITERATIONS {
        if input.is_part_one() && button_presses == 1000 {
            return Ok(sent_lows * sent_highs);
        }

        let mut work_queue = ArrayDeque::<MAX_WORK_QUEUE_SIZE, (u8, u8, bool)>::new();
        work_queue.push_back((63, broadcaster_idx as u8, false))?;

        while let Some((src_idx, dest_idx, high_pulse)) = work_queue.pop_front() {
            let dest_idx = usize::from(dest_idx);

            if input.is_part_one() {
                sent_highs += u64::from(high_pulse);
                sent_lows += u64::from(!high_pulse);
            } else if !high_pulse && (rx_emitter_sources & (1 << dest_idx)) != 0 {
                let src_idx =
                    EXPECTED_NUM_RX_TOGGLES - (rx_emitter_sources >> dest_idx).count_ones();
                let bit = 1 << src_idx;
                if found_cycles_bitset & bit == 0 {
                    found_cycles_bitset |= bit;
                    cycle_length *= button_presses + 1;
                    if found_cycles_bitset.count_ones() == EXPECTED_NUM_RX_TOGGLES {
                        return Ok(cycle_length);
                    }
                }
            }

            let is_flip_flop = is_flip_flop & (1 << dest_idx) != 0;
            let emit_high = if dest_idx == broadcaster_idx {
                false
            } else if is_flip_flop {
                if high_pulse {
                    // "If a flip-flop module receives a high pulse, it is ignored and nothing happens."
                    continue;
                }
                // "However, if a flip-flop module receives a low pulse, it flips between on and off. If it was
                // off, it turns on and sends a high pulse. If it was on, it turns off and sends a low pulse."
                let bit = 1 << dest_idx;
                enabled_flip_flops ^= bit;
                (enabled_flip_flops & bit) != 0
            } else {
                // "Conjunction modules (prefix &) remember the type of the most recent pulse received from each of
                // their connected input modules; they initially default to remembering a low pulse for each input.
                // When a pulse is received, the conjunction module first updates its memory for that input."
                if high_pulse {
                    conjunctions_memory[dest_idx] |= 1 << src_idx;
                } else {
                    conjunctions_memory[dest_idx] &= !(1 << src_idx);
                }
                // "Then, if it remembers high pulses for all inputs, it
                // sends a low pulse; otherwise, it sends a high pulse."
                conjunctions_memory[dest_idx].count_ones() != num_sources[dest_idx]
            };

            let mut destination_bitset = destinations[dest_idx];
            while destination_bitset != 0 {
                let destination_idx = destination_bitset.trailing_zeros();
                destination_bitset &= !(1 << destination_idx);
                work_queue.push_back((dest_idx as u8, destination_idx as u8, emit_high))?;
            }
        }
    }

    Err("No solution found".to_string())
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one_no_allocations, test_part_two_no_allocations};

    let test_input = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";
    test_part_one_no_allocations!(test_input => 32_000_000);
    let test_input = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";
    test_part_one_no_allocations!(test_input => 11_687_500);

    let real_input = include_str!("day20_input.txt");
    test_part_one_no_allocations!(real_input => 812_721_756);
    test_part_two_no_allocations!(real_input => 233_338_595_643_977);
}
