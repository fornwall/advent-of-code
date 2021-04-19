use crate::Input;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Default, Eq, PartialEq, Hash, Clone, Debug)]
struct Effect {
    turns: u8,
    armor: u8,
    damage: u8,
    mana: u8,
}

#[derive(Debug)]
struct Spell {
    mana_cost: u8,
    damage: u8,
    heals: u8,
    effect: Option<usize>,
}

#[derive(Eq, PartialEq, Clone, Debug, PartialOrd, Ord)]
struct State {
    spent_mana: u32,
    mana_left: u32,
    player_turn: bool,
    boss_hit_points: u8,
    player_hit_points: u8,
    boss_damage: u8,
    effects_remaining_turns: [u8; 3],
}

pub fn solve(input: &mut Input) -> Result<u32, String> {
    let mut boss_hit_points = 0;
    let mut boss_damage = 0;
    for line in input.text.lines() {
        if let Some(suffix) = line.strip_prefix("Hit Points: ") {
            boss_hit_points = suffix
                .parse::<u8>()
                .map_err(|_| "Cannot parse hit points")?;
        }
        if let Some(suffix) = line.strip_prefix("Damage: ") {
            boss_damage = suffix.parse::<u8>().map_err(|_| "Cannot parse damage")?;
        }
    }
    if boss_hit_points == 0 || boss_damage == 0 {
        return Err("Need to specify boss hit points and damage".to_string());
    }

    let effects = [
        Effect {
            turns: 6,
            armor: 7,
            ..Default::default()
        },
        Effect {
            turns: 6,
            damage: 3,
            ..Default::default()
        },
        Effect {
            turns: 5,
            mana: 101,
            ..Default::default()
        },
    ];

    let spells = [
        Spell {
            // Magic Missile
            mana_cost: 53,
            damage: 4,
            heals: 0,
            effect: None,
        },
        Spell {
            // Drain
            mana_cost: 73,
            damage: 2,
            heals: 2,
            effect: None,
        },
        Spell {
            // Shield
            mana_cost: 113,
            damage: 0,
            heals: 0,
            effect: Some(0),
        },
        Spell {
            // Poison
            mana_cost: 173,
            damage: 0,
            heals: 0,
            effect: Some(1),
        },
        Spell {
            // Recharge
            mana_cost: 229,
            damage: 0,
            heals: 0,
            effect: Some(2),
        },
    ];

    let mut to_visit = BinaryHeap::new();
    to_visit.push(Reverse(State {
        spent_mana: 0,
        mana_left: 500,
        player_hit_points: 50,
        player_turn: true,
        effects_remaining_turns: [0, 0, 0],
        boss_damage,
        boss_hit_points,
    }));

    while let Some(Reverse(state)) = to_visit.pop() {
        let mut new_state = state.clone();
        let was_player_turn = new_state.player_turn;
        new_state.player_turn = !state.player_turn;

        if input.is_part_two() && was_player_turn {
            if new_state.player_hit_points == 1 {
                continue;
            }
            new_state.player_hit_points -= 1;
        }

        let mut effective_armor = 0;
        for (effect_idx, effect) in effects.iter().enumerate() {
            if new_state.effects_remaining_turns[effect_idx] > 0 {
                effective_armor += effect.armor;

                new_state.mana_left += u32::from(effect.mana);

                if effect.damage >= new_state.boss_hit_points {
                    return Ok(new_state.spent_mana);
                }
                new_state.boss_hit_points -= effect.damage;

                new_state.effects_remaining_turns[effect_idx] -= 1;
            }
        }

        if was_player_turn {
            for spell in spells.iter() {
                let mut state_after_spell = new_state.clone();

                if u32::from(spell.mana_cost) > state_after_spell.mana_left {
                    continue;
                }
                state_after_spell.mana_left -= u32::from(spell.mana_cost);
                state_after_spell.spent_mana += u32::from(spell.mana_cost);

                if let Some(effect_idx) = spell.effect {
                    if state_after_spell.effects_remaining_turns[effect_idx] > 0 {
                        continue;
                    } else {
                        state_after_spell.effects_remaining_turns[effect_idx] =
                            effects[effect_idx].turns;
                    }
                }

                state_after_spell.player_hit_points += spell.heals;
                if state_after_spell.boss_hit_points <= spell.damage {
                    return Ok(state_after_spell.spent_mana);
                }
                state_after_spell.boss_hit_points -= spell.damage;

                to_visit.push(Reverse(state_after_spell));
            }
        } else {
            let damage_on_player = std::cmp::max(1, state.boss_damage - effective_armor);
            if damage_on_player >= new_state.player_hit_points {
                // Player is dead - abort.
                continue;
            }
            new_state.player_hit_points -= damage_on_player;
            to_visit.push(Reverse(new_state));
        }
    }

    Err("No solution found".to_string())
}

#[test]
pub fn tests() {
    use crate::{test_part_one, test_part_two};

    let real_input = include_str!("day22_input.txt");
    test_part_one!(real_input => 1824);
    test_part_two!(real_input => 1937);
}
