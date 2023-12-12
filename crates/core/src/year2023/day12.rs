use crate::common::array_stack::ArrayStack;
use crate::input::{on_error, Input};
use std::collections::HashMap;

pub fn solve(input: &Input) -> Result<u64, String> {
    let num_copies = input.part_values(1, 5);
    let mut sum = 0;

    for line in input.text.lines() {
        let (springs, numbers) = line.split_once(' ').ok_or_else(on_error)?;
        let (mut set, mut unknown) =
            springs
                .bytes()
                .enumerate()
                .fold((0_u128, 0_u128), |(mut s, mut u), (idx, b)| {
                    if b == b'#' {
                        s |= 1 << idx;
                    } else if b == b'?' {
                        u |= 1 << idx;
                    }
                    (s, u)
                });
        let mut nums = ArrayStack::<60, u8>::new();
        for num in numbers.split(',') {
            nums.push(num.parse::<u8>().map_err(|_| on_error())?)?;
        }
        if input.is_part_two() {
            let num_bits = springs.len();
            for copy in 0..(num_copies - 1) {
                // "replace the list of spring conditions with five copies of itself (separated by ?)":
                let offset = num_bits + (num_bits + 1) * copy;
                unknown |= 1 << offset;
                let offset = offset + 1;
                unknown |= (unknown & set_lowest_bits(num_bits as u8)) << offset;
                set |= (set & set_lowest_bits(num_bits as u8)) << offset;
            }

            let mut orig_nums = ArrayStack::<12, u8>::new();
            for &n in nums.slice() {
                orig_nums.push(n)?;
            }
            for _ in 0..(num_copies - 1) {
                for &n in orig_nums.slice() {
                    nums.push(n)?;
                }
            }
        }

        let len = ((springs.len() + 1) * num_copies) as u32 + 1;
        let mut cache = HashMap::new();
        sum += count_alternatives(&mut cache, 0, len, set, 0, unknown, 0, nums.slice());
    }
    Ok(sum)
}

fn set_lowest_bits(n: u8) -> u128 {
    u128::MAX >> (u128::BITS as u16 - u16::from(n))
}

fn count_alternatives(
    cache: &mut HashMap<(u32, u32, usize), u64>,
    bit_offset: u32,
    num_bits: u32,
    set: u128,
    set_streak: u32,
    unknown: u128,
    mut numbers_offset: usize,
    numbers: &[u8],
) -> u64 {
    let cache_entry = (bit_offset, set_streak, numbers_offset);
    if let Some(val) = cache.get(&cache_entry) {
        return *val;
    }
    let result = if bit_offset == num_bits {
        return u64::from(numbers_offset == numbers.len());
    } else {
        let is_set = set & (1 << bit_offset) != 0;
        let is_unknown = unknown & (1 << bit_offset) != 0;

        // Set in stone.
        if is_set {
            count_alternatives(
                cache,
                bit_offset + 1,
                num_bits,
                set,
                set_streak + 1,
                unknown,
                numbers_offset,
                numbers,
            )
        } else {
            // Possibly set from unknown.
            let initial = if is_unknown {
                count_alternatives(
                    cache,
                    bit_offset + 1,
                    num_bits,
                    set,
                    set_streak + 1,
                    unknown,
                    numbers_offset,
                    numbers,
                )
            } else {
                0
            };

            // Not set.
            if set_streak != 0 {
                if numbers_offset == numbers.len() {
                    return initial;
                }
                let num = numbers[numbers_offset];
                if u32::from(num) != set_streak {
                    return initial;
                }
                numbers_offset += 1;
            }
            initial
                + count_alternatives(
                    cache,
                    bit_offset + 1,
                    num_bits,
                    set,
                    0,
                    unknown,
                    numbers_offset,
                    numbers,
                )
        }
    };
    cache.insert(cache_entry, result);
    result
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one_no_allocations, test_part_two_no_allocations};

    let test_input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
    test_part_one_no_allocations!(test_input => 21);
    test_part_two_no_allocations!(test_input => 525_152);

    let real_input = include_str!("day12_input.txt");
    test_part_one_no_allocations!(real_input => 8419);
    test_part_two_no_allocations!(real_input => 160_500_973_317_706);
}
