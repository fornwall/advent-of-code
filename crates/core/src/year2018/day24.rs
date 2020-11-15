#[derive(Copy, Clone, PartialEq)]
enum AttackType {
    Bludgeoning,
    Cold,
    Fire,
    Radiation,
    Slashing,
}

impl AttackType {
    fn new(name: &str) -> Result<Self, String> {
        Ok(match name {
            "bludgeoning" => Self::Bludgeoning,
            "cold" => Self::Cold,
            "fire" => Self::Fire,
            "radiation" => Self::Radiation,
            "slashing" => Self::Slashing,
            _ => {
                return Err("Invalid attack type".to_string());
            }
        })
    }
}

struct ArmyGroup {
    id: i32,
    units: i32,
    hit_points: i32,
    attack_damage: i32,
    attack_type: AttackType,
    initiative: i32,
    weaknesses: Vec<AttackType>,
    immunities: Vec<AttackType>,
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

        let error = |_| "Invalid input";

        for line in input_string.lines().skip(1) {
            if line.is_empty() {
                // Skip empty line.
            } else if line == "Infection:" {
                immune_system = false;
            } else {
                // "17 units each with 5390 hit points (weak to radiation, bludgeoning) with
                // an attack that does 4507 fire damage at initiative 2".
                let main_parts: Vec<&str> = line.split(|c| c == '(' || c == ')').collect();

                let mut weaknesses = Vec::new();
                let mut immunities = Vec::new();
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
                    units = words[0].parse::<i32>().map_err(error)?;
                    hit_points = words[4].parse::<i32>().map_err(error)?;
                    attack_damage = words[12].parse::<i32>().map_err(error)?;
                    attack_type = AttackType::new(words[13])?;
                    initiative = words[17].parse::<i32>().map_err(error)?;
                } else {
                    if main_parts.len() != 3 {
                        return Err("Invalid input".to_string());
                    }
                    let before_parentheses: Vec<&str> = main_parts[0].split_whitespace().collect();
                    let after_parentheses: Vec<&str> = main_parts[2].split_whitespace().collect();
                    if before_parentheses.len() != 7 || after_parentheses.len() != 11 {
                        return Err("Invalid input".to_string());
                    }

                    units = before_parentheses[0].parse::<i32>().map_err(error)?;
                    hit_points = before_parentheses[4].parse::<i32>().map_err(error)?;
                    attack_damage = after_parentheses[5].parse::<i32>().map_err(error)?;
                    attack_type = AttackType::new(after_parentheses[6])?;
                    initiative = after_parentheses[10].parse::<i32>().map_err(error)?;

                    for part in main_parts[1].split("; ") {
                        if part.starts_with("weak to") {
                            for s in part[8..].split(", ") {
                                weaknesses.push(AttackType::new(s)?);
                            }
                        } else {
                            for s in part[10..].split(", ") {
                                immunities.push(AttackType::new(s)?);
                            }
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

    fn damage_when_attacked_by(&self, effective_power: i32, attack_type: AttackType) -> i32 {
        effective_power
            * if self.immunities.contains(&attack_type) {
                0
            } else if self.weaknesses.contains(&attack_type) {
                2
            } else {
                1
            }
    }

    fn resolve_attack(&mut self, attacker_effective_power: i32, attack_type: AttackType) -> bool {
        let damage = self.damage_when_attacked_by(attacker_effective_power, attack_type);
        let killed_units = damage / self.hit_points;
        self.units -= killed_units;
        killed_units > 0
    }
}

fn execute_battle(mut groups: Vec<ArmyGroup>) -> Vec<ArmyGroup> {
    loop {
        // Target selection.
        groups.sort_by(|a, b| {
            b.effective_power()
                .cmp(&a.effective_power())
                .then_with(|| b.initiative.cmp(&a.initiative))
        });
        groups.iter_mut().for_each(|g| {
            g.attacked_by = -1;
        });

        for i in 0..groups.len() {
            let (attacker_effective_power, attack_type, attacking_group_id, immune_system) = {
                let g = &groups[i];
                (g.effective_power(), g.attack_type, g.id, g.immune_system)
            };

            if let Some(attacked_group) = groups
                .iter_mut()
                // Only consider attacking non-attacked enemies:
                .filter(|g| g.immune_system != immune_system && g.attacked_by == -1)
                // If an attacking group is considering two defending groups to which it would deal equal damage,
                // it chooses to target the defending group with the largest effective power; if there is still a
                // tie, it chooses the defending group with the highest initiative:
                .max_by(|a, b| {
                    let damage_to_a =
                        a.damage_when_attacked_by(attacker_effective_power, attack_type);
                    let damage_to_b =
                        b.damage_when_attacked_by(attacker_effective_power, attack_type);
                    damage_to_a
                        .cmp(&damage_to_b)
                        .then_with(|| a.effective_power().cmp(&b.effective_power()))
                        .then_with(|| a.initiative.cmp(&b.initiative))
                })
            {
                // If it cannot deal any defending groups damage, it does not choose a target:
                if attacked_group.damage_when_attacked_by(attacker_effective_power, attack_type) > 0
                {
                    attacked_group.attacked_by = attacking_group_id;
                }
            }
        }

        // Attacking.
        let mut any_killed_units = false;
        groups.sort_by(|a, b| b.initiative.cmp(&a.initiative));
        for i in 0..groups.len() {
            let (attacking_group_id, is_alive, effective_power, attack_type) = {
                let g = &groups[i];
                (g.id, g.is_alive(), g.effective_power(), g.attack_type)
            };
            if is_alive {
                for other_group in groups.iter_mut() {
                    if other_group.attacked_by == attacking_group_id
                        && other_group.resolve_attack(effective_power, attack_type)
                    {
                        any_killed_units = true;
                    }
                }
            }
        }

        if !any_killed_units {
            break;
        }

        groups.retain(|g| g.is_alive());

        let alive_sides = groups.iter().fold((false, false), |acc, g| {
            let mut result = acc;
            if g.immune_system {
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
    let result = groups.iter().fold(0, |acc, g| acc + g.units);
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

        if groups.iter().all(|g| g.immune_system) {
            let result = groups.iter().fold(0, |acc, g| acc + g.units);
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
