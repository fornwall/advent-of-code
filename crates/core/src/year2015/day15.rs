use crate::Input;

#[derive(Debug)]
struct Ingredient {
    capacity: i32,
    durability: i32,
    flavour: i32,
    texture: i32,
    calories: i32,
}

fn calculate_recipe(ingredients: &[Ingredient], teaspoons: &[i32], part2: bool) -> i32 {
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

pub fn solve(input: &mut Input) -> Result<i32, String> {
    let error_mapper = |_| "Invalid number";

    let mut ingredients = Vec::new();
    for line in input.text.lines() {
        let words = line.split(' ').collect::<Vec<_>>();
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

    let mut max_score = 0;
    for i1 in 0..=100 {
        teaspoons[0] = i1;

        for i2 in 0..=100 - i1 {
            teaspoons[1] = i2;

            for i3 in 0..=100 - (i1 + i2) {
                teaspoons[2] = i3;

                teaspoons[3] = 100 - (i1 + i2 + i3);

                let score = calculate_recipe(&ingredients, &teaspoons, input.is_part_two());
                max_score = std::cmp::max(max_score, score);
            }
        }
    }

    Ok(max_score)
}

#[test]
pub fn tests() {
    use crate::{test_part_one, test_part_two};

    let real_input = include_str!("day15_input.txt");
    test_part_one!(real_input => 18_965_440);
    test_part_two!(real_input => 15_862_900);
}
