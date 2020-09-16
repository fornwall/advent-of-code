use mod_exp::mod_exp;

pub fn part1(input_string: &str) -> Result<usize, String> {
    let mut deck = Vec::new();
    for i in 0..10_007 {
        deck.push(i);
    }

    for (line_index, line) in input_string.lines().enumerate() {
        let error_message = || format!("Incorrect input at line {}: {}", line_index + 1, line);
        let error_message_arg = |_| error_message();

        if line.starts_with("deal into") {
            deck = deck.iter().rev().copied().collect();
        } else if line.starts_with("cut") {
            let how_many = line.split_whitespace().nth(1).ok_or_else(error_message)?;
            let how_many = how_many.parse::<i32>().map_err(error_message_arg)?;
            let how_many = if how_many > 0 {
                how_many
            } else {
                10_007 - how_many.abs()
            } as usize;
            deck = [&deck[how_many..], &deck[..how_many]].concat();
        } else if line.starts_with("deal with") {
            let increment = line
                .split_whitespace()
                .nth(3)
                .ok_or_else(error_message)
                .and_then(|value_str| value_str.parse::<usize>().map_err(error_message_arg))?;
            let old_deck = deck.clone();
            let mut current_index = 0;
            for &card_at_front in old_deck.iter() {
                deck[current_index] = card_at_front;
                current_index = (current_index + increment) % deck.len();
            }
        } else {
            return Err(format!("Invalid line: {}", line));
        }
    }

    let desired_card = 2019;
    deck.iter()
        .position(|&card| card == 2019)
        .ok_or(format!("No card {} found", desired_card))
}

/// Explanation:
/// https://www.reddit.com/r/adventofcode/comments/ee0rqi/2019_day_22_solutions/fbnkaju?utm_source=share&utm_medium=web2x
pub fn part2(input_string: &str) -> Result<i128, String> {
    fn inv(n: i128) -> i128 {
        mod_exp(n, MOD - 2, MOD)
    }

    const SHUFFLES: i128 = 101_741_582_076_661;

    const NUM_CARDS: i128 = 119_315_717_514_047;
    const MOD: i128 = NUM_CARDS;

    let mut offset_diff: i128 = 0;
    let mut increment_mul: i128 = 1;

    for (line_index, line) in input_string.lines().enumerate() {
        let error_message = || format!("Incorrect input at line {}: {}", line_index + 1, line);
        let error_message_arg = |_| error_message();

        if line.starts_with("deal into") {
            increment_mul *= -1;
            offset_diff += increment_mul;
        } else if line.starts_with("cut") {
            let n = line
                .split_whitespace()
                .nth(1)
                .ok_or_else(error_message)
                .and_then(|value_str| value_str.parse::<i128>().map_err(error_message_arg))?;
            offset_diff += increment_mul * n;
        } else if line.starts_with("deal with") {
            let n = line
                .split_whitespace()
                .nth(3)
                .ok_or_else(error_message)
                .and_then(|value_str| value_str.parse::<i128>().map_err(error_message_arg))?;
            increment_mul *= inv(n);
        // The offset (first card) does not change.
        // To get the increment, we need to calculate the second card.
        // "The 0th card in our old list goes to the 0th card in our new list, 1st card in
        // old goes to the nth card in new list (mod MOD), 2nd card in old goes to the 2*nth
        // card in new list, and so on. So, the ith card in our old list goes to the i*nth
        // card in the new list. When is i*n = 1?"
        } else {
            return Err(format!("Invalid line: {}", line));
        }

        increment_mul = increment_mul.rem_euclid(MOD);
        offset_diff = offset_diff.rem_euclid(MOD);
    }

    let increment = mod_exp(increment_mul, SHUFFLES, MOD);
    let offset = (offset_diff * (1_i128 - increment)).rem_euclid(MOD) * inv(1 - increment_mul);

    let result = (offset + increment * 2020).rem_euclid(MOD);
    Ok(result)
}

#[test]
pub fn tests_part1() {
    assert_eq!(part1(include_str!("day22_input.txt")), Ok(6526));
}

#[test]
fn tests_part2() {
    assert_eq!(
        part2(include_str!("day22_input.txt")),
        Ok(79_855_812_422_607)
    );
}
