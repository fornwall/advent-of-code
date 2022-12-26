use crate::input::Input;

pub fn solve(input: &mut Input) -> Result<String, String> {
    let sum = input
        .text
        .lines()
        .filter(|line| line.len() < 32)
        .map(snafu_to_decimal)
        .sum();
    Ok(decimal_to_snafu(sum))
}

fn snafu_to_decimal(input: &str) -> i64 {
    input.bytes().fold(0, |acc, x| {
        acc * 5
            + match x {
                b'2' => 2,
                b'1' => 1,
                b'0' => 0,
                b'-' => -1,
                _ => -2,
            }
    })
}

fn decimal_to_snafu(mut decimal: i64) -> String {
    #![allow(clippy::unwrap_used)]
    if decimal == 0 {
        return "0".to_string();
    }

    let mut result = Vec::with_capacity(32);
    while decimal > 0 {
        result.push(match decimal % 5 {
            0 => b'0',
            1 => b'1',
            2 => b'2',
            3 => b'=',
            _ => b'-',
        });
        decimal = (decimal + 2) / 5;
    }
    result.reverse();
    String::from_utf8(result).unwrap()
}

#[test]
pub fn tests() {
    use crate::input::test_part_one;

    let test_input = "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";
    test_part_one!(test_input => "2=-1=0".to_string());

    let real_input = include_str!("day25_input.txt");
    test_part_one!(real_input => "2=--=0000-1-0-=1=0=2".to_string());
}
