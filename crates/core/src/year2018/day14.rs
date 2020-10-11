fn run_until<F>(condition: F) -> Result<Vec<u8>, String>
where
    F: Fn(&Vec<u8>) -> bool,
{
    const MAX_ITERATIONS: i32 = 100_000_000;

    let mut scores = vec![3_u8, 7_u8];
    let mut elf_positions = vec![0, 1];

    let mut loop_count = 0;
    loop {
        let current_recipes_score = scores[elf_positions[0]] + scores[elf_positions[1]];

        let scores_to_push = if current_recipes_score < 10 {
            vec![current_recipes_score]
        } else {
            vec![current_recipes_score / 10, current_recipes_score % 10]
        };

        for score in scores_to_push {
            scores.push(score);
            if condition(&scores) {
                return Ok(scores);
            }
        }

        for position in elf_positions.iter_mut() {
            *position = (*position + 1 + scores[*position as usize] as usize) % scores.len();
        }

        loop_count += 1;
        if loop_count > MAX_ITERATIONS {
            return Err(format!("Aborted after {} iterations", MAX_ITERATIONS));
        }
    }
}

pub fn part1(input_string: &str) -> Result<String, String> {
    let input_num_recipes = input_string
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
}

pub fn part2(input_string: &str) -> Result<usize, String> {
    let input_bytes: Vec<u8> = input_string
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
    Ok(scores.len() - input_string.len())
}

#[test]
fn tests_part1() {
    assert_eq!(Ok("5158916779".to_string()), part1("9"));
    assert_eq!(Ok("0124515891".to_string()), part1("5"));
    assert_eq!(Ok("9251071085".to_string()), part1("18"));
    assert_eq!(Ok("5941429882".to_string()), part1("2018"));

    assert_eq!(
        Ok("1150511382".to_string()),
        part1(include_str!("day14_input.txt"))
    );
}

#[test]
fn tests_part2() {
    assert_eq!(Ok(9), part2("51589"));
    assert_eq!(Ok(5), part2("01245"));
    assert_eq!(Ok(18), part2("92510"));
    assert_eq!(Ok(2018), part2("59414"));

    assert_eq!(Ok(20_173_656), part2(include_str!("day14_input.txt")));
}
