use crate::common::character_recognition::recognize;
use crate::input::Input;

struct Device {
    register_x: i32,
    cycle: i32,
    accumulated_signal_strength: i32,
    screen: [bool; 240],
}

impl Device {
    const fn new() -> Self {
        Self {
            register_x: 1,
            cycle: 1,
            accumulated_signal_strength: 0,
            screen: [false; 240],
        }
    }

    fn on_cycle(&mut self, value_to_add: i32) {
        if ((self.cycle - 1) % 40).abs_diff(self.register_x) <= 1 {
            self.screen[(self.cycle - 1) as usize] = true;
        }

        self.cycle += 1;
        self.register_x += value_to_add;

        if (self.cycle - 20) % 40 == 0 {
            self.accumulated_signal_strength += self.register_x * self.cycle;
        }
    }
}

pub fn solve(input: &mut Input) -> Result<String, String> {
    let mut device = Device::new();

    for line in input.text.lines() {
        device.on_cycle(0);

        if line != "noop" {
            let value_to_add = line
                .split(' ')
                .nth(1)
                .and_then(|num| num.parse::<i32>().ok())
                .ok_or_else(|| "Invalid input".to_string())?;
            device.on_cycle(value_to_add);
        };
    }

    if input.is_part_one() {
        Ok(device.accumulated_signal_strength.to_string())
    } else {
        recognize(&device.screen)
    }
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    let test_input = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
    test_part_one!(test_input => "13140".to_string());

    let real_input = include_str!("day10_input.txt");
    test_part_one!(real_input => "12740".to_string());
    test_part_two!(real_input => "RBPARAGF".to_string());
}
