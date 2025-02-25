use crate::common::array_stack::ArrayStack;
use crate::input::{Input, on_error};

pub fn solve(input: &Input) -> Result<u32, String> {
    let mut safe = 0;
    for line in input.text.lines() {
        let mut parts = ArrayStack::<16, i8>::new();
        for s in line.split(" ") {
            parts.push(s.parse().map_err(|_| on_error())?)?;
        }
        if input.is_part_one() {
            if is_safe(parts.slice().iter().copied()) {
                safe += 1;
            }
        } else {
            for remove_idx in 0..parts.len() {
                let removed = parts
                    .slice()
                    .iter()
                    .enumerate()
                    .filter_map(|(idx, &e)| (idx != remove_idx).then_some(e));
                if is_safe(removed) {
                    safe += 1;
                    break;
                }
            }
        }
    }
    Ok(safe)
}

fn is_safe<I: Iterator<Item = i8>>(iter: I) -> bool {
    let mut last = 0;
    let mut direction = 0;
    iter.enumerate().all(|(idx, val)| {
        if idx > 0 {
            let diff = val.abs_diff(last);
            if !(1..=3).contains(&diff) {
                return false;
            }
            if idx == 1 {
                direction = last - val;
            } else if (last < val) != (direction < 0) {
                return false;
            }
        }
        last = val;
        true
    })
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one_no_allocations, test_part_two_no_allocations};

    let test_input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
    test_part_one_no_allocations!(test_input => 2);
    test_part_two_no_allocations!(test_input => 4);

    let real_input = include_str!("day02_input.txt");
    test_part_one_no_allocations!(real_input => 479);
    test_part_two_no_allocations!(real_input => 531);
}
