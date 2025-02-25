use crate::common::array_stack::ArrayStack;
use crate::input::{Input, on_error};

pub fn solve(input: &Input) -> Result<u64, String> {
    const MAX_GROUPS: usize = 12;
    const MAX_SPRINGS: usize = 24;
    const MAX_COPIES: usize = 5;

    let num_copies = input.part_values(1, MAX_COPIES);
    let mut sum = 0;

    for line in input.text.lines() {
        let (springs, groups_str) = line.split_once(' ').ok_or_else(on_error)?;
        if springs.len() > MAX_SPRINGS {
            return Err(format!("Too many springs - max {MAX_SPRINGS} supported"));
        }
        let (damaged, unknown) =
            springs
                .bytes()
                .enumerate()
                .fold((0_u32, 0_u32), |(d, u), (idx, b)| {
                    (
                        d | (u32::from(b == b'#') * (1 << idx)),
                        u | (u32::from(b == b'?') * (1 << idx)),
                    )
                });
        let (mut damaged, mut unknown) = (u128::from(damaged), u128::from(unknown));

        let mut groups = ArrayStack::<{ MAX_GROUPS * MAX_COPIES }, u8>::new();
        for num in groups_str.split(',') {
            groups.push(num.parse::<u8>().map_err(|_| on_error())?)?;
        }
        if groups.len() > MAX_GROUPS {
            return Err("Too many groups".to_string());
        }

        if input.is_part_two() {
            let num_bits = springs.len();
            for copy in 0..(num_copies - 1) {
                // "replace the list of spring conditions with five copies of itself (separated by ?)":
                let offset = num_bits + (num_bits + 1) * copy;
                unknown |= 1 << offset;
                let offset = offset + 1;
                unknown |= (unknown & set_lowest_bits(num_bits as u8)) << offset;
                damaged |= (damaged & set_lowest_bits(num_bits as u8)) << offset;
            }

            let mut orig_groups = ArrayStack::<MAX_GROUPS, u8>::new();
            for &n in groups.slice() {
                orig_groups.push(n)?;
            }
            for _ in 0..(num_copies - 1) {
                for &n in orig_groups.slice() {
                    groups.push(n)?;
                }
            }
        }

        sum += count_alternatives(damaged, unknown, groups.slice());
    }
    Ok(sum)
}

fn set_lowest_bits(n: u8) -> u128 {
    u128::MAX >> (u128::BITS as u16 - u16::from(n))
}

fn count_alternatives(damaged: u128, unknown: u128, groups: &[u8]) -> u64 {
    // Insert unset bit at beginning:
    let damaged = damaged << 1;
    let unknown = unknown << 1;
    // Count the bits we care about
    let num_bits = 128 - damaged.leading_zeros().min(unknown.leading_zeros());

    // alternatives[i] is the number of alternatives, initially
    // with bits set at positions up until the first damaged position
    // - note that we inserted an unset bit above.
    let mut alternatives = [0; 128];
    alternatives[0..(damaged.trailing_zeros() as usize)].fill(1);

    // For each group that is necessary, go over all the positions
    // and carry over possibilities from the previous run at positions
    // where that is possible (once a streak group of possibly damaged
    // positions have been passed, and the streak may end here).
    //
    // That means that after a run for a group, alternatives[i] contains
    // the number of alternatives to end that group at position i.
    for &group in groups {
        let mut new_alternatives = [0; 128];
        let mut longest_possible_set_streak = 0;

        for i in 0..num_bits {
            let is_set = (damaged & (1 << i)) != 0;
            let is_unknown = (unknown & (1 << i)) != 0;

            // If this position is either damaged or unknown, we can continue a streak.
            longest_possible_set_streak =
                u8::from(is_set | is_unknown) * (longest_possible_set_streak + 1);

            new_alternatives[i as usize + 1] = if longest_possible_set_streak >= group
                && (damaged & (1 << (i - u32::from(group))) == 0)
            {
                // It's possible to end a sequence at i, if there is not a damaged one at
                // at position i-group. Note that we do not care about the next bit, which
                // may be damaged, since in that case it won't carry over in this run, and
                // a subsequent run won't use the incorrect value either, since no
                // sequence can start at a damaged position.
                alternatives[i as usize - group as usize]
            } else {
                0
            } + u64::from(!is_set)
                // Carry over the current alternative count (which is only non-zero if this
                // group can have been finished) for non-damaged positions
                // - for damaged positions we need to start anew for this group, and do not
                // care about earlier alternatives in this run.
                * new_alternatives[i as usize];
        }

        alternatives = new_alternatives;
    }

    alternatives[num_bits as usize]
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one_no_allocations, test_part_two_no_allocations};

    test_part_one_no_allocations!("#.....??... 1,1" => 2);

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
