use crate::input::Input;
use std::collections::HashMap;

#[derive(Debug)]
struct BagEntry<'a> {
    amount: u32,
    bag_type: &'a str,
}

type BagId = usize;

struct BagIdAssigner<'a> {
    bag_names: Vec<&'a str>,
}

impl<'a> BagIdAssigner<'a> {
    fn new() -> Self {
        BagIdAssigner {
            bag_names: Vec::new(),
        }
    }
    fn assign_id(&mut self, bag_name: &'a str) -> BagId {
        self.bag_names.push(bag_name);
        self.bag_names.len()
    }
}

fn can_contain_shiny_gold(reactions: &HashMap<&str, Vec<BagEntry>>, bag_type: &str) -> bool {
    reactions
        .get(bag_type)
        .map(|resulting_entries| {
            resulting_entries.iter().any(|entry| {
                entry.bag_type == "shiny gold" || can_contain_shiny_gold(reactions, entry.bag_type)
            })
        })
        .unwrap_or(false)
}

fn count_total_bags(reactions: &HashMap<&str, Vec<BagEntry>>, bag_type: &str) -> u32 {
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

    for line in input.text.lines() {
        let mut parts = line.split(" bags contain ");
        let from_bag = parts.next().unwrap();
        let to_part = parts.next().unwrap().strip_suffix('.').unwrap();

        let mut to = Vec::new();
        if to_part != "no other bags" {
            for to_part in to_part.split(", ") {
                let mut amount_and_bag_type = to_part.splitn(2, ' ');
                let amount = amount_and_bag_type.next().unwrap().parse::<u32>().unwrap();
                let bag_type: &str = amount_and_bag_type.next().unwrap();
                let bag_type = if amount > 1 {
                    bag_type.strip_suffix(" bags").unwrap()
                } else {
                    bag_type.strip_suffix(" bag").unwrap()
                };
                to.push(BagEntry { amount, bag_type });
            }
            reactions.insert(from_bag, to);
        }
    }

    Ok(if input.is_part_one() {
        reactions
            .keys()
            .filter(|bag_type| can_contain_shiny_gold(&reactions, bag_type))
            .count() as u32
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
