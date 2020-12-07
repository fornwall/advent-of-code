use crate::input::Input;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct BagEntry<'a> {
    amount: u32,
    bag_type: &'a str,
}

fn insert_ancestors<'a>(
    child_to_parent: &'a HashMap<&'a str, Vec<&str>>,
    child_bag_type: &'a str,
    ancestors: &mut HashSet<&'a str>,
) {
    if let Some(parents) = child_to_parent.get(child_bag_type) {
        ancestors.extend(parents);
        for parent in parents {
            insert_ancestors(child_to_parent, parent, ancestors);
        }
    }
}

fn count_total_bags<'a>(reactions: &'a HashMap<&'a str, Vec<BagEntry>>, bag_type: &'a str) -> u32 {
    reactions
        .get(bag_type)
        .map(|resulting_entries| {
            resulting_entries
                .iter()
                .map(|entry| entry.amount * (count_total_bags(reactions, entry.bag_type) + 1))
                .sum::<u32>()
        })
        .unwrap_or(0_u32)
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

        let mut parts = line.split(" bags contain ");
        let from_bag = parts.next().ok_or_else(on_error)?;

        let mut to = Vec::new();
        let to_part = parts
            .next()
            .ok_or_else(on_error)?
            .strip_suffix('.')
            .ok_or_else(on_error)?;

        for to_part in to_part.split(", ") {
            let mut amount_and_bag_type = to_part.splitn(2, ' ');
            let amount = amount_and_bag_type
                .next()
                .ok_or_else(on_error)?
                .parse::<u32>()
                .map_err(|_| on_error())?;
            let bag_type: &str = amount_and_bag_type.next().ok_or_else(on_error)?;
            let bag_type = bag_type.rsplitn(2, ' ').nth(1).ok_or_else(on_error)?;

            if input.is_part_one() {
                child_to_parent
                    .entry(bag_type)
                    .or_insert(Vec::new())
                    .push(from_bag);
            } else {
                to.push(BagEntry { amount, bag_type });
            }
        }
        reactions.insert(from_bag, to);
    }

    Ok(if input.is_part_one() {
        let outermost_bags = reactions.keys().copied().collect::<HashSet<&str>>();
        let mut distinct_roots: HashSet<&str> = HashSet::new();
        insert_ancestors(&child_to_parent, "shiny gold", &mut distinct_roots);
        distinct_roots.intersection(&outermost_bags).count() as u32
    } else {
        count_total_bags(&reactions, "shiny gold")
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
