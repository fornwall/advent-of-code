use crate::common::array_stack::ArrayStack;
use crate::input::{Input, on_error};

pub fn solve(input: &Input) -> Result<u64, String> {
    let mut sum = 0;
    for line in input.text.lines() {
        let mut remaining = ArrayStack::<64, u64>::new();

        let mut str_parts = line.split(' ');
        let desired = str_parts
            .next()
            .ok_or_else(on_error)?
            .strip_suffix(':')
            .ok_or_else(on_error)?
            .parse::<u64>()
            .map_err(|_| on_error())?;
        for s in str_parts {
            remaining.push(s.parse().map_err(|_| on_error())?)?;
        }
        if is_possible(desired, remaining.slice(), input.is_part_two()) {
            sum += desired;
        }
    }
    Ok(sum)
}

fn is_possible(desired: u64, remaining: &[u64], concatenate: bool) -> bool {
    let Some((&last, remaining)) = remaining.split_last() else {
        return false;
    };
    if remaining.is_empty() {
        return desired == last;
    }

    if (desired > last && is_possible(desired - last, remaining, concatenate))
        || (desired.is_multiple_of(last) && is_possible(desired / last, remaining, concatenate))
    {
        return true;
    } else if concatenate && desired > last {
        let last_num_digits = last.checked_ilog10().unwrap_or_default() + 1;
        let pow = 10_u64.pow(last_num_digits);
        // desired = concat(remaining, last)
        // =>
        // desired = remaining * pow + last
        // =>
        // (desired - last) / pow = remaining
        if (desired - last).is_multiple_of(pow)
            && is_possible((desired - last) / pow, remaining, concatenate)
        {
            return true;
        }
    }
    false
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one_no_allocations, test_part_two_no_allocations};

    let test_input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
    test_part_one_no_allocations!(test_input => 3749);
    test_part_two_no_allocations!(test_input => 11387);

    let real_input = include_str!("day07_input.txt");
    test_part_one_no_allocations!(real_input => 3_119_088_655_389);
    test_part_two_no_allocations!(real_input => 264_184_041_398_847);
}
