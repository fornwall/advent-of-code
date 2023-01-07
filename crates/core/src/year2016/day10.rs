use crate::input::Input;

type BotId = u8;

#[derive(Copy, Clone)]
enum OnDone {
    GiveTo(BotId),
    OutputTo(u8),
}

#[derive(Copy, Clone)]
struct Bot {
    low_to: OnDone,
    high_to: OnDone,
    received_chip: Option<u8>,
}

impl Default for Bot {
    fn default() -> Self {
        Self {
            low_to: OnDone::OutputTo(0),
            high_to: OnDone::OutputTo(0),
            received_chip: None,
        }
    }
}

fn receive(
    bots: &mut [Bot; 256],
    first_three_outputs: &mut [u8; 3],
    microchip: u8,
    to_bot: BotId,
    part1: bool,
) -> Option<BotId> {
    let mut bot = &mut bots[usize::from(to_bot)];

    if let Some(first_microchip) = bot.received_chip {
        let low_microchip = std::cmp::min(first_microchip, microchip);
        let high_microchip = std::cmp::max(first_microchip, microchip);

        if part1 && (low_microchip, high_microchip) == (17, 61) {
            return Some(to_bot);
        }

        let low_to = bot.low_to;
        let high_to = bot.high_to;

        match low_to {
            OnDone::GiveTo(recipient) => {
                let desired_bot =
                    receive(bots, first_three_outputs, low_microchip, recipient, part1);
                if desired_bot.is_some() {
                    return desired_bot;
                }
            }
            OnDone::OutputTo(output_idx) => {
                if output_idx < 3 {
                    first_three_outputs[output_idx as usize] = low_microchip;
                }
            }
        }

        match high_to {
            OnDone::GiveTo(recipient) => {
                let desired_bot =
                    receive(bots, first_three_outputs, high_microchip, recipient, part1);
                if desired_bot.is_some() {
                    return desired_bot;
                }
            }
            OnDone::OutputTo(output_idx) => {
                if output_idx < 3 {
                    first_three_outputs[output_idx as usize] = high_microchip;
                }
            }
        }
    } else {
        bot.received_chip = Some(microchip);
    }

    None
}

pub fn solve(input: &Input) -> Result<u32, String> {
    let error_mapper = |_| "Invalid input";
    let mut bots = [Bot::default(); 256];
    let mut initial_values = Vec::new();

    for line in input.text.lines() {
        let parts = line.split(' ').collect::<Vec<_>>();
        if parts[0] == "value" {
            // "value X goes to bot Y"
            let value = parts[1].parse::<u8>().map_err(error_mapper)?;
            let to_bot_id = parts[5].parse::<BotId>().map_err(error_mapper)?;
            initial_values.push((value, to_bot_id));
        } else if parts[0] == "bot" {
            // "bot X gives low to bot|output Y and high to bot|output Z"
            let bot_id = parts[1].parse::<u8>().map_err(error_mapper)?;
            let low_to_number = parts[6].parse::<u8>().map_err(error_mapper)?;
            let high_to_number = parts[11].parse::<u8>().map_err(error_mapper)?;

            let low_to = if parts[5] == "bot" {
                OnDone::GiveTo(low_to_number)
            } else {
                OnDone::OutputTo(low_to_number)
            };

            let high_to = if parts[10] == "bot" {
                OnDone::GiveTo(high_to_number)
            } else {
                OnDone::OutputTo(high_to_number)
            };

            let bot = Bot {
                low_to,
                high_to,
                received_chip: None,
            };

            bots[usize::from(bot_id)] = bot;
        } else {
            return Err("Invalid input".to_string());
        }
    }

    let mut first_three_outputs = [0_u8; 3];
    for &(value, to_bot_id) in &initial_values {
        if let Some(desired_bot_id) = receive(
            &mut bots,
            &mut first_three_outputs,
            value,
            to_bot_id,
            input.is_part_one(),
        ) {
            return Ok(u32::from(desired_bot_id));
        }
    }

    if input.is_part_one() {
        Err("Not bot comparing chips 17 and 61".to_string())
    } else {
        Ok(first_three_outputs.iter().map(|&v| u32::from(v)).product())
    }
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    let real_input = include_str!("day10_input.txt");
    test_part_one!(real_input => 98);
    test_part_two!(real_input => 4042);
}
