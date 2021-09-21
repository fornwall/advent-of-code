use crate::Input;

struct Stats {
    hit_points: u16,
    damage: u16,
    armor: u16,
}

impl Stats {
    fn parse(input: &str) -> Result<Self, String> {
        let mut hit_points = 0;
        let mut damage = 0;
        let mut armor = 0;
        for line in input.lines() {
            if let Some(remaining) = line.strip_prefix("Hit Points: ") {
                hit_points = remaining
                    .parse::<u16>()
                    .map_err(|_| "Cannot parse hit points")?;
            }
            if let Some(remaining) = line.strip_prefix("Damage: ") {
                damage = remaining
                    .parse::<u16>()
                    .map_err(|_| "Cannot parse damage")?;
            }
            if let Some(remaining) = line.strip_prefix("Armor: ") {
                armor = remaining.parse::<u16>().map_err(|_| "Cannot parse armor")?;
            }
        }
        Ok(Self {
            hit_points,
            damage,
            armor,
        })
    }
}

struct Item {
    cost: u16,
    damage: u16,
    armor: u16,
}

const fn divide_rounding_up(dividend: u16, divisor: u16) -> u16 {
    (dividend + (divisor - 1)) / divisor
}

fn player_wins(player: &Stats, boss: &Stats) -> bool {
    let player_rounds_to_win = divide_rounding_up(
        boss.hit_points,
        std::cmp::max(i32::from(player.damage) - i32::from(boss.armor), 1) as u16,
    );
    let boss_rounds_to_win = divide_rounding_up(
        player.hit_points,
        std::cmp::max(i32::from(boss.damage) - i32::from(player.armor), 1) as u16,
    );
    player_rounds_to_win <= boss_rounds_to_win
}

pub fn solve(input: &mut Input) -> Result<u16, String> {
    let boss_stats = Stats::parse(input.text)?;

    let weapons = [
        Item {
            // Dagger
            cost: 8,
            damage: 4,
            armor: 0,
        },
        Item {
            // Shortsword
            cost: 10,
            damage: 5,
            armor: 0,
        },
        Item {
            // Warhammer
            cost: 25,
            damage: 6,
            armor: 0,
        },
        Item {
            // Longsword
            cost: 40,
            damage: 7,
            armor: 0,
        },
        Item {
            // Greataxe
            cost: 74,
            damage: 8,
            armor: 0,
        },
    ];

    let armors = [
        Item {
            // Nothing
            cost: 0,
            damage: 0,
            armor: 0,
        },
        Item {
            // Leather
            cost: 13,
            damage: 0,
            armor: 1,
        },
        Item {
            // Chainmail
            cost: 31,
            damage: 0,
            armor: 2,
        },
        Item {
            // Splintmail
            cost: 53,
            damage: 0,
            armor: 3,
        },
        Item {
            // Bandedmail
            cost: 75,
            damage: 0,
            armor: 4,
        },
        Item {
            // Platemail
            cost: 102,
            damage: 0,
            armor: 5,
        },
    ];

    let rings = [
        Item {
            // Nothing
            cost: 0,
            damage: 0,
            armor: 0,
        },
        Item {
            // Damage +1
            cost: 25,
            damage: 1,
            armor: 0,
        },
        Item {
            // Damage +2
            cost: 50,
            damage: 2,
            armor: 0,
        },
        Item {
            // Damage +3
            cost: 100,
            damage: 3,
            armor: 0,
        },
        Item {
            // Defense +1
            cost: 20,
            damage: 0,
            armor: 1,
        },
        Item {
            // Defense +2
            cost: 40,
            damage: 0,
            armor: 2,
        },
        Item {
            // Defence +3
            cost: 80,
            damage: 0,
            armor: 3,
        },
    ];

    let mut best_cost = input.part_values(u16::MAX, u16::MIN);
    let want_player_to_win = input.is_part_one();
    let cost_keeper = if input.is_part_one() {
        std::cmp::min
    } else {
        std::cmp::max
    };

    for weapon in weapons.iter() {
        for armor in armors.iter() {
            for ring1 in rings.iter() {
                for ring2 in rings.iter() {
                    // Rings must be different - except for "Nothing" (cost == 0):
                    if ring1.cost != ring2.cost || (ring1.cost == 0 && ring2.cost == 0) {
                        let player_stats = Stats {
                            hit_points: 100,
                            damage: weapon.damage + armor.damage + ring1.damage + ring2.damage,
                            armor: weapon.armor + armor.armor + ring1.armor + ring2.armor,
                        };

                        if player_wins(&player_stats, &boss_stats) == want_player_to_win {
                            let cost = weapon.cost + armor.cost + ring1.cost + ring2.cost;
                            best_cost = cost_keeper(best_cost, cost);
                        }
                    }
                }
            }
        }
    }

    Ok(best_cost)
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    let real_input = include_str!("day21_input.txt");
    test_part_one!(real_input => 91);
    test_part_two!(real_input => 158);
}
