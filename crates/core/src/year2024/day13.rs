use crate::input::{on_error, Input};

pub fn solve(input: &Input) -> Result<i64, String> {
    let offset = input.part_values(0, 10_000_000_000_000);

    input
        .text
        .split("\n\n")
        .map(|s| {
            let mut parts = s.split(&['+', '=', ',', '\n']);
            let (a_x, a_y) = (parse_it(&mut parts)?, parse_it(&mut parts)?);
            let (b_x, b_y) = (parse_it(&mut parts)?, parse_it(&mut parts)?);
            let (prize_x, prize_y) = (
                parse_it(&mut parts)? + offset,
                parse_it(&mut parts)? + offset,
            );
            // a * a_x + b * b_x = prize_x
            // a * a_y + b * b_y = prize_y
            //    =>
            // a = (prize_x - b * b_x) / a_x
            // b = (prize_y - a * a_y) / b_y
            //    => substitute b in equation for a and vice versa =>
            // a = prize_x / a_x - (prize_y - a * a_y) * b_x / (a_x * b_y)
            //    => multiply both sides with (a_x * b_y) =>
            // (a_x * b_y) * a = prize_x * b_y - prize_y * b_x + a * a_y * b_x
            //    => subtract (a * a_y * b_x) from both sides =>
            // (a_x * b_y - a_y * b_x) * a = prize_x * b_y - prize_y * b_x
            //    => (similarly for b) =>
            let a = (prize_x * b_y - prize_y * b_x) / (a_x * b_y - a_y * b_x);
            let b = (a_x * prize_y - a_y * prize_x) / (a_x * b_y - a_y * b_x);
            Ok((3 * a + b)
                * i64::from(a * a_x + b * b_x == prize_x && a * a_y + b * b_y == prize_y))
        })
        .sum()
}

fn parse_it<'a, I: Iterator<Item = &'a str>>(it: &mut I) -> Result<i64, String> {
    it.nth(1)
        .ok_or_else(on_error)?
        .parse::<i64>()
        .map_err(|_| on_error())
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one_no_allocations, test_part_two_no_allocations};

    let test_input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
    test_part_one_no_allocations!(test_input => 480);

    let real_input = include_str!("day13_input.txt");
    test_part_one_no_allocations!(real_input => 37_297);
    test_part_two_no_allocations!(real_input => 83_197_086_729_371);
}
