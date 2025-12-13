use crate::input::Input;

struct Ingredient {
    capacity: i32,
    durability: i32,
    flavour: i32,
    texture: i32,
    calories: i32,
}

fn score_recipe(ingredients: &[Ingredient], teaspoons: &[i32], part2: bool) -> i32 {
    if teaspoons.iter().sum::<i32>() != 100 {
        return 0;
    }

    let mut capacity = 0;
    let mut durability = 0;
    let mut flavour = 0;
    let mut texture = 0;
    let mut calories = 0;

    for i in 0..ingredients.len() {
        capacity += ingredients[i].capacity * teaspoons[i];
        durability += ingredients[i].durability * teaspoons[i];
        flavour += ingredients[i].flavour * teaspoons[i];
        texture += ingredients[i].texture * teaspoons[i];
        calories += ingredients[i].calories * teaspoons[i];
    }

    if capacity <= 0 || durability <= 0 || flavour <= 0 || texture <= 0 {
        // "If any properties had produced a negative total, it would have
        // instead become zero, causing the whole score to multiply to zero."
        return 0;
    }

    if part2 && calories != 500 {
        return 0;
    }

    capacity * durability * flavour * texture
}

fn highest_score(
    ingredients: &[Ingredient],
    teaspoons: &mut [i32],
    index: usize,
    part2: bool,
) -> i32 {
    if index == teaspoons.len() {
        return score_recipe(ingredients, teaspoons, part2);
    }
    let spoons_used_so_far = teaspoons.iter().take(index).sum::<i32>();
    let mut max_score = 0;
    for i in 0..=(100 - spoons_used_so_far) {
        teaspoons[index] = i;
        let score = highest_score(ingredients, teaspoons, index + 1, part2);
        max_score = std::cmp::max(max_score, score);
    }
    max_score
}

pub fn solve(input: &Input) -> Result<i32, String> {
    let error_mapper = |_| "Invalid number";

    let mut ingredients = Vec::new();
    for line in input.text.lines() {
        let words = line.split(' ').collect::<Vec<_>>();
        if words.len() != 11 || words.iter().any(|s| s.is_empty()) {
            return Err("Invalid line not consisting of 11 words".to_string());
        }

        let capacity = words[2][0..words[2].len() - 1]
            .parse::<i32>()
            .map_err(error_mapper)?;
        let durability = words[4][0..words[4].len() - 1]
            .parse::<i32>()
            .map_err(error_mapper)?;
        let flavour = words[6][0..words[6].len() - 1]
            .parse::<i32>()
            .map_err(error_mapper)?;
        let texture = words[8][0..words[8].len() - 1]
            .parse::<i32>()
            .map_err(error_mapper)?;
        let calories = words[10].parse::<i32>().map_err(error_mapper)?;
        let ingredient = Ingredient {
            capacity,
            durability,
            flavour,
            texture,
            calories,
        };
        ingredients.push(ingredient);
    }

    let mut teaspoons = vec![0; ingredients.len()];

    Ok(highest_score(
        &ingredients,
        &mut teaspoons,
        0,
        input.is_part_two(),
    ))
}

#[test]
pub fn tests() {
    let example = "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3";
    test_part_one!(example => 62_842_880);
    test_part_two!(example => 57_600_000);

    let real_input = include_str!("day15_input.txt");
    test_part_one!(real_input => 18_965_440);
    test_part_two!(real_input => 15_862_900);
}
