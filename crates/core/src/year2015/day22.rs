use crate::input::Input;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[derive(Default, Eq, PartialEq, Hash, Clone)]
struct Effect {
    turns: u8,
    armor: u8,
    damage: u8,
    mana: u8,
}

struct Spell {
    mana_cost: u8,
    damage: u8,
    heals: u8,
    effect_idx: Option<usize>,
}

#[derive(Eq, PartialEq, Clone, PartialOrd, Ord)]
struct State {
    spent_mana: u32,
    mana_left: u32,
    boss_hit_points: u8,
    player_hit_points: u8,
    effects_remaining_turns: [u8; 3],
}

fn process_effects(state: &mut State, effects: &[Effect]) -> Option<u8> {
    let mut effective_armor = 0;
    for (effect_idx, effect) in effects.iter().enumerate() {
        if state.effects_remaining_turns[effect_idx] > 0 {
            if effect.damage >= state.boss_hit_points {
                return None;
            }

            effective_armor += effect.armor;
            state.mana_left += u32::from(effect.mana);
            state.boss_hit_points -= effect.damage;
            state.effects_remaining_turns[effect_idx] -= 1;
        }
    }
    Some(effective_armor)
}

pub fn solve(input: &Input) -> Result<u32, String> {
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
            effect_idx: None,
        },
        Spell {
            // Drain
            mana_cost: 73,
            damage: 2,
            heals: 2,
            effect_idx: None,
        },
        Spell {
            // Shield
            mana_cost: 113,
            damage: 0,
            heals: 0,
            effect_idx: Some(0),
        },
        Spell {
            // Poison
            mana_cost: 173,
            damage: 0,
            heals: 0,
            effect_idx: Some(1),
        },
        Spell {
            // Recharge
            mana_cost: 229,
            damage: 0,
            heals: 0,
            effect_idx: Some(2),
        },
    ];

    let mut to_visit = BinaryHeap::new();
    to_visit.push(Reverse(State {
        spent_mana: 0,
        mana_left: 500,
        player_hit_points: 50,
        effects_remaining_turns: [0, 0, 0],
        boss_hit_points,
    }));

    while let Some(Reverse(state)) = to_visit.pop() {
        let mut new_state = state.clone();

        if input.is_part_two() {
            new_state.player_hit_points -= 1;
            if new_state.player_hit_points == 0 {
                continue;
            }
        }

        if process_effects(&mut new_state, &effects).is_none() {
            return Ok(new_state.spent_mana);
        };

        for spell in spells.iter() {
            let mut state_after_spell = new_state.clone();

            if u32::from(spell.mana_cost) > state_after_spell.mana_left {
                continue;
            }
            state_after_spell.mana_left -= u32::from(spell.mana_cost);
            state_after_spell.spent_mana += u32::from(spell.mana_cost);

            if let Some(effect_idx) = spell.effect_idx {
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

            // Boss turn
            if let Some(effective_armor) = process_effects(&mut state_after_spell, &effects) {
                let damage_on_player = std::cmp::max(1, boss_damage - effective_armor);
                if damage_on_player < state_after_spell.player_hit_points {
                    state_after_spell.player_hit_points -= damage_on_player;
                    to_visit.push(Reverse(state_after_spell));
                }
            } else {
                return Ok(state_after_spell.spent_mana);
            }
        }
    }

    Err("No solution found".to_string())
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    let real_input = include_str!("day22_input.txt");
    test_part_one!(real_input => 1824);
    test_part_two!(real_input => 1937);
}
