use crate::input::Input;

pub fn solve(input: &mut Input) -> Result<u32, String> {
    let minutes = input.part_values(24, 32);
    let max_blueprints = input.part_values(64, 3);

    let blueprints = Blueprint::parse(input.text);

    let max_geodes = blueprints
        .take(max_blueprints)
        .map(|blueprint| State::most_geodes_opened(&blueprint, minutes));

    Ok(if input.is_part_one() {
        max_geodes
            .enumerate()
            .map(|(offset, geodes)| (offset + 1) as u32 * geodes)
            .sum()
    } else {
        max_geodes.product()
    })
}

struct Blueprint {
    ore_cost_ore: u32,
    clay_cost_ore: u32,
    obsidian_cost_ore: u32,
    obsidian_cost_clay: u32,
    geode_cost_ore: u32,
    geode_cost_obsidian: u32,
}

impl Blueprint {
    fn parse(input: &str) -> impl Iterator<Item = Self> + '_ {
        input.lines().filter_map(|line| {
            let mut words = line.split(' ');
            Some(Self {
                ore_cost_ore: words.nth(6)?.parse::<u32>().ok()?,
                clay_cost_ore: words.nth(5)?.parse::<u32>().ok()?,
                obsidian_cost_ore: words.nth(5)?.parse::<u32>().ok()?,
                obsidian_cost_clay: words.nth(2)?.parse::<u32>().ok()?,
                geode_cost_ore: words.nth(5)?.parse::<u32>().ok()?,
                geode_cost_obsidian: words.nth(2)?.parse::<u32>().ok()?,
            })
        })
    }
}

#[derive(Clone)]
struct State {
    resource_ore: u32,
    robots_ore: u32,
    resource_clay: u32,
    robots_clay: u32,
    resource_obsidian: u32,
    robots_obsidian: u32,
    resource_geode: u32,
    robots_geode: u32,
    could_have_created_ore: bool,
    could_have_created_clay: bool,
    could_have_created_obsidian: bool,
}

impl State {
    fn most_geodes_opened(blueprint: &Blueprint, minutes: u32) -> u32 {
        let max_ore_cost = blueprint
            .ore_cost_ore
            .max(blueprint.clay_cost_ore)
            .max(blueprint.obsidian_cost_ore)
            .max(blueprint.geode_cost_ore);

        let mut initial_state = Self {
            resource_ore: 0,
            robots_ore: 1,
            resource_clay: 0,
            robots_clay: 0,
            resource_obsidian: 0,
            robots_obsidian: 0,
            resource_geode: 0,
            robots_geode: 0,
            could_have_created_ore: false,
            could_have_created_clay: false,
            could_have_created_obsidian: false,
        };

        initial_state.most_geodes_opened_recursive(blueprint, max_ore_cost, minutes)
    }

    fn most_geodes_opened_recursive(
        &mut self,
        blueprint: &Blueprint,
        max_ore_cost: u32,
        remaining_minutes: u32,
    ) -> u32 {
        if remaining_minutes == 1 {
            return self.resource_geode + self.robots_geode;
        }

        let remaining_minutes = remaining_minutes - 1;
        let mut most_geodes = 0;

        for (resource_type, mut new_state) in ALL_RESOURCE_TYPES
            .iter()
            .filter(|&&resource_type| remaining_minutes > 1 || resource_type == ResourceType::Geode)
            .filter(|&&resource_type| {
                self.will_produce_robot(blueprint, resource_type, max_ore_cost)
            })
            .map(|&resource_type| (resource_type, self.produce_robot(blueprint, resource_type)))
        {
            let produced =
                new_state.most_geodes_opened_recursive(blueprint, max_ore_cost, remaining_minutes);
            if resource_type == ResourceType::Geode {
                // Build a geode robot if possible.
                return produced;
            }
            most_geodes = most_geodes.max(produced);
        }

        self.could_have_created_ore = self.resource_ore >= blueprint.ore_cost_ore;
        self.could_have_created_clay = self.resource_ore >= blueprint.clay_cost_ore;
        self.could_have_created_obsidian = self.resource_ore >= blueprint.obsidian_cost_ore
            && self.resource_clay >= blueprint.obsidian_cost_clay;
        self.produce_resources();

        most_geodes.max(self.most_geodes_opened_recursive(
            blueprint,
            max_ore_cost,
            remaining_minutes,
        ))
    }

    fn produce_resources(&mut self) {
        self.resource_ore += self.robots_ore;
        self.resource_clay += self.robots_clay;
        self.resource_obsidian += self.robots_obsidian;
        self.resource_geode += self.robots_geode;
    }

    fn produce_robot(&self, blueprint: &Blueprint, resource_type: ResourceType) -> Self {
        let mut result = self.clone();
        result.produce_resources();
        match resource_type {
            ResourceType::Ore => {
                result.resource_ore -= blueprint.ore_cost_ore;
                result.robots_ore += 1;
            }
            ResourceType::Clay => {
                result.resource_ore -= blueprint.clay_cost_ore;
                result.robots_clay += 1;
            }
            ResourceType::Obsidian => {
                result.resource_ore -= blueprint.obsidian_cost_ore;
                result.resource_clay -= blueprint.obsidian_cost_clay;
                result.robots_obsidian += 1;
            }
            ResourceType::Geode => {
                result.resource_ore -= blueprint.geode_cost_ore;
                result.resource_obsidian -= blueprint.geode_cost_obsidian;
                result.robots_geode += 1;
            }
        }
        result.could_have_created_ore = false;
        result.could_have_created_clay = false;
        result.could_have_created_obsidian = false;
        result
    }

    const fn will_produce_robot(
        &self,
        blueprint: &Blueprint,
        resource_type: ResourceType,
        max_ore_cost: u32,
    ) -> bool {
        match resource_type {
            ResourceType::Ore => {
                !self.could_have_created_ore
                    && self.robots_ore < max_ore_cost
                    && self.resource_ore >= blueprint.ore_cost_ore
            }
            ResourceType::Clay => {
                !self.could_have_created_clay
                    && self.robots_clay < blueprint.obsidian_cost_clay
                    && self.resource_ore >= blueprint.clay_cost_ore
            }
            ResourceType::Obsidian => {
                !self.could_have_created_obsidian
                    && self.robots_obsidian < blueprint.geode_cost_obsidian
                    && self.resource_ore >= blueprint.obsidian_cost_ore
                    && self.resource_clay >= blueprint.obsidian_cost_clay
            }
            ResourceType::Geode => {
                self.resource_ore >= blueprint.geode_cost_ore
                    && self.resource_obsidian >= blueprint.geode_cost_obsidian
            }
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum ResourceType {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

const ALL_RESOURCE_TYPES: [ResourceType; 4] = [
    ResourceType::Geode,
    ResourceType::Obsidian,
    ResourceType::Clay,
    ResourceType::Ore,
];

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    let test_input = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";
    test_part_one!(test_input => 33);
    test_part_two!(test_input => 3472);

    let real_input = include_str!("day19_input.txt");
    test_part_one!(real_input => 1834);
    test_part_two!(real_input => 2240);
}
