use crate::input::Input;

#[derive(Copy, Clone)]
struct ExtendedEuclidResult {
    gcd: i128,
    x: i128,
    y: i128,
}

/// Given the integers `a` and `b`, compute:
///
/// - `gcd`, the greatest common divisor of `a` and `b`.
/// - Two integers `x` and `y` such that `gcd = a * x + b * y`, called the "Bézout coefficients".
///
/// See <https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm/>
/// and <https://cp-algorithms.com/algebra/extended-euclid-algorithm.html#toc-tgt-0/>
fn extended_euclid(a: i128, b: i128) -> ExtendedEuclidResult {
    if b == 0 {
        ExtendedEuclidResult { gcd: a, x: 1, y: 0 }
    } else {
        let next = extended_euclid(b, a % b);
        // We have found the coefficients (x', y') for (b, a % b):
        //     b * x' + (a % b) * y' = gcd             [1]
        // We want to find values for (x, y) in:
        //     a * x + b * y = gcd                     [2]
        // Since `a % b` can be written as:
        //     a % b = a - (a / b) * b                 [3]
        // we can rewrite [1] as:
        //     b * x' + (a - (a / b) * b) * y' = gcd   [4]
        // which means that for [2] and [4] to be equal:
        //     x = y'
        //     y = x' - (a / b) y'
        ExtendedEuclidResult {
            gcd: next.gcd,
            x: next.y,
            y: next.x - (a / b) * next.y,
        }
    }
}

/// Find the modular multiplicative inverse of the integer `a` with respect to modulo `m`
/// if and only if `a` and `m` are coprime (the only positive integer dividing both are 1).
///
/// That is, the value `x` returned makes `(a * x) % m` equal `1`.
fn modular_multiplicative_inverse(a: i128, m: i128) -> Option<i128> {
    // See https://en.wikipedia.org/wiki/Modular_multiplicative_inverse#Extended_Euclidean_algorithm
    // The extended Euclidean algorithm gives us `x` and `y` such that:
    //     a * x + m * y = gcd(a, m)
    // Since gcd(a, m) is 1 when a and m are coprime, this can be rewritten as:
    //     a * x + m * y = 1
    // Dividing both sides with m:
    //     (a * x) / m = 1 / m
    // Which means that
    //     (a * x) % m = 1
    // So that x is the searched after modular multiplicative inverse, which
    // we finally need to make positive if necessary.
    let ExtendedEuclidResult { gcd, x, .. } = extended_euclid(a, m);
    if gcd == 1 {
        if x > 0 { Some(x) } else { Some(x + m) }
    } else {
        None
    }
}

/// Compute X where X fulfill:
///   remainders[i] == T % divisors[i]
/// for all i.
///
/// See <https://en.wikipedia.org/wiki/Chinese_remainder_theorem/>
/// and <https://www.youtube.com/watch?v=ru7mWZJlRQg/>.
fn chinese_remainder(remainders: &[i128], divisors: &[i128]) -> Option<i128> {
    // Start by multiplying all divisors together, to facilitate obtaining
    // other_divisors_multiplied in the loop below:
    let all_divisors_multiplied = divisors.iter().product::<i128>();
    if all_divisors_multiplied == 0 {
        return None;
    }

    // Consider T split into a sum:
    //   T = value[0] + value[1] + ...
    // We want to calculate each term, value[i] to satify
    //   remainders[i] = value[i] % divisors[i]
    // locally without having to care about other indices.
    let mut sum = 0;
    for (&remainder, &divisor) in remainders.iter().zip(divisors) {
        // Start with all other divisors multiplied together
        let other_divisors_multiplied = all_divisors_multiplied / divisor;

        // That is evenly divided by all other divisors, so we can ignore those and focus on
        // finding a multiple of this value that has the sought after remainder value:
        let value_with_one_as_remainder = other_divisors_multiplied
            * modular_multiplicative_inverse(other_divisors_multiplied, divisor)?;
        let fulfilling_value = remainder * value_with_one_as_remainder;

        sum += fulfilling_value;
    }

    Some(sum % all_divisors_multiplied)
}

pub fn solve(input: &Input) -> Result<i128, String> {
    let mut lines = input.text.lines();

    let not_until = lines
        .next()
        .ok_or("Not two lines")?
        .parse::<u32>()
        .map_err(|error| format!("Line 1: Cannot parse number - {error}"))?;

    let bus_ids = lines
        .next()
        .ok_or("Not two lines")?
        .split(',')
        .enumerate()
        .filter_map(|(offset, entry)| {
            if entry == "x" {
                None
            } else {
                entry.parse::<u32>().map_or_else(
                    |_| Some(Err("Line 2: Invalid entry".to_string())),
                    |value| Some(Ok((offset, value))),
                )
            }
        })
        .collect::<Result<Vec<_>, _>>()?;

    if input.is_part_one() {
        // For a bus with id bus_id, the we want to find the one with minimal waiting
        // time, which is given by `bus_id - not_until % bus_id`, since we need to wait
        // a full bus_id cycle, but can subtract the time that has passed,
        // `not_until % bus_id`, for that cycle.
        let (bus_id, wait_time) = bus_ids
            .iter()
            .map(|(_offset, bus_id)| (bus_id, (bus_id - not_until % bus_id) % bus_id))
            .min_by(|&a, &b| a.1.cmp(&b.1))
            .ok_or("No bus ID:s")?;
        Ok(i128::from(*bus_id) * i128::from(wait_time))
    } else {
        // Searching for a time T such that, for every bus_ids[i]:
        //
        //    bus_ids[i] - T % bus_ids[i] == i.
        //
        // Which is the same as:
        //
        //    bus_ids[i] - i == T % bus_ids[i]
        //
        // Formulated differently:
        //
        //    remainders[i] = T % divisors[i]
        //
        // where
        //
        //    remainders[i] := bus_ids[i] - i
        //    divisors[i] := bus_ids[i]
        //
        // Which is what the chinese reminder theorem is about.
        let remainders = bus_ids
            .iter()
            .map(|&(offset, bus_id)| i128::from(bus_id) - offset as i128)
            .collect::<Vec<_>>();

        let divisors = bus_ids
            .iter()
            .map(|&(_offset, bus_id)| i128::from(bus_id))
            .collect::<Vec<_>>();

        chinese_remainder(&remainders, &divisors)
            .ok_or_else(|| "Bus id:s not pairwise coprime".to_string())
    }
}

#[test]
pub fn tests() {
    let example = "939\n7,13,x,x,59,x,31,19";
    test_part_one!(example => 295);
    test_part_two!(example => 1_068_781);

    test_part_one!("100\n10" => 0);

    let real_input = include_str!("day13_input.txt");
    test_part_one!(real_input => 4722);
    test_part_two!(real_input => 825_305_207_525_452);
}
