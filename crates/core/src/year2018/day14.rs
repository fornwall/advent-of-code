pub fn part1(input_string: &str) -> String {
    let input_number = input_string.parse::<usize>().unwrap();
    let desired_length = input_number + 10;

    let mut scores = vec![3_u8, 7_u8];
    let mut elf_positions = vec![0, 1];

    while scores.len() < desired_length {
        let current_recipes_score = scores[elf_positions[0]] + scores[elf_positions[1]];

        if current_recipes_score < 10 {
            scores.push(current_recipes_score);
        } else {
            scores.push(current_recipes_score / 10);
            scores.push(current_recipes_score % 10);
        }

        elf_positions.iter_mut().for_each(|position| {
            *position = (*position + 1 + scores[*position as usize] as usize) % scores.len();
        });
    }

    scores
        .iter()
        .skip(input_number)
        .take(10)
        .fold(String::new(), |acc, score| acc + &score.to_string())
}

pub fn part2(input_string: &str) -> String {
    let input_bytes: Vec<u8> = input_string.as_bytes().iter().map(|b| b - 48).collect();

    let mut scores = vec![3_u8, 7_u8];
    let mut elf_positions = vec![0, 1];

    loop {
        let current_recipes_score = scores[elf_positions[0]] + scores[elf_positions[1]];

        let scores_to_push = if current_recipes_score < 10 {
            vec![current_recipes_score]
        } else {
            vec![current_recipes_score / 10, current_recipes_score % 10]
        };

        for score in scores_to_push {
            scores.push(score);
            if scores.ends_with(&input_bytes) {
                return (scores.len() - input_string.len()).to_string();
            }
        }

        elf_positions.iter_mut().for_each(|position| {
            *position = (*position + 1 + scores[*position as usize] as usize) % scores.len();
        });
    }
}

#[test]
fn tests_part1() {
    assert_eq!("5158916779", part1("9"));
    assert_eq!("0124515891", part1("5"));
    assert_eq!("9251071085", part1("18"));
    assert_eq!("5941429882", part1("2018"));

    assert_eq!("1150511382", part1(include_str!("day14_input.txt")));
}

#[test]
fn tests_part2() {
    assert_eq!("9", part2("51589"));
    assert_eq!("5", part2("01245"));
    assert_eq!("18", part2("92510"));
    assert_eq!("2018", part2("59414"));

    assert_eq!("20173656", part2(include_str!("day14_input.txt")));
}
