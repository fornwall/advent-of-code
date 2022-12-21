use std::collections::HashMap;

use crate::input::Input;
use crate::year2022::rational::Rational;

pub fn solve(input: &mut Input) -> Result<i64, String> {
    let (root_id, human_id, monkeys) = MonkeyAction::parse(input.text).ok_or("Invalid input")?;

    if input.is_part_one() {
        eval(&monkeys, root_id, MonkeyId::MAX).number
    } else {
        let MonkeyAction::Operation {
            first_operand,
            operator: _,
            second_operand,
        } = monkeys[root_id as usize] else {
            return Err("The root monkey does not do any action".to_string());
        };

        let v1 = eval(&monkeys, first_operand, human_id);
        let v2 = eval(&monkeys, second_operand, human_id);
        // v1.number + h * v1.human_output = v2.number + h * v2.human_output
        // =>
        // h * (v1.human_output - v2.human_output) = v2.number - v1.number
        // =>
        (v2.number - v1.number) / (v1.human_output - v2.human_output)
    }
    .int_value()
    .ok_or_else(|| "No solution found".to_string())
}

#[derive(Copy, Clone)]
struct Value {
    number: Rational,
    human_output: Rational,
}

impl Value {
    fn op(self, operator: u8, other: Self) -> Self {
        match operator {
            b'+' => Self {
                number: self.number + other.number,
                human_output: self.human_output + other.human_output,
            },
            b'-' => Self {
                number: self.number - other.number,
                human_output: self.human_output - other.human_output,
            },
            b'*' => {
                assert!(self.human_output.x == 0 || other.human_output.x == 0);
                Self {
                    number: self.number * other.number,
                    human_output: self.number * other.human_output
                        + self.human_output * other.number,
                }
            }
            _ => {
                assert_eq!(other.human_output.x, 0);
                Self {
                    number: self.number / other.number,
                    human_output: self.human_output / other.number,
                }
            }
        }
    }
}

fn eval(actions: &[MonkeyAction], evaluated_id: MonkeyId, human_idx: MonkeyId) -> Value {
    if evaluated_id == human_idx {
        return Value {
            number: Rational::integer(0),
            human_output: Rational::integer(1),
        };
    }
    match actions[evaluated_id as usize] {
        MonkeyAction::Constant(number) => Value {
            number: Rational::integer(i64::from(number)),
            human_output: Rational::integer(0),
        },
        MonkeyAction::Operation {
            first_operand,
            operator,
            second_operand,
        } => {
            let o1 = eval(actions, first_operand, human_idx);
            let o2 = eval(actions, second_operand, human_idx);
            o1.op(operator, o2)
        }
    }
}

type MonkeyId = u16;

#[derive(Copy, Clone)]
enum MonkeyAction {
    Constant(u16),
    Operation {
        first_operand: MonkeyId,
        operator: u8,
        second_operand: MonkeyId,
    },
}

impl MonkeyAction {
    fn parse<'a>(input: &'a str) -> Option<(MonkeyId, MonkeyId, Vec<Self>)> {
        let mut name_to_id = HashMap::new();
        let mut actions = Vec::with_capacity(40_000);

        let mut human_id = None;
        let mut root_id = None;

        let mut id_of = |name: &'a str| -> u16 {
            let num_monkeys = name_to_id.len();
            *name_to_id.entry(name).or_insert(num_monkeys as u16)
        };

        for line in input.lines() {
            let mut words = line.split(' ');

            let monkey_name = words.next().map(|w| &w[0..w.len() - 1])?;
            let monkey_id = id_of(monkey_name);
            if monkey_name == "humn" {
                human_id = Some(monkey_id);
            } else if monkey_name == "root" {
                root_id = Some(monkey_id);
            }

            let second_word = words.next()?;
            let action = if let Some(third_word) = words.next() {
                let first_operand = id_of(second_word);
                let operator = third_word.as_bytes()[0];
                let second_operand = id_of(words.next()?);
                Self::Operation {
                    first_operand,
                    operator,
                    second_operand,
                }
            } else {
                let number = second_word.parse::<u16>().ok()?;
                Self::Constant(number)
            };

            if actions.len() < (monkey_id + 1) as usize {
                actions.resize((monkey_id + 1) as usize, Self::Constant(999));
            }
            actions[monkey_id as usize] = action;
        }

        Some((root_id?, human_id?, actions))
    }
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    let test_input = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";
    test_part_one!(test_input => 152);
    test_part_two!(test_input => 301);

    let real_input = include_str!("day21_input.txt");
    test_part_one!(real_input => 51_928_383_302_238);
    test_part_two!(real_input => 3_305_669_217_840);
}
