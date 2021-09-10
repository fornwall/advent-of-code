use crate::input::Input;
use std::collections::{HashMap, HashSet};

const MAX_DEPTH: u32 = 100;

struct BagEntry<'a> {
    amount: u32,
    bag_type: &'a str,
}

fn insert_ancestors<'a>(
    child_to_parent: &'a HashMap<&'a str, Vec<&str>>,
    child_bag_type: &'a str,
    ancestors: &mut HashSet<&'a str>,
    height: u32,
) -> Result<(), String> {
    if height > MAX_DEPTH {
        return Err(format!(
            "Too deep tree (possibly recursive) - bailing at depth {}",
            MAX_DEPTH
        ));
    }

    if let Some(parents) = child_to_parent.get(child_bag_type) {
        ancestors.extend(parents);
        for parent in parents {
            insert_ancestors(child_to_parent, parent, ancestors, height + 1)?;
        }
    }
    Ok(())
}

fn count_total_bags<'a>(
    reactions: &'a HashMap<&'a str, Vec<BagEntry>>,
    bag_type: &'a str,
    depth: u32,
) -> Result<u32, String> {
    if depth > MAX_DEPTH {
        return Err(format!(
            "Too deep tree (possibly recursive) - bailing at depth {}",
            MAX_DEPTH
        ));
    }

    reactions
        .get(bag_type)
        .map_or(Ok(0_u32), |resulting_entries| {
            resulting_entries
                .iter()
                .map(|entry| {
                    count_total_bags(reactions, entry.bag_type, depth + 1)
                        .map(|value| entry.amount * (value + 1))
                })
                .sum::<Result<u32, String>>()
        })
}

pub fn solve(input: &mut Input) -> Result<u32, String> {
    let mut reactions: HashMap<&str, Vec<BagEntry>> = HashMap::new();
    let mut child_to_parent: HashMap<&str, Vec<&str>> = HashMap::new();

    for (line_idx, line) in input
        .text
        .lines()
        .enumerate()
        .filter(|(_line_idx, line)| !line.contains("no other bags"))
    {
        let on_error = || format!("Line {}: Invalid format", line_idx + 1);

        let (from_bag, to_parts) = line
            .strip_suffix('.')
            .and_then(|line| line.split_once(" bags contain "))
            .ok_or_else(on_error)?;

        let mut children_entries = Vec::new();

        for to_part in to_parts.split(", ") {
            let mut amount_and_bag_type = to_part.splitn(2, ' ');
            let amount = amount_and_bag_type
                .next()
                .ok_or_else(on_error)?
                .parse::<u32>()
                .map_err(|_| on_error())?;
            let bag_type: &str = amount_and_bag_type.next().ok_or_else(on_error)?;
            let (bag_type, _) = bag_type.rsplit_once(' ').ok_or_else(on_error)?;

            if input.is_part_one() {
                child_to_parent
                    .entry(bag_type)
                    .or_insert_with(Vec::new)
                    .push(from_bag);
            } else {
                children_entries.push(BagEntry { amount, bag_type });
            }
        }
        reactions.insert(from_bag, children_entries);
    }

    Ok(if input.is_part_one() {
        let mut gold_ancestors: HashSet<&str> = HashSet::new();
        insert_ancestors(&child_to_parent, "shiny gold", &mut gold_ancestors, 0)?;
        gold_ancestors.len() as u32
    } else {
        count_total_bags(&reactions, "shiny gold", 0)?
    })
}

#[test]
pub fn tests() {
    use crate::{test_part_one, test_part_two};

    let example = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
    test_part_one!(example => 4);
    test_part_two!(example => 32);

    let real_input = include_str!("day07_input.txt");
    test_part_one!(real_input => 229);
    test_part_two!(real_input => 6683);
}
