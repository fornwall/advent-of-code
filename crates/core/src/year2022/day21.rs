use std::collections::HashMap;

use crate::input::Input;

pub fn solve(input: &Input) -> Result<i64, String> {
    let (root_id, human_id, mut actions) =
        MonkeyAction::parse(input.text).ok_or("Invalid input")?;

    if input.is_part_one() {
        Ok(eval(&actions, root_id))
    } else {
        let MonkeyAction::Operation {
            lhs: first_operand,
            operator: _,
            rhs: second_operand,
        } = actions[root_id as usize].action else {
            return Err("The root monkey does not do any action".to_string());
        };

        setup_contains_human(&mut actions, root_id);

        let (human_operand, other_operand) = match (
            actions[first_operand as usize].contains_human,
            actions[second_operand as usize].contains_human,
        ) {
            (true, false) => (first_operand, second_operand),
            (false, true) => (second_operand, first_operand),
            _ => {
                return Err(
                    "Root monkey operation does not contain a single side with human output"
                        .to_string(),
                );
            }
        };

        Ok(eval_for_human_output(
            &actions,
            human_operand,
            human_id,
            eval(&actions, other_operand),
        ))
    }
}

fn eval(actions: &[Monkey], evaluated_id: MonkeyId) -> Value {
    match actions[evaluated_id as usize].action {
        MonkeyAction::Constant(number) => Value::from(number),
        MonkeyAction::Operation { lhs, operator, rhs } => {
            let o1 = eval(actions, lhs);
            let o2 = eval(actions, rhs);
            match operator {
                b'+' => o1 + o2,
                b'-' => o1 - o2,
                b'*' => o1 * o2,
                _ => o1 / o2,
            }
        }
    }
}

fn setup_contains_human(actions: &mut [Monkey], evaluated_id: MonkeyId) -> bool {
    let value = if actions[evaluated_id as usize].contains_human {
        true
    } else {
        match actions[evaluated_id as usize].action {
            MonkeyAction::Constant(_) => false,
            MonkeyAction::Operation {
                lhs,
                operator: _,
                rhs,
            } => setup_contains_human(actions, lhs) || setup_contains_human(actions, rhs),
        }
    };
    actions[evaluated_id as usize].contains_human = value;
    value
}

fn eval_for_human_output(
    actions: &[Monkey],
    evaluated_id: MonkeyId,
    human_id: MonkeyId,
    human_output: Value,
) -> Value {
    if evaluated_id == human_id {
        return human_output;
    }

    match actions[evaluated_id as usize].action {
        MonkeyAction::Constant(number) => Value::from(number),
        MonkeyAction::Operation { lhs, operator, rhs } => {
            let lhs_human = actions[lhs as usize].contains_human;
            let rhs_human = actions[rhs as usize].contains_human;

            let new_value = match (operator, lhs_human, rhs_human) {
                (b'+', true, false) => human_output - eval(actions, rhs),
                (b'+', false, true) => human_output - eval(actions, lhs),
                (b'-', true, false) => human_output + eval(actions, rhs),
                (b'-', false, true) => eval(actions, lhs) - human_output,
                (b'*', true, false) => human_output / eval(actions, rhs),
                (b'*', false, true) => human_output / eval(actions, lhs),
                (b'/', true, false) => human_output * eval(actions, rhs),
                _ => eval(actions, lhs) / human_output,
            };

            let evaluated_id = if lhs_human { lhs } else { rhs };
            eval_for_human_output(actions, evaluated_id, human_id, new_value)
        }
    }
}

type Value = i64;
type MonkeyId = u16;

#[derive(Copy, Clone)]
struct Monkey {
    contains_human: bool,
    action: MonkeyAction,
}

#[derive(Copy, Clone)]
enum MonkeyAction {
    Constant(u16),
    Operation {
        lhs: MonkeyId,
        operator: u8,
        rhs: MonkeyId,
    },
}

impl MonkeyAction {
    fn parse<'a>(input: &'a str) -> Option<(MonkeyId, MonkeyId, Vec<Monkey>)> {
        let mut name_to_id = HashMap::with_capacity(3000);
        let mut actions = Vec::with_capacity(3000);

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
            let is_human = monkey_name == "humn";
            if is_human {
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
                    lhs: first_operand,
                    operator,
                    rhs: second_operand,
                }
            } else {
                let number = second_word.parse::<u16>().ok()?;
                Self::Constant(number)
            };
            let monkey = Monkey {
                contains_human: is_human,
                action,
            };

            if actions.len() < (monkey_id + 1) as usize {
                actions.resize(
                    (monkey_id + 1) as usize,
                    Monkey {
                        contains_human: false,
                        action: Self::Constant(0),
                    },
                );
            }
            actions[monkey_id as usize] = monkey;
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
