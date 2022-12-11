use crate::input::Input;

type WorryType = u64;

#[derive(Copy, Clone, Debug)]
enum Operation {
    Add(WorryType),
    AddOld,
    Multiply(WorryType),
    MultiplyOld,
}

impl Operation {
    const fn apply(self, value: WorryType) -> WorryType {
        match self {
            Self::Add(operand) => value + operand,
            Self::AddOld => value + value,
            Self::Multiply(operand) => value * operand,
            Self::MultiplyOld => value * value,
        }
    }
}

#[derive(Debug)]
struct Monkey {
    items: Vec<WorryType>,
    operation: Operation,
    divider_test: WorryType,
    throws: [u32; 2],
    inspections: u32,
}

impl Monkey {
    fn parse(input: &str) -> Option<Self> {
        let mut lines = input.lines();
        // Sample: "Monkey 1:"
        lines.next()?;
        // Sample: "  Starting items: 90, 79, 97, 52, 90, 94, 71, 70":
        let operation_line = lines.next()?;
        let items = operation_line[18..]
            .split(", ")
            .map(str::parse::<WorryType>)
            .collect::<Result<_, _>>()
            .ok()?;
        // Samples: "  Operation: new = old + 2" and "  Operation: new = old + old":
        let operation_line = lines.next()?;
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
            items,
            divider_test: test,
            throws: [if_false, if_true],
            inspections: 0,
            operation,
        })
    }
}

pub fn solve(input: &mut Input) -> Result<u64, String> {
    let mut monkeys = input
        .text
        .split("\n\n")
        .map(|block| Monkey::parse(block.trim()))
        .collect::<Option<Vec<_>>>()
        .ok_or_else(|| "Unable to parse input".to_string())?;

    let divider_test_product = monkeys
        .iter()
        .map(|m| m.divider_test)
        .product::<WorryType>();
    let relax_divider = input.part_values(3, 1);

    let mut to_push = Vec::with_capacity(16);

    for _round in 0..input.part_values(20, 10_000) {
        for i in 0..monkeys.len() {
            let mut monkey = &mut monkeys[i];
            monkey.inspections += monkey.items.len() as u32;

            let operation = monkey.operation;
            let throws = monkey.throws;
            let test = monkey.divider_test;

            for worry in monkey.items.drain(..) {
                let worry = (operation.apply(worry) % divider_test_product) / relax_divider;
                let to_monkey_idx = throws[usize::from(worry % test == 0)];
                to_push.push((to_monkey_idx, worry));
            }

            for (to, worry) in to_push.drain(..) {
                monkeys[to as usize].items.push(worry);
            }
        }
    }

    monkeys.sort_unstable_by(|a, b| b.inspections.cmp(&a.inspections));
    Ok(u64::from(monkeys[0].inspections) * u64::from(monkeys[1].inspections))
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
