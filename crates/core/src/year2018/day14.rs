use crate::input::Input;

fn run_until<F>(condition: F) -> Result<Vec<u8>, String>
where
    F: Fn(&Vec<u8>) -> bool,
{
    const MAX_ITERATIONS: i32 = 100_000_000;

    let mut scores = Vec::with_capacity(25_000_000);
    scores.push(3_u8);
    scores.push(7_u8);

    let push_score = |scores: &mut Vec<u8>, score: u8| {
        scores.push(score);
        condition(scores)
    };

    let mut elf_positions = (0_u32, 1_u32);

    let mut loop_count = 0;
    loop {
        let score_0 = scores[elf_positions.0 as usize];
        let score_1 = scores[elf_positions.1 as usize];
        let current_recipes_score = score_0 + score_1;

        let done = if current_recipes_score < 10 {
            push_score(&mut scores, current_recipes_score)
        } else {
            push_score(&mut scores, current_recipes_score / 10)
                || push_score(&mut scores, current_recipes_score % 10)
        };
        if done {
            return Ok(scores);
        }

        elf_positions.0 = (elf_positions.0 + 1 + u32::from(score_0)) % scores.len() as u32;
        elf_positions.1 = (elf_positions.1 + 1 + u32::from(score_1)) % scores.len() as u32;

        loop_count += 1;
        if loop_count > MAX_ITERATIONS {
            return Err(format!("Aborted after {} iterations", MAX_ITERATIONS));
        }
    }
}

pub fn solve(input: &mut Input) -> Result<String, String> {
    if input.is_part_one() {
        let input_num_recipes = input
            .text
            .parse::<u32>()
            .map_err(|error| format!("Invalid input: {}", error.to_string()))?
            as usize;
        let num_recipes_after = 10;
        let desired_length = input_num_recipes + num_recipes_after;

        let scores = run_until(|scores| scores.len() >= desired_length)?;

        Ok(scores
            .iter()
            .skip(input_num_recipes)
            .take(num_recipes_after)
            .fold(String::new(), |acc, score| acc + &score.to_string()))
    } else {
        let input_bytes: Vec<u8> = input
            .text
            .chars()
            .map(|b| {
                b.to_digit(10)
                    .map(|b| b as u8)
                    .ok_or_else(|| "Invalid input".to_string())
            })
            .collect::<Result<Vec<_>, String>>()?;

        if input_bytes.len() > 20 {
            return Err("Too long input".to_string());
        }

        let scores = run_until(|scores| scores.ends_with(&input_bytes))?;
        Ok((scores.len() - input.text.len()).to_string())
    }
}

#[test]
fn tests() {
    use crate::input::{test_part_one, test_part_two};

    test_part_one!("9" => "5158916779".into());
    test_part_one!("5" => "0124515891".into());
    test_part_one!("18" => "9251071085".into());
    test_part_one!("2018" => "5941429882".into());

    test_part_two!("51589" => "9".into());
    test_part_two!("01245" => "5".into());
    test_part_two!("92510" => "18".into());
    test_part_two!("59414" => "2018".into());

    let input = include_str!("day14_input.txt");
    test_part_one!(input => "1150511382".into());
    test_part_two!(input => "20173656".into());
}
