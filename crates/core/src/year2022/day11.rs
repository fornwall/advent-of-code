use std::collections::HashMap;

use crate::input::Input;

pub fn solve(input: &Input) -> Result<u64, String> {
    let rounds = input.part_values(20, 10000);
    let use_cache = input.part_values(false, true);

    let mut items = Vec::with_capacity(36);

    let monkeys = input
        .text
        .split("\n\n")
        .enumerate()
        .map(|(monkey_idx, block)| Monkey::parse(monkey_idx as u8, block.trim(), &mut items))
        .collect::<Option<Vec<_>>>()
        .ok_or_else(|| "Unable to parse input".to_string())?;

    let divider_test_product = monkeys
        .iter()
        .map(|m| m.divider_test)
        .product::<WorryType>();

    let mut inspected = vec![0; monkeys.len()];
    let mut seen = HashMap::new();
    let mut inspected_sequence = Vec::new();

    for Item {
        mut owner_idx,
        mut worry,
    } in items
    {
        let mut already_skipped = false;

        let mut round = 0;
        while round < rounds {
            if use_cache && !already_skipped {
                if let Some((cycled_round, cycled_idx)) =
                    seen.insert((owner_idx, worry), (round, inspected_sequence.len()))
                {
                    let steps = round - cycled_round;
                    let cycles = (rounds - round - 1) / steps;
                    inspected_sequence[cycled_idx..]
                        .iter()
                        .for_each(|&monkey| inspected[monkey as usize] += cycles as u64);
                    round += steps * cycles;
                    already_skipped = true;
                }
                inspected_sequence.push(owner_idx);
            }

            inspected[owner_idx as usize] += 1;

            worry = monkeys[owner_idx as usize].operation.apply(worry);
            worry = if input.is_part_one() {
                worry / 3
            } else {
                worry % divider_test_product
            };

            let prev_monkey = owner_idx;
            owner_idx = monkeys[owner_idx as usize].throws
                [usize::from(worry % monkeys[owner_idx as usize].divider_test == 0)]
                as u8;
            round += usize::from(owner_idx < prev_monkey);
        }

        inspected_sequence.clear();
        seen.clear();
    }

    inspected.sort_unstable();
    Ok(inspected[inspected.len() - 2] * inspected[inspected.len() - 1])
}

type WorryType = u64;

#[derive(Copy, Clone)]
enum Operation {
    Add(WorryType),
    AddOld,
    Multiply(WorryType),
    MultiplyOld,
}

impl Operation {
    const fn apply(self, value: WorryType) -> u64 {
        match self {
            Self::Add(operand) => value + operand,
            Self::AddOld => value + value,
            Self::Multiply(operand) => value * operand,
            Self::MultiplyOld => value * value,
        }
    }
}

struct Monkey {
    operation: Operation,
    divider_test: WorryType,
    throws: [u32; 2],
}

impl Monkey {
    fn parse(monkey_idx: u8, input: &str, items: &mut Vec<Item>) -> Option<Self> {
        let mut lines = input.lines();
        // Sample: "Monkey 1:"
        lines.next()?;
        // Sample: "  Starting items: 90, 79, 97, 52, 90, 94, 71, 70":
        let operation_line = lines.next()?;
        if operation_line.len() < 19 {
            return None;
        }
        for item_str in operation_line[18..].split(", ") {
            items.push(Item {
                owner_idx: monkey_idx,
                worry: item_str.parse::<WorryType>().ok()?,
            });
        }
        // Samples: "  Operation: new = old + 2" and "  Operation: new = old + old":
        let operation_line = lines.next()?;
        if operation_line.len() < 26 {
            return None;
        }
        let operation = match (
            operation_line.as_bytes()[23],
            operation_line[25..].parse::<WorryType>().ok(),
        ) {
            (b'+', Some(operand)) => Operation::Add(operand),
            (b'+', None) => Operation::AddOld,
            (b'*', Some(operand)) => Operation::Multiply(operand),
            (b'*', None) => Operation::MultiplyOld,
            _ => {
                return None;
            }
        };
        // Sample: "  Test: divisible by 19":
        let test = lines.next()?[21..].parse::<WorryType>().ok()?;
        // Sample: "    If true: throw to monkey 5":
        let if_true = lines.next()?[29..].parse::<u32>().ok()?;
        // Sample: "    If false: throw to monkey 6":
        let if_false = lines.next()?[30..].parse::<u32>().ok()?;

        Some(Self {
            divider_test: test,
            throws: [if_false, if_true],
            operation,
        })
    }
}

struct Item {
    owner_idx: u8,
    worry: WorryType,
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    let test_input = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";
    test_part_one!(test_input => 10_605);
    test_part_two!(test_input => 2_713_310_158);

    let real_input = include_str!("day11_input.txt");
    test_part_one!(real_input => 64_032);
    test_part_two!(real_input => 12_729_522_272);
}
