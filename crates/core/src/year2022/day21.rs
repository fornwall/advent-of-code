use std::collections::HashMap;

use crate::input::Input;

pub fn solve(input: &Input) -> Result<i64, String> {
    let (root_id, human_id, mut actions) =
        MonkeyAction::parse(input.text).ok_or("Invalid input")?;

    if is_to_deep(&actions, root_id, 0) {
        return Err("Too deep or recursive tree".to_string());
    }

    if input.is_part_one() {
        Ok(eval(&actions, root_id))
    } else {
        let MonkeyAction::Operation {
            lhs: first_operand,
            rhs: second_operand,
            ..
        } = actions[root_id as usize] else {
            return Err("The root monkey does not do any action".to_string());
        };

        setup_contains_human(&mut actions, root_id, human_id);

        let (human_operand, non_human_operand) = match (
            actions[first_operand as usize].contains_human(),
            actions[second_operand as usize].contains_human(),
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
            eval(&actions, non_human_operand),
        ))
    }
}

fn eval(actions: &[MonkeyAction], evaluated_id: MonkeyId) -> Value {
    match actions[evaluated_id as usize] {
        MonkeyAction::Constant(number) => Value::from(number),
        MonkeyAction::Operation {
            lhs, operator, rhs, ..
        } => {
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

fn setup_contains_human(
    actions: &mut [MonkeyAction],
    evaluated_id: MonkeyId,
    human_id: MonkeyId,
) -> bool {
    let has_human = match actions[evaluated_id as usize] {
        MonkeyAction::Constant(_) => {
            return evaluated_id == human_id;
        }
        MonkeyAction::Operation { lhs, rhs, .. } => {
            setup_contains_human(actions, lhs, human_id)
                || setup_contains_human(actions, rhs, human_id)
        }
    };
    if let MonkeyAction::Operation { contains_human, .. } = &mut actions[evaluated_id as usize] {
        *contains_human = has_human;
    }
    has_human
}

fn eval_for_human_output(
    actions: &[MonkeyAction],
    evaluated_id: MonkeyId,
    human_id: MonkeyId,
    operand_required_value: Value,
) -> Value {
    if evaluated_id == human_id {
        return operand_required_value;
    }

    match actions[evaluated_id as usize] {
        MonkeyAction::Constant(number) => Value::from(number),
        MonkeyAction::Operation {
            lhs, operator, rhs, ..
        } => {
            let lhs_human = lhs == human_id || actions[lhs as usize].contains_human();

            let (human_operand, non_human_operand) =
                if lhs_human { (lhs, rhs) } else { (rhs, lhs) };

            let non_human_value = eval(actions, non_human_operand);

            let human_value = match (operator, lhs_human) {
                // operand_required_value = human_value + non_human_value (commutative) =>
                (b'+', _) => operand_required_value - non_human_value,
                // operand_required_value = human_value * non_human_value (commutative) =>
                (b'*', _) => operand_required_value / non_human_value,
                // operand_required_value = human_value - non_human_value =>
                (b'-', true) => operand_required_value + non_human_value,
                // operand_required_value = non_human_value - human_value =>
                (b'-', false) => non_human_value - operand_required_value,
                // operand_required_value = human_value / non_human_value =>
                (b'/', true) => operand_required_value * non_human_value,
                // operand_required_value = non_human_value / human_value =>
                _ => non_human_value / operand_required_value,
            };

            eval_for_human_output(actions, human_operand, human_id, human_value)
        }
    }
}

type Value = i64;
type MonkeyId = u16;

#[derive(Copy, Clone, Debug)]
enum MonkeyAction {
    Constant(u16),
    Operation {
        lhs: MonkeyId,
        operator: u8,
        rhs: MonkeyId,
        contains_human: bool,
    },
}

impl MonkeyAction {
    fn parse<'a>(input: &'a str) -> Option<(MonkeyId, MonkeyId, Vec<Self>)> {
        let mut name_to_id = HashMap::with_capacity(3000);
        let mut monkeys = Vec::with_capacity(3000);

        let mut human_id = None;
        let mut root_id = None;

        let mut id_of = |name: &'a str| -> u16 {
            let num_monkeys = name_to_id.len();
            *name_to_id.entry(name).or_insert(num_monkeys as u16)
        };

        for line in input.lines() {
            let mut words = line.split(' ');

            let monkey_name = words.next().and_then(|w| {
                if w.len() != 5 {
                    return None;
                }
                Some(&w[0..w.len() - 1])
            })?;
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
                if !matches!(third_word, "+" | "-" | "*" | "/") {
                    return None;
                }
                let operator = third_word.as_bytes()[0];
                let second_operand = id_of(words.next()?);
                Self::Operation {
                    lhs: first_operand,
                    operator,
                    rhs: second_operand,
                    contains_human: false,
                }
            } else {
                let number = second_word.parse::<u16>().ok()?;
                Self::Constant(number)
            };

            if monkeys.len() < (monkey_id + 1) as usize {
                monkeys.resize((monkey_id + 1) as usize, Self::Constant(0));
            }
            monkeys[monkey_id as usize] = action;
        }

        if monkeys.iter().any(|a| {
            matches!(a, &Self::Operation {
                lhs,
                operator: _,
                rhs,
                ..
            } if usize::from(lhs.max(rhs)) >= monkeys.len())
        }) {
            return None;
        }

        Some((root_id?, human_id?, monkeys))
    }

    const fn contains_human(self) -> bool {
        matches!(
            self,
            Self::Operation {
                contains_human: true,
                ..
            }
        )
    }
}

fn is_to_deep(actions: &[MonkeyAction], id: u16, depth: usize) -> bool {
    const MAX_TREE_DEPTH: usize = 100;
    if depth > MAX_TREE_DEPTH {
        return true;
    }
    match actions[id as usize] {
        MonkeyAction::Constant(_) => false,
        MonkeyAction::Operation { lhs, rhs, .. } => {
            is_to_deep(actions, lhs, depth + 1) || is_to_deep(actions, rhs, depth + 1)
        }
    }
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_one_error, test_part_two, test_part_two_error};

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

    let test_input = "root: pppw + sjmn\ndbpl: 5\ncczh: sllz  lgvd";
    test_part_one_error!(test_input => "Invalid input");

    // Non existing "yyyy":
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
drzm: hmdt - yyyy
hmdt: 32";
    test_part_two_error!(test_input => "Invalid input");

    // Recursive
    let test_input = "root: aaaa + bbbb
aaaa: aaaa + aaaa
bbbb: humn + cccc
cccc: 2
humn: 5";
    test_part_one_error!(test_input => "Too deep or recursive tree");
    test_part_two_error!(test_input => "Too deep or recursive tree");

    let test_input = " ";
    test_part_one_error!(test_input => "Invalid input");
}
