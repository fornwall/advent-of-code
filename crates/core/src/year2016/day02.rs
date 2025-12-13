use crate::input::Input;

pub fn solve(input: &Input) -> Result<String, String> {
    #![allow(clippy::match_same_arms)]
    let mut code = String::new();
    let mut current_button = '5';

    for line in input.text.lines() {
        for direction in line.chars() {
            if input.is_part_one() {
                current_button = match (direction, current_button) {
                    ('U', '4') => '1',
                    ('U', '5') => '2',
                    ('U', '6') => '3',
                    ('U', '7') => '4',
                    ('U', '8') => '5',
                    ('U', '9') => '6',

                    ('R', '1') => '2',
                    ('R', '2') => '3',
                    ('R', '4') => '5',
                    ('R', '5') => '6',
                    ('R', '7') => '8',
                    ('R', '8') => '9',

                    ('D', '1') => '4',
                    ('D', '2') => '5',
                    ('D', '3') => '6',
                    ('D', '4') => '7',
                    ('D', '5') => '8',
                    ('D', '6') => '9',

                    ('L', '2') => '1',
                    ('L', '5') => '4',
                    ('L', '8') => '7',
                    ('L', '3') => '2',
                    ('L', '6') => '5',
                    ('L', '9') => '8',
                    _ => current_button,
                };
            } else {
                current_button = match (direction, current_button) {
                    ('U', '3') => '1',
                    ('U', '6') => '2',
                    ('U', '7') => '3',
                    ('U', '8') => '4',
                    ('U', 'A') => '6',
                    ('U', 'B') => '7',
                    ('U', 'C') => '8',
                    ('U', 'D') => 'B',

                    ('R', '2') => '3',
                    ('R', '3') => '4',
                    ('R', '5') => '6',
                    ('R', '6') => '7',
                    ('R', '7') => '8',
                    ('R', '8') => '9',
                    ('R', 'A') => 'B',
                    ('R', 'B') => 'C',

                    ('D', '1') => '3',
                    ('D', '2') => '6',
                    ('D', '3') => '7',
                    ('D', '4') => '8',
                    ('D', '6') => 'A',
                    ('D', '7') => 'B',
                    ('D', '8') => 'C',
                    ('D', 'B') => 'D',

                    ('L', '3') => '2',
                    ('L', '4') => '3',
                    ('L', '6') => '5',
                    ('L', '7') => '6',
                    ('L', '8') => '7',
                    ('L', '9') => '8',
                    ('L', 'B') => 'A',
                    ('L', 'C') => 'B',
                    _ => current_button,
                };
            }
        }

        code.push(current_button);
    }

    Ok(code)
}

#[test]
pub fn tests() {
    let real_input = include_str!("day02_input.txt");
    test_part_one!(real_input => "38961".to_string());
    test_part_two!(real_input => "46C92".to_string());
}
