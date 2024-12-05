use crate::common::array_stack::ArrayStack;
use crate::input::{on_error, Input};

pub fn solve(input: &Input) -> Result<u32, String> {
    let mut page_ordering = [0_u128; 100];
    let mut sum = 0;

    for line in input.text.lines() {
        if let Some((before, after)) = line.split_once('|') {
            let before = parse_page(before)?;
            let after = parse_page(after)?;
            page_ordering[before as usize] |= 1 << (after as u128);
        } else if !line.is_empty() {
            let mut parts = ArrayStack::<100, u8>::new();
            for part in line.split(',') {
                let part = part.parse::<u8>().map_err(|_| on_error())?;
                parts.push(part)?;
            }
            let mut valid = true;
            for (idx, &before) in parts.slice().iter().enumerate() {
                for &after in parts.slice().iter().skip(idx + 1) {
                    if page_ordering[before as usize] & (1 << (after as u128)) == 0 {
                        valid = false;
                    }
                }
            }
            sum += match (input.is_part_one(), valid) {
                (true, true) => parts.elements[parts.len() / 2] as u32,
                (false, false) => {
                    let l = parts.slice_mut();
                    l.sort_unstable_by(|&a, &b| {
                        0.cmp(&(page_ordering[a as usize] & (1 << (b as u128))))
                    });
                    l[l.len() / 2] as u32
                }
                _ => 0,
            };
        }
    }

    Ok(sum)
}

fn parse_page(s: &str) -> Result<u8, String> {
    let n = s.parse::<u8>().map_err(|_| on_error())?;
    if n >= 100 {
        return Err(format!("Too big page number: {n}"));
    }
    Ok(n)
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one_no_allocations, test_part_two_no_allocations};

    let test_input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
    test_part_one_no_allocations!(test_input => 143);
    test_part_two_no_allocations!(test_input => 123);

    let real_input = include_str!("day05_input.txt");
    test_part_one_no_allocations!(real_input => 6612);
    test_part_two_no_allocations!(real_input => 4944);
}
