use crate::input::Input;
use crate::mod_exp::mod_exp;
use std::collections::HashMap;

const MODULO: u64 = 20_201_227;

/// Computes `loop_size` so that `BASE ^ loop_size % MODULO == public_key`.
///
/// See https://en.wikipedia.org/wiki/Baby-step_giant-step
fn babystep_giantstep(public_key: u32) -> Option<u32> {
    // math.ceil(math.sqrt(MODULO-1)):
    const SQRT_MODULO_MINUS_ONE: u32 = 4_495;
    const BASE: u64 = 7;
    // mod_exp(BASE, SQRT_MODULO_MINUS_ONE * (MODULO - 2), MODULO):
    const FACTOR: u64 = 680_915;

    // Store the mapping from `BASE ^ x % MODULO` to `x` in a table:
    let baby_table = (0..SQRT_MODULO_MINUS_ONE)
        .scan(1_u32, |table_key, table_value| {
            let entry = Some((*table_key, table_value));
            *table_key = ((u64::from(*table_key) * BASE) % MODULO) as u32;
            entry
        })
        .collect::<HashMap<u32, u32>>();

    // https://stackoverflow.com/a/36893730/300710
    // Fermat little theorem states that
    //   "If p is a prime number, then for any integer a, the number a^p − a is an integer multiple of p."
    // Which is written as:
    //   a^p ≡ a (mod p)
    // If a is not divisible by p, it means that we can divide with a to get:
    //   a^(p-1) ≡ 1 (mod p)
    // or
    //   a * a^(p-2) ≡ 1 (mod p)
    // Which means that `a^(p-2)` is the multiplicative inverse of `a` (mod p), since the definition
    // of a multiplicative inverse `b` is: `a*b = 1 (mod p)`.
    //
    // So here:
    //    FACTOR := BASE^(SQRT_MODULO_MINUS_ONE * (MODULO - 2)) % MODULO
    // is the multiplicative inverse of `BASE` raised with SQRT_MODULO_MINUS_ONE,
    // so `BASE^-SQRT_MODULO_MINUS_ONE (mod p)`

    if let Err(value) = (0..SQRT_MODULO_MINUS_ONE).try_fold(public_key, |state, giant_step| {
        baby_table.get(&state).map_or_else(
            // No entry - continue with new state `(state * FACTOR) % MODULO`:
            || Ok(((u64::from(state) * FACTOR) % MODULO) as u32),
            // We have found x in `BASE ^ x % MODULO = state`, where
            //   state = (public_key * FACTOR ^ giant_step) % MODULO.
            // Multiply with `SQRT_MODULO_MINUS_ONE` (of which is `FACTOR` is
            // the multiplicative inverse) `giant_step` times to get back `public_key`.
            |x| Err(giant_step * SQRT_MODULO_MINUS_ONE + x),
        )
    }) {
        Some(value)
    } else {
        None
    }
}

pub fn solve(input: &Input) -> Result<u64, String> {
    if input.is_part_two() {
        return Ok(0);
    }

    let on_error = || "Invalid input".to_string();

    let mut lines = input.text.lines();

    let card_public_key = lines
        .next()
        .ok_or_else(on_error)?
        .parse::<u32>()
        .map_err(|_| on_error())?;

    let door_public_key = lines
        .next()
        .ok_or_else(on_error)?
        .parse::<u32>()
        .map_err(|_| on_error())?;

    let card_loop_size =
        babystep_giantstep(card_public_key).ok_or_else(|| "Invalid input".to_string())?;

    let encryption_key = mod_exp(
        i128::from(door_public_key),
        i128::from(card_loop_size),
        i128::from(MODULO),
    ) as u64;

    Ok(encryption_key)
}

#[test]
pub fn tests() {
    use crate::input::test_part_one;

    let example = "5764801\n17807724";
    test_part_one!(example => 14_897_079);
    let real_input = include_str!("day25_input.txt");
    test_part_one!(real_input => 18_862_163);
}
