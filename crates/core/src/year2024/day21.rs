use std::collections::HashMap;

use crate::common::permutation::all_permutations;
use crate::input::{Input, on_error};

pub fn solve(input: &Input) -> Result<u64, String> {
    let mut result = 0;
    let mut cache = HashMap::with_capacity(1024);
    for line in input.text.lines() {
        let numeric_part = line[..(line.len() - 1)]
            .parse::<u64>()
            .map_err(|_| on_error())?;
        let depth = input.part_values(2, 25);
        result += numeric_part
            * lowest_press_count(
                line.as_bytes(),
                depth,
                KeypadType::Numeric,
                KeypadType::Numeric.location_of(b'A'),
                &mut cache,
            );
    }
    Ok(result)
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
enum KeypadType {
    Numeric,
    Directional,
}

impl KeypadType {
    fn location_of(self, c: u8) -> (i32, i32) {
        match (self, c) {
            (Self::Numeric, b'7') => (0, 0),
            (Self::Numeric, b'8') => (1, 0),
            (Self::Numeric, b'9') => (2, 0),
            (Self::Numeric, b'4') => (0, 1),
            (Self::Numeric, b'5') => (1, 1),
            (Self::Numeric, b'6') => (2, 1),
            (Self::Numeric, b'1') => (0, 2),
            (Self::Numeric, b'2') => (1, 2),
            (Self::Numeric, b'3') => (2, 2),
            (Self::Numeric, b'0') => (1, 3),
            (Self::Numeric, b'A') => (2, 3),
            (Self::Directional, b'^') => (1, 0),
            (Self::Directional, b'A') => (2, 0),
            (Self::Directional, b'<') => (0, 1),
            (Self::Directional, b'v') => (1, 1),
            (Self::Directional, b'>') => (2, 1),
            _ => unreachable!("Bad key '{}', type = {:?}", c as char, self),
        }
    }

    const fn contains(self, position: (i32, i32)) -> bool {
        matches!(
            (self, position),
            (Self::Numeric, (0..=2, 0..=2))
                | (Self::Numeric, (1 | 2, 3))
                | (Self::Directional, (1 | 2, 0))
                | (Self::Directional, (0..=2, 1))
        )
    }
}

const fn direction_of(button: u8, location: (i32, i32)) -> (i32, i32) {
    let (dx, dy) = match button {
        b'^' => (0, -1),
        b'>' => (1, 0),
        b'v' => (0, 1),
        _ => (-1, 0),
    };
    (location.0 + dx, location.1 + dy)
}

type Cache = HashMap<(u64, u32, KeypadType, (i32, i32)), u64>;

fn lowest_press_count(
    sequence: &[u8],
    depth: u32,
    keypad_type: KeypadType,
    current_location: (i32, i32),
    cache: &mut Cache,
) -> u64 {
    if sequence.is_empty() {
        return 0;
    }

    let sequence_cache_key = slice_to_u64(sequence);
    if let Some(&val) = cache.get(&(sequence_cache_key, depth, keypad_type, current_location)) {
        return val;
    }

    let destination_position = keypad_type.location_of(sequence[0]);
    let (dx, dy) = (
        destination_position.0 - current_location.0,
        destination_position.1 - current_location.1,
    );

    let first_button_min_presses = if depth == 0 {
        // Outermost human direct key presses (+1 to press the button)
        (dx.abs() + dy.abs()) as u64 + 1
    } else {
        let mut buttons = [0; 4];
        let buttons_cnt = (dx.abs() + dy.abs()) as usize;
        let x_button = if dx > 0 { b'>' } else { b'<' };
        let y_button = if dy > 0 { b'v' } else { b'^' };
        buttons[..(dx.unsigned_abs() as usize)].fill(x_button);
        buttons[(dx.unsigned_abs() as usize)..((dx.abs() + dy.abs()) as usize)].fill(y_button);

        let mut lowest_count = u64::MAX;
        all_permutations(&mut buttons[0..buttons_cnt], &mut |permutation| {
            let mut moved_location = current_location;
            for &button in permutation {
                moved_location = direction_of(button, moved_location);
                if !keypad_type.contains(moved_location) {
                    return Ok(());
                }
            }
            let mut button_presses = [0_u8; 5];
            button_presses[0..permutation.len()].clone_from_slice(permutation);
            button_presses[permutation.len()] = b'A';
            lowest_count = lowest_count.min(lowest_press_count(
                &button_presses[..(permutation.len() + 1)],
                depth - 1,
                KeypadType::Directional,
                KeypadType::Directional.location_of(b'A'),
                cache,
            ));
            Ok(())
        })
        .unwrap();
        lowest_count
    };

    let result = first_button_min_presses
        + lowest_press_count(
            &sequence[1..],
            depth,
            keypad_type,
            destination_position,
            cache,
        );
    cache.insert(
        (sequence_cache_key, depth, keypad_type, current_location),
        result,
    );
    result
}

fn slice_to_u64(slice: &[u8]) -> u64 {
    slice
        .iter()
        .enumerate()
        .fold(0, |acc, (i, &x)| acc | ((x as u64) << ((8 * i) as u64)))
}

#[test]
pub fn tests() {
    for key in ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A'] {
        assert!(KeypadType::Numeric.contains(KeypadType::Numeric.location_of(key as u8)));
    }
    for key in ['<', '^', '>', 'v', 'A'] {
        assert!(KeypadType::Directional.contains(KeypadType::Directional.location_of(key as u8)));
    }

    let test_input = "029A
980A
179A
456A
379A";
    test_part_one!(test_input => 126_384);

    let real_input = include_str!("day21_input.txt");
    test_part_one!(real_input => 138_764);
    test_part_two!(real_input => 169_137_886_514_152);
}
