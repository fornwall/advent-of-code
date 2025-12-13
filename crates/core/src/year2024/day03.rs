use crate::input::Input;

pub fn solve(input: &Input) -> Result<u32, String> {
    Ok(if input.is_part_one() {
        add_multiplications(input.text)
    } else {
        input
            .text
            .split("do()")
            .map(|x| x.find("don't()").map(|idx| &x[..idx]).unwrap_or(x))
            .map(add_multiplications)
            .sum()
    })
}

fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}

fn find_next_num(s: &[u8], followed_by: u8) -> Option<(usize, u32)> {
    let mut num = 0;
    for (i, &c) in s.iter().enumerate().take(4.min(s.len())) {
        if i <= 3 && c.is_ascii_digit() {
            num = num * 10 + (s[i] - b'0') as u32;
        } else {
            return (c == followed_by).then_some((i + 1, num));
        }
    }
    None
}

fn add_multiplications(input: &str) -> u32 {
    let mut s = input.as_bytes();
    let mut sum = 0;
    while let Some(idx) = find_subsequence(s, b"mul(") {
        s = &s[(idx + 4)..];
        if let Some((offset, first_num)) = find_next_num(s, b',') {
            s = &s[offset..];
            if let Some((offset, second_num)) = find_next_num(s, b')') {
                s = &s[offset..];
                sum += first_num * second_num;
            }
        }
    }
    sum
}

#[test]
pub fn tests() {
    let test_input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    test_part_one_no_allocations!(test_input => 161);
    let test_input = "xmul(1000,4)";
    test_part_one_no_allocations!(test_input => 0);
    let test_input = "xmul(,4)mul(3,3)";
    test_part_one_no_allocations!(test_input => 9);
    let test_input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    test_part_two_no_allocations!(test_input => 48);
    let test_input = "xmul(2,4)";
    test_part_two_no_allocations!(test_input => 8);

    let real_input = include_str!("day03_input.txt");
    test_part_one_no_allocations!(real_input => 181_345_830);
    test_part_two_no_allocations!(real_input => 98_729_041);
}
