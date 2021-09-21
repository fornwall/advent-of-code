use crate::input::Input;
use std::collections::{HashMap, HashSet};

pub fn solve(input: &mut Input) -> Result<String, String> {
    let on_error = || "Invalid input";

    let mut allergen_to_idx = HashMap::with_capacity(8);
    let mut allergen_names = Vec::with_capacity(8);

    let mut ingredient_to_idx = HashMap::with_capacity(200);
    let mut ingredient_names = Vec::with_capacity(200);

    let mut ingredient_occurences: Vec<u16> = Vec::with_capacity(200);
    let mut allergen_to_possible_ingredients = Vec::with_capacity(8);

    for line in input.text.lines() {
        let mut line_parts = line.splitn(2, " (contains ");

        let ingredients = line_parts.next().ok_or_else(on_error)?;
        let mut current_ingredients = HashSet::new();
        for ingredient_name in ingredients.split(' ') {
            let num_ingredients = ingredient_to_idx.len();
            let ingredient_id = *ingredient_to_idx
                .entry(ingredient_name)
                .or_insert(num_ingredients);

            if ingredient_id == ingredient_occurences.len() {
                ingredient_occurences.push(1);
                ingredient_names.push(ingredient_name);
            } else {
                ingredient_occurences[ingredient_id] += 1;
            }

            current_ingredients.insert(ingredient_id);
        }

        if let Some(allergens) = line_parts.next().ok_or_else(on_error)?.strip_suffix(')') {
            for allergen_name in allergens.split(", ") {
                let num_allergens = allergen_to_idx.len();
                let allergen_id = *allergen_to_idx
                    .entry(allergen_name)
                    .or_insert(num_allergens);
                if allergen_id == allergen_names.len() {
                    allergen_names.push(allergen_name);
                    allergen_to_possible_ingredients.push(current_ingredients.clone());
                } else {
                    let existing = &allergen_to_possible_ingredients[allergen_id];
                    allergen_to_possible_ingredients[allergen_id] = current_ingredients
                        .intersection(existing)
                        .copied()
                        .collect();
                }
            }
        } else {
            return Err(on_error().to_string());
        }
    }

    if input.is_part_one() {
        return Ok((0..ingredient_to_idx.len())
            .filter_map(|ingredient_id| {
                if allergen_to_possible_ingredients
                    .iter()
                    .any(|possible| possible.contains(&ingredient_id))
                {
                    None
                } else {
                    Some(u64::from(ingredient_occurences[ingredient_id]))
                }
            })
            .sum::<u64>()
            .to_string());
    }

    let mut identified_products = allergen_to_possible_ingredients
        .iter()
        .filter_map(|possibilities| {
            if possibilities.len() == 1 {
                possibilities.iter().next().copied()
            } else {
                None
            }
        })
        .collect::<Vec<usize>>();

    while let Some(product_id) = identified_products.pop() {
        for possibilities in allergen_to_possible_ingredients.iter_mut() {
            if possibilities.len() > 1
                && possibilities.remove(&product_id)
                && possibilities.len() == 1
            {
                let p = possibilities
                    .iter()
                    .next()
                    .ok_or_else(|| "Internal error".to_string())?;
                identified_products.push(*p);
            }
        }
    }

    let mut ingredient_and_allergents = allergen_to_possible_ingredients
        .iter()
        .enumerate()
        .map(
            |(idx, possible_ingredients)| -> Result<(usize, usize), String> {
                Ok((
                    *possible_ingredients
                        .iter()
                        .next()
                        .ok_or_else(|| "Internal error".to_string())?,
                    idx,
                ))
            },
        )
        .collect::<Result<Vec<_>, _>>()?;

    ingredient_and_allergents.sort_unstable_by(|a, b| {
        let a_allergen_name = allergen_names[a.1];
        let b_allergen_name = allergen_names[b.1];
        a_allergen_name.cmp(b_allergen_name)
    });

    Ok(ingredient_and_allergents
        .iter()
        .map(|&(ingredient_id, _)| ingredient_names[ingredient_id])
        .collect::<Vec<_>>()
        .join(","))
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    let example = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";
    test_part_one!(example => "5".to_string());
    test_part_two!(example => "mxmxvkd,sqjhc,fvjkl".to_string());

    let other_input = include_str!("day21_input_other.txt");
    test_part_one!(other_input => "2724".to_string());
    test_part_two!(other_input => "xlxknk,cskbmx,cjdmk,bmhn,jrmr,tzxcmr,fmgxh,fxzh".to_string());

    let real_input = include_str!("day21_input.txt");
    test_part_one!(real_input => "2317".to_string());
    test_part_two!(real_input => "kbdgs,sqvv,slkfgq,vgnj,brdd,tpd,csfmb,lrnz".to_string());
}
