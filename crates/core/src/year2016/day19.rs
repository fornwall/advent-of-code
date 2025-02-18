use crate::input::Input;

pub fn solve(input: &Input) -> Result<u32, String> {
    let n = u32::from(
        input
            .text
            .parse::<std::num::NonZeroU32>()
            .map_err(|e| format!("Invalid number of elves: {e}"))?,
    );

    if input.is_part_one() {
        // See "The Josephus Problem - Numberphile": https://www.youtube.com/watch?v=uCsD3ZGzMgE
        let most_significant_bit = 1 << (u32::BITS - n.leading_zeros() - 1);
        let with_msb_cleared = n & !most_significant_bit;
        let with_lsb_added = (with_msb_cleared << 1) | 1;
        Ok(with_lsb_added)
    } else {
        let power_three = 3_u32.pow(n.ilog(3));
        if n == power_three {
            Ok(n)
        } else if n - power_three <= power_three {
            Ok(n - power_three)
        } else {
            Ok(2 * n - 3 * power_three)
        }
    }
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    test_part_one!("5" => 3);
    test_part_two!("5" => 2);

    let real_input = include_str!("day19_input.txt");
    test_part_one!(real_input => 1_808_357);
    test_part_two!(real_input => 1_407_007);
}
