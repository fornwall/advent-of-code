use crate::Input;
use std::collections::HashMap;

type SignalValue = u16;

#[derive(Copy, Clone)]
enum Operation<'a> {
    Assign(&'a str),
    Not(&'a str),
    And(&'a str, &'a str),
    Or(&'a str, &'a str),
    LeftShift(&'a str, &'a str),
    RightShift(&'a str, &'a str),
}

struct Gate<'a> {
    operation: Operation<'a>,
    computed_value: Option<SignalValue>,
}

impl<'a> Gate<'a> {
    const fn new(operation: Operation<'a>) -> Self {
        Gate {
            operation,
            computed_value: None,
        }
    }
}

fn output_of(num_or_wire: &str, gates: &mut HashMap<&str, Gate>) -> SignalValue {
    num_or_wire
        .parse::<SignalValue>()
        .unwrap_or_else(|_| find_output(num_or_wire, gates))
}

fn find_output(wire: &str, gates: &mut HashMap<&str, Gate>) -> SignalValue {
    let gate = gates.get(wire).unwrap();
    match gate.computed_value {
        Some(value) => value,
        None => {
            let signal_value = match gate.operation {
                Operation::Assign(value) => output_of(value, gates),
                Operation::Not(value) => !output_of(value, gates),
                Operation::And(lhs, rhs) => output_of(lhs, gates) & output_of(rhs, gates),
                Operation::Or(lhs, rhs) => output_of(lhs, gates) | output_of(rhs, gates),
                Operation::LeftShift(lhs, rhs) => output_of(lhs, gates) << output_of(rhs, gates),
                Operation::RightShift(lhs, rhs) => output_of(lhs, gates) >> output_of(rhs, gates),
            };

            gates.get_mut(wire).unwrap().computed_value = Some(signal_value);
            signal_value
        }
    }
}

pub fn solve(input: &mut Input) -> Result<SignalValue, String> {
    let mut gates = HashMap::new();

    for line in input.text.lines() {
        let words = line.split(' ').collect::<Vec<&str>>();
        match words.len() {
            3 => {
                // "123 -> x"
                let value = words[0];
                let wire = words[2];
                let gate = Gate::new(Operation::Assign(value));
                gates.insert(wire, gate);
            }
            4 => {
                // "NOT e -> f".
                if !line.starts_with("NOT ") {
                    return Err("Strange NOT line".to_string());
                }
                let negated_value = words[1];
                let wire = words[3];
                let gate = Gate::new(Operation::Not(negated_value));
                gates.insert(wire, gate);
            }
            5 => {
                let first_value = words[0];
                let wire = words[4];
                let second_value = words[2];

                let operation = match words[1] {
                    "AND" => Operation::And(first_value, second_value),
                    "OR" => Operation::Or(first_value, second_value),
                    "LSHIFT" => Operation::LeftShift(first_value, second_value),
                    "RSHIFT" => Operation::RightShift(first_value, second_value),
                    _ => {
                        return Err("Unexpected line".to_string());
                    }
                };
                let gate = Gate::new(operation);
                gates.insert(wire, gate);
            }
            _ => {
                return Err("Invalid input".to_string());
            }
        }
    }

    let value_of_a = find_output("a", &mut gates);
    if input.is_part_one() {
        Ok(value_of_a)
    } else {
        let value_of_a_str = value_of_a.to_string();
        for (_key, value) in gates.iter_mut() {
            value.computed_value = None;
        }
        gates.insert("b", Gate::new(Operation::Assign(value_of_a_str.as_str())));
        Ok(find_output("a", &mut gates))
    }
}

#[test]
pub fn tests() {
    use crate::{test_part_one, test_part_two};

    let real_input = include_str!("day07_input.txt");
    test_part_one!(real_input => 3176);
    test_part_two!(real_input => 14710);
}
