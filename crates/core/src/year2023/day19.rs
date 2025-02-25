use crate::common::array_stack::ArrayStack;
use crate::common::id_assigner::IdAssigner;
use crate::input::{Input, on_error};

pub fn solve(input: &Input) -> Result<u64, String> {
    let mut workflows = [Workflow::default(); MAX_WORKFLOWS];
    let mut workflow_id_assigner = WorkflowIdAssigner::new("");
    let mut start_workflow_id = 0;

    let (workflows_str, parts_str) = input.text.split_once("\n\n").ok_or_else(on_error)?;
    for line in workflows_str.lines() {
        let (workflow_name, rules_str) = line.split_once('{').ok_or_else(on_error)?;
        let workflow_id = workflow_id_assigner.id_of(workflow_name)?;
        if workflow_name == "in" {
            start_workflow_id = workflow_id;
        }
        let mut new_rules = Workflow::default();
        for rule_str in rules_str[..rules_str.len() - 1].split(',') {
            if new_rules.num_rules >= MAX_RULES {
                return Err("Too many rules".to_string());
            }
            if let Some((condition_str, outcome_str)) = rule_str.split_once(':') {
                let outcome = Outcome::parse(outcome_str, &mut workflow_id_assigner)?;
                if let Some((xmas_name, value_str)) = condition_str.split_once('<') {
                    let xmas_idx = xmas_name_to_idx(xmas_name);
                    new_rules.rules[new_rules.num_rules] = Rule {
                        xmas_idx,
                        condition: Condition::LessThan(value_str.parse().map_err(|_| on_error())?),
                        outcome,
                    };
                } else if let Some((xmas_name, value_str)) = condition_str.split_once('>') {
                    let xmas_idx = xmas_name_to_idx(xmas_name);
                    new_rules.rules[new_rules.num_rules] = Rule {
                        xmas_idx,
                        condition: Condition::GreaterThan(
                            value_str.parse().map_err(|_| on_error())?,
                        ),
                        outcome,
                    };
                } else {
                    return Err("Invalid input - rule not correct formatted".to_string());
                }
            } else {
                let outcome = Outcome::parse(rule_str, &mut workflow_id_assigner)?;
                new_rules.rules[new_rules.num_rules] = Rule {
                    xmas_idx: 0,
                    condition: Condition::Always,
                    outcome,
                };
            }
            new_rules.num_rules += 1;
        }
        workflows[workflow_id as usize] = new_rules;
    }

    let mut remaining = ArrayStack::<MAX_WORKFLOWS, ([(u16, u16); 4], u16, usize)>::new();
    remaining.push(([(1, 4000); 4], start_workflow_id, 0))?;
    let mut passing = ArrayStack::<MAX_WORKFLOWS, [(u16, u16); 4]>::new();

    while let Some(intervals) = remaining.pop() {
        let (mut xmas, workflow_idx, rule_idx) = intervals;
        if xmas.iter().any(|&(start, end)| start > end) {
            continue;
        }

        let workflow = workflows[workflow_idx as usize];
        let rule = workflow.rules[rule_idx];
        match rule.condition {
            Condition::Always => match rule.outcome {
                Outcome::SendTo(workflow_idx) => remaining.push((xmas, workflow_idx, 0))?,
                Outcome::Accepted => passing.push(xmas)?,
                Outcome::Rejected => continue,
            },
            Condition::LessThan(value) | Condition::GreaterThan(value) => {
                let mut new_xmas = xmas;
                if matches!(rule.condition, Condition::LessThan(_)) {
                    xmas[rule.xmas_idx as usize].0 = value;
                    new_xmas[rule.xmas_idx as usize].1 = value - 1;
                } else {
                    xmas[rule.xmas_idx as usize].1 = value;
                    new_xmas[rule.xmas_idx as usize].0 = value + 1;
                }
                match rule.outcome {
                    Outcome::SendTo(sendto_idx) => remaining.push((new_xmas, sendto_idx, 0))?,
                    Outcome::Accepted => passing.push(new_xmas)?,
                    Outcome::Rejected => {}
                };
                remaining.push((xmas, workflow_idx, rule_idx + 1))?;
            }
        }
    }

    if input.is_part_one() {
        parts_str
            .lines()
            .map(|line| {
                let line = &line[1..(line.len() - 1)];
                let mut xmas = [0; 4];
                for (xmas_idx, part_component_str) in line.split(',').enumerate() {
                    let (_, value_str) = part_component_str.split_once('=').ok_or_else(on_error)?;
                    xmas[xmas_idx] = value_str.parse::<u16>().map_err(|_| on_error())?;
                }
                Ok(
                    if passing
                        .slice()
                        .iter()
                        .any(|p| (0..4).all(|i| (p[i].0..=p[i].1).contains(&xmas[i])))
                    {
                        u64::from(xmas.iter().sum::<u16>())
                    } else {
                        0
                    },
                )
            })
            .sum()
    } else {
        Ok(passing
            .slice()
            .iter()
            .map(|xmas| {
                xmas.iter()
                    .fold(1_u64, |sum, (start, end)| sum * u64::from(end - start + 1))
            })
            .sum())
    }
}

const MAX_WORKFLOWS: usize = 1024;
const MAX_RULES: usize = 5;

type WorkflowIdAssigner<'a> = IdAssigner<'a, MAX_WORKFLOWS, str>;
type XmasIdx = u16;

fn xmas_name_to_idx(name: &str) -> XmasIdx {
    match name {
        "x" => 0,
        "m" => 1,
        "a" => 2,
        _ => 3,
    }
}

#[derive(Copy, Clone, Default, Debug)]
enum Condition {
    GreaterThan(u16),
    LessThan(u16),
    #[default]
    Always,
}

#[derive(Copy, Clone, Default, Debug)]
enum Outcome {
    SendTo(u16),
    #[default]
    Accepted,
    Rejected,
}

impl Outcome {
    fn parse<'a>(
        outcome_str: &'a str,
        workflow_id_assigner: &mut WorkflowIdAssigner<'a>,
    ) -> Result<Self, String> {
        Ok(if outcome_str == "A" {
            Self::Accepted
        } else if outcome_str == "R" {
            Self::Rejected
        } else {
            let outcome_idx = workflow_id_assigner.id_of(outcome_str)?;
            Self::SendTo(outcome_idx)
        })
    }
}

#[derive(Copy, Clone, Default, Debug)]
struct Rule {
    xmas_idx: XmasIdx,
    condition: Condition,
    outcome: Outcome,
}

#[derive(Copy, Clone, Default, Debug)]
struct Workflow {
    rules: [Rule; MAX_RULES],
    num_rules: usize,
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one_no_allocations, test_part_two_no_allocations};

    let test_input = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";
    test_part_one_no_allocations!(test_input => 19_114);
    test_part_two_no_allocations!(test_input => 167_409_079_868_000);

    let real_input = include_str!("day19_input.txt");
    test_part_one_no_allocations!(real_input => 398_527);
    test_part_two_no_allocations!(real_input => 133_973_513_090_020);
}
