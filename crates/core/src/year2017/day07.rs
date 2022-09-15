use crate::input::Input;
use std::collections::{HashMap, HashSet};

type ProgramId = usize;

#[derive(Clone)]
struct Program<'a> {
    name: &'a str,
    weight: u32,
    children: Vec<ProgramId>,
}

struct ProgramTree<'a> {
    nodes: Vec<Program<'a>>,
    root_node: ProgramId,
}

impl<'a> ProgramTree<'a> {
    fn total_weight(&self, program_id: ProgramId) -> u32 {
        let program = &self.nodes[program_id];
        program.weight
            + program
                .children
                .iter()
                .map(|&child| self.total_weight(child))
                .sum::<u32>()
    }
}

impl<'a> ProgramTree<'a> {
    fn parse(input_string: &'a str) -> Result<ProgramTree, String> {
        let mut nodes = Vec::new();
        let mut name_to_node: HashMap<&str, ProgramId> = HashMap::new();

        for (line_index, line) in input_string.lines().enumerate() {
            let parts: Vec<&str> = line.split(" -> ").collect();
            let (name, weight_str) = parts[0].split_once(' ').ok_or_else(|| {
                format!(
                    "Line {}: Invalid format, expected '$NAME ($WEIGHT)'",
                    line_index + 1
                )
            })?;
            let weight = weight_str
                .replace(['(', ')'], "")
                .parse::<u32>()
                .map_err(|error| format!("Line {}: Invalid weight ({})", line_index + 1, error))?;

            let program_id = nodes.len();
            let program = Program {
                name,
                weight,
                children: Vec::new(),
            };
            nodes.push(program);
            name_to_node.insert(name, program_id);
        }

        for line in input_string.lines() {
            let parts: Vec<&str> = line.split(" -> ").collect();
            let name_weight_parts: Vec<&str> = parts[0].split(' ').collect();
            if parts.len() == 2 {
                let name = name_weight_parts[0];
                let parent = *name_to_node
                    .get_mut(name)
                    .ok_or("Internal error - no node for name")?;
                for child_name in parts[1].trim().split(", ") {
                    let child_ref = *name_to_node
                        .get(child_name)
                        .ok_or("Invalid input - no node for name")?;
                    nodes[parent].children.push(child_ref);
                }
            }
        }

        let all_program_ids: HashSet<&ProgramId> = name_to_node.values().collect();
        let children: HashSet<&ProgramId> = name_to_node
            .values()
            .flat_map(|&child_program_id| nodes[child_program_id].children.iter())
            .collect();

        let roots: Vec<&&ProgramId> = all_program_ids.difference(&children).collect();
        let root_node = **roots[0];
        if roots.len() == 1 {
            Ok(Self { nodes, root_node })
        } else {
            Err("No single root found".to_string())
        }
    }
}

pub fn solve(input: &mut Input) -> Result<String, String> {
    let tree = ProgramTree::parse(input.text)?;
    if input.is_part_one() {
        Ok(tree.nodes[tree.root_node].name.to_string())
    } else {
        fixup_weight(tree.root_node, &tree)
            .map(|value| value.to_string())
            .ok_or_else(|| "No solution found".to_string())
    }
}

fn fixup_weight(program_id: ProgramId, tree: &ProgramTree) -> Option<u32> {
    let program = &tree.nodes[program_id];
    if program.children.len() > 1 {
        if let (Some(lone_weight), desired_weight) = program
            .children
            .iter()
            .fold(HashMap::new(), |mut acc, &child_id| {
                *acc.entry(tree.total_weight(child_id)).or_insert(0) += 1;
                acc
            })
            .iter()
            .fold((None, 0), |acc, (&weight, &occurrences)| {
                if occurrences == 1 {
                    (Some(weight), acc.1)
                } else {
                    (acc.0, weight)
                }
            })
        {
            if let Some(&child_id) = program
                .children
                .iter()
                .find(|&&p| tree.total_weight(p) == lone_weight)
            {
                return fixup_weight(child_id, tree).or_else(|| {
                    let total_weight = tree.total_weight(child_id);
                    let child = &tree.nodes[child_id];
                    Some(desired_weight - (total_weight - child.weight))
                });
            }
        }
    } else if !program.children.is_empty() {
        if let Some(value) = fixup_weight(program.children[0], tree) {
            return Some(value);
        }
    }
    None
}

#[test]
fn tests() {
    use crate::input::{test_part_one, test_part_two};
    let real_input = include_str!("day07_input.txt");
    test_part_one!(real_input => "veboyvy".to_string());
    test_part_two!(real_input => "749".to_string());
}
