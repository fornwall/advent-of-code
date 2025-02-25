use crate::input::{Input, on_error};

pub fn solve(input: &Input) -> Result<u64, String> {
    const HASH_BASE: u32 = 18;
    const HASH_MAX_VAL: usize =
        (HASH_BASE.pow(4) + HASH_BASE.pow(3) + HASH_BASE.pow(2) + HASH_BASE + 1) as usize;

    let part2 = input.is_part_two();

    let mut secret_num_sum = 0;
    let mut sequence_buys = [0_u16; HASH_MAX_VAL];

    for line in input.text.lines() {
        let mut this_sequence_buys = [false; HASH_MAX_VAL];
        let mut changes = 0_u32;
        let mut last_banana_value = 0;

        let mut n: u64 = line.parse().map_err(|_| on_error())?;
        for i in 0..2000 {
            n = (n ^ (n << 6)) & 16777215;
            n ^= n >> 5;
            n = (n ^ (n << 11)) & 16777215;

            let this_banana_value = (n % 10) as i8;
            let banan_value_change = this_banana_value - last_banana_value;
            last_banana_value = this_banana_value;

            changes =
                ((changes << 5) | ((banan_value_change + 9) as u32)) & 0b11111_11111_11111_11111;

            if i >= 3 && part2 {
                let hash_val = (HASH_BASE.pow(3) * ((changes >> 15) & 0b11111)
                    + HASH_BASE.pow(2) * ((changes >> 10) & 0b11111)
                    + HASH_BASE * ((changes >> 5) & 0b11111)
                    + (changes & 0b11111)) as usize;
                if !this_sequence_buys[hash_val] {
                    sequence_buys[hash_val] += this_banana_value as u16;
                    this_sequence_buys[hash_val] = true;
                }
            }
        }
        secret_num_sum += n;
    }

    Ok(if input.is_part_one() {
        secret_num_sum
    } else {
        *sequence_buys.iter().max().unwrap_or(&0) as u64
    })
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one_no_allocations, test_part_two_no_allocations};

    let test_input: &str = "1
10
100
2024";
    test_part_one_no_allocations!(test_input => 37_327_623);
    let test_input: &str = "1
2
3
2024";
    test_part_two_no_allocations!(test_input => 23);

    let real_input = include_str!("day22_input.txt");
    test_part_one_no_allocations!(real_input => 14_623_556_510);
    test_part_two_no_allocations!(real_input => 1701);
}
