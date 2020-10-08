use std::cell::RefCell;
use std::collections::HashSet;

#[derive(Debug)]
struct ArmyGroup {
    id: i32,
    units: i32,
    hit_points: i32,
    attack_damage: i32,
    attack_type: String,
    initiative: i32,
    weaknesses: HashSet<String>,
    immunities: HashSet<String>,
    immune_system: bool,
    attacked_by: i32,
}

impl ArmyGroup {
    const fn is_alive(&self) -> bool {
        self.units > 0
    }

    fn parse(input_string: &str) -> Result<Vec<Self>, String> {
        let mut id_generator = 0;
        let mut immune_system = true;
        let mut groups: Vec<Self> = Vec::new();
        for line in input_string.lines().skip(1) {
            if line == "" {
                // Skip empty line.
            } else if line == "Infection:" {
                immune_system = false;
            } else {
                // "17 units each with 5390 hit points (weak to radiation, bludgeoning) with
                // an attack that does 4507 fire damage at initiative 2".
                let main_parts: Vec<&str> = line.split(|c| c == '(' || c == ')').collect();

                let mut weaknesses = HashSet::new();
                let mut immunities = HashSet::new();
                let units;
                let hit_points;
                let attack_damage;
                let attack_type;
                let initiative;

                if main_parts.len() == 1 {
                    // No parenthesis.
                    let words: Vec<&str> = line.split_whitespace().collect();
                    if words.len() != 18 {
                        return Err("Invalid input".to_string());
                    }
                    units = words[0].parse::<i32>().unwrap();
                    hit_points = words[4].parse::<i32>().unwrap();
                    attack_damage = words[12].parse::<i32>().unwrap();
                    attack_type = words[13].to_string();
                    initiative = words[17].parse::<i32>().unwrap();
                } else {
                    if main_parts.len() != 3 {
                        return Err("Invalid input".to_string());
                    }
                    let before_parentheses: Vec<&str> = main_parts[0].split_whitespace().collect();
                    let after_parentheses: Vec<&str> = main_parts[2].split_whitespace().collect();

                    units = before_parentheses[0].parse::<i32>().unwrap();
                    hit_points = before_parentheses[4].parse::<i32>().unwrap();
                    attack_damage = after_parentheses[5].parse::<i32>().unwrap();
                    attack_type = after_parentheses[6].to_string();
                    initiative = after_parentheses[10].parse::<i32>().unwrap();

                    for part in main_parts[1].split("; ") {
                        if part.starts_with("weak to") {
                            part[8..].split(", ").for_each(|s| {
                                weaknesses.insert(s.to_string());
                            });
                        } else {
                            part[10..].split(", ").for_each(|s| {
                                immunities.insert(s.to_string());
                            });
                        }
                    }
                }

                id_generator += 1;
                let group = Self {
                    id: id_generator,
                    units,
                    hit_points,
                    attack_damage,
                    attack_type,
                    initiative,
                    weaknesses,
                    immunities,
                    immune_system,
                    attacked_by: -1,
                };
                groups.push(group);
            }
        }
        Ok(groups)
    }

    const fn effective_power(&self) -> i32 {
        self.units * self.attack_damage
    }

    fn damage_when_attacking(&self, other_group: &Self) -> i32 {
        self.effective_power()
            * if other_group.immunities.contains(&self.attack_type) {
                0
            } else if other_group.weaknesses.contains(&self.attack_type) {
                2
            } else {
                1
            }
    }

    fn attack(&self, other_group: &mut Self) -> bool {
        let damage = self.damage_when_attacking(other_group);
        let killed_units = damage / other_group.hit_points;
        //println!("{} attacks {}, causing damage: {}", self.id, other_group.id, damage);
        other_group.units -= killed_units;
        killed_units > 0
    }
}

fn execute_battle(groups: Vec<ArmyGroup>) -> Vec<RefCell<ArmyGroup>> {
    let mut groups: Vec<RefCell<ArmyGroup>> = groups.into_iter().map(RefCell::new).collect();
    loop {
        // Target selection.
        groups.sort_by(|a, b| {
            b.borrow()
                .effective_power()
                .cmp(&a.borrow().effective_power())
                .then_with(|| b.borrow().initiative.cmp(&a.borrow().initiative))
        });
        groups.iter().for_each(|g| {
            g.borrow_mut().attacked_by = -1;
        });

        for attacking_group in groups.iter() {
            let attacking_group_id = attacking_group.borrow().id;
            let immune_system = attacking_group.borrow().immune_system;

            if let Some(attacked_group) = groups
                .iter()
                // Only consider attacking non-attacked enemies:
                .filter(|g| {
                    g.borrow().immune_system != immune_system && g.borrow().attacked_by == -1
                })
                // If an attacking group is considering two defending groups to which it would deal equal damage,
                // it chooses to target the defending group with the largest effective power; if there is still a
                // tie, it chooses the defending group with the highest initiative:
                .max_by(|a, b| {
                    let damage_to_a = attacking_group.borrow().damage_when_attacking(&a.borrow());
                    let damage_to_b = attacking_group.borrow().damage_when_attacking(&b.borrow());
                    damage_to_a
                        .cmp(&damage_to_b)
                        .then_with(|| {
                            a.borrow()
                                .effective_power()
                                .cmp(&b.borrow().effective_power())
                        })
                        .then_with(|| a.borrow().initiative.cmp(&b.borrow().initiative))
                })
            {
                // If it cannot deal any defending groups damage, it does not choose a target:
                if attacking_group
                    .borrow()
                    .damage_when_attacking(&attacked_group.borrow())
                    > 0
                {
                    attacked_group.borrow_mut().attacked_by = attacking_group_id;
                }
            }
        }

        // Attacking.
        let mut any_killed_units = false;
        groups.sort_by(|a, b| b.borrow().initiative.cmp(&a.borrow().initiative));
        for attacking_group in groups.iter() {
            if attacking_group.borrow().is_alive() {
                let attacking_group_id = attacking_group.borrow().id;
                for other_group in groups.iter() {
                    let mut other_group_borrowed = other_group.borrow_mut();
                    if other_group_borrowed.attacked_by == attacking_group_id
                        && attacking_group.borrow().attack(&mut other_group_borrowed)
                    {
                        any_killed_units = true;
                    }
                }
            }
        }

        if !any_killed_units {
            break;
        }

        groups.retain(|g| g.borrow().is_alive());

        let alive_sides = groups.iter().fold((false, false), |acc, g| {
            let mut result = acc;
            if g.borrow().immune_system {
                result.0 = true;
            } else {
                result.1 = true;
            }
            result
        });
        if alive_sides != (true, true) {
            break;
        }
    }

    groups
}

pub fn part1(input_string: &str) -> Result<i32, String> {
    let groups = execute_battle(ArmyGroup::parse(input_string)?);
    let result = groups.iter().fold(0, |acc, g| acc + g.borrow().units);
    Ok(result)
}

pub fn part2(input_string: &str) -> Result<i32, String> {
    let mut boost = 1;
    loop {
        let mut groups = ArmyGroup::parse(input_string)?;
        for g in groups.iter_mut() {
            if g.immune_system {
                g.attack_damage += boost;
            }
        }

        let groups = execute_battle(groups);

        if groups.iter().all(|g| g.borrow().immune_system) {
            let result = groups.iter().fold(0, |acc, g| acc + g.borrow().units);
            return Ok(result);
        }

        boost += 1;
    }
}

#[test]
fn tests_part1() {
    assert_eq!(Ok(5216), part1("Immune System:
17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2
989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3

Infection:
801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1
4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4"));

    assert_eq!(Ok(26914), part1(include_str!("day24_input.txt")));
}

#[test]
fn tests_part2() {
    assert_eq!(Ok(862), part2(include_str!("day24_input.txt")));
}
