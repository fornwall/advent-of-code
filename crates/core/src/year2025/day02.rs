use crate::input::{Input, on_error};

#[allow(clippy::zero_prefixed_literal)]
pub fn solve(input: &Input) -> Result<u64, String> {
    let part_two = input.is_part_two();
    let mut result = 0;
    for line in input.text.split(",") {
        let mut parts = line.split("-");
        let first_part = parts
            .next()
            .ok_or_else(on_error)?
            .parse()
            .map_err(|_| on_error())?;
        let second_part = parts
            .next()
            .ok_or_else(on_error)?
            .parse()
            .map_err(|_| on_error())?;

        for x in first_part..=second_part {
            let num_digits = num_digits(x);
            let is_invalid_id = match num_digits {
                1 => false,
                2 => x % /* one digits repeated twice */ 1_1 == 0,
                3 => part_two && x % /* all digits same */ 111 == 0,
                4 => {
                    x % /* two digits repeated twice */ 01_01 == 0
                }
                5 => part_two && x % /* all digits same */ 11111 == 0,
                6 => {
                    x % /* three digits repeated twice */ 001_001 == 0
                        || (part_two && (x % /* two digits repeated three times */ 01_01_01 == 0))
                }
                7 => part_two && x % /* all digits same */ 1_111_111 == 0,
                8 => {
                    x % /* four digits repeated twice */ 0001_0001 == 0
                        || (part_two && (x % /* two digits repeated four times */ 01_01_01_01 == 0))
                }
                9 => {
                    part_two && (x % /* three digits repeated thrice */ 001_001_001 == 0)
                }
                10 => {
                    x % /* five digits repeated twice */ 00001_00001 == 0
                        || (part_two
                            && (x % /* two digits repeated five times */ 01_01_01_01_01 == 0))
                }
                _ => {
                    return Err(format!("Number too large: {}", x));
                }
            };
            result += u64::from(is_invalid_id) * x;
        }
    }
    Ok(result)
}

fn num_digits(integer: u64) -> u32 {
    integer.checked_ilog10().unwrap_or(0) + 1
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one_no_allocations, test_part_two_no_allocations};

    assert_eq!(num_digits(0), 1);
    assert_eq!(num_digits(1), 1);
    assert_eq!(num_digits(9), 1);
    assert_eq!(num_digits(10), 2);
    assert_eq!(num_digits(99), 2);
    assert_eq!(num_digits(100), 3);
    assert_eq!(num_digits(999), 3);

    let test_input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
    test_part_one_no_allocations!(test_input => 1_227_775_554);
    test_part_two_no_allocations!(test_input => 4_174_379_265);

    let real_input = include_str!("day02_input.txt");
    test_part_one_no_allocations!(real_input => 30_323_879_646);
    test_part_two_no_allocations!(real_input => 43_872_163_557);
}
