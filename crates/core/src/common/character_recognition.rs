pub const CHAR_WIDTH: usize = 5;
pub const CHAR_HEIGHT: usize = 6;

fn recognize_letter(
    r1: &[bool],
    r2: &[bool],
    r3: &[bool],
    r4: &[bool],
    r5: &[bool],
    r6: &[bool],
) -> Result<char, String> {
    Ok(match (r1, r2, r3, r4, r5, r6) {
        (
            [false, true, true, false, false],
            [true, false, false, true, false],
            [true, false, false, true, false],
            [true, true, true, true, false],
            [true, false, false, true, false],
            [true, false, false, true, false],
        ) => 'A',

        (
            [true, true, true, false, false],
            [true, false, false, true, false],
            [true, true, true, false, false],
            [true, false, false, true, false],
            [true, false, false, true, false],
            [true, true, true, false, false],
        ) => 'B',

        (
            [false, true, true, false, false],
            [true, false, false, true, false],
            [true, false, false, false, false],
            [true, false, false, false, false],
            [true, false, false, true, false],
            [false, true, true, false, false],
        ) => 'C',

        (
            [true, true, true, true, false],
            [true, false, false, false, false],
            [true, true, true, false, false],
            [true, false, false, false, false],
            [true, false, false, false, false],
            [true, true, true, true, false],
        ) => 'E',

        (
            [true, true, true, true, false],
            [true, false, false, false, false],
            [true, true, true, false, false],
            [true, false, false, false, false],
            [true, false, false, false, false],
            [true, false, false, false, false],
        ) => 'F',

        (
            [false, true, true, false, false],
            [true, false, false, true, false],
            [true, false, false, false, false],
            [true, false, true, true, false],
            [true, false, false, true, false],
            [false, true, true, true, false],
        ) => 'G',

        (
            [true, false, false, true, false],
            [true, false, false, true, false],
            [true, true, true, true, false],
            [true, false, false, true, false],
            [true, false, false, true, false],
            [true, false, false, true, false],
        ) => 'H',

        (
            [false, true, true, true, false],
            [false, false, true, false, false],
            [false, false, true, false, false],
            [false, false, true, false, false],
            [false, false, true, false, false],
            [false, true, true, true, false],
        ) => 'I',

        (
            [false, false, true, true, false],
            [false, false, false, true, false],
            [false, false, false, true, false],
            [false, false, false, true, false],
            [true, false, false, true, false],
            [false, true, true, false, false],
        ) => 'J',

        (
            [true, false, false, true, false],
            [true, false, true, false, false],
            [true, true, false, false, false],
            [true, false, true, false, false],
            [true, false, true, false, false],
            [true, false, false, true, false],
        ) => 'K',

        (
            [true, false, false, false, false],
            [true, false, false, false, false],
            [true, false, false, false, false],
            [true, false, false, false, false],
            [true, false, false, false, false],
            [true, true, true, true, false],
        ) => 'L',

        (
            [false, true, true, false, false],
            [true, false, false, true, false],
            [true, false, false, true, false],
            [true, false, false, true, false],
            [true, false, false, true, false],
            [false, true, true, false, false],
        ) => 'O',

        (
            [true, true, true, false, false],
            [true, false, false, true, false],
            [true, false, false, true, false],
            [true, true, true, false, false],
            [true, false, false, false, false],
            [true, false, false, false, false],
        ) => 'P',

        (
            [true, true, true, false, false],
            [true, false, false, true, false],
            [true, false, false, true, false],
            [true, true, true, false, false],
            [true, false, true, false, false],
            [true, false, false, true, false],
        ) => 'R',

        (
            [false, true, true, true, false],
            [true, false, false, false, false],
            [true, false, false, false, false],
            [false, true, true, false, false],
            [false, false, false, true, false],
            [true, true, true, false, false],
        ) => 'S',

        (
            [true, false, false, true, false],
            [true, false, false, true, false],
            [true, false, false, true, false],
            [true, false, false, true, false],
            [true, false, false, true, false],
            [false, true, true, false, false],
        ) => 'U',

        (
            [true, false, false, false, true],
            [true, false, false, false, true],
            [false, true, false, true, false],
            [false, false, true, false, false],
            [false, false, true, false, false],
            [false, false, true, false, false],
        ) => 'Y',

        (
            [true, true, true, true, false],
            [false, false, false, true, false],
            [false, false, true, false, false],
            [false, true, false, false, false],
            [true, false, false, false, false],
            [true, true, true, true, false],
        ) => 'Z',

        _ => {
            return Err("Unrecognized character".to_string());
        }
    })
}

pub fn recognize(bytes: &[bool]) -> Result<String, String> {
    if bytes.len() % (CHAR_WIDTH * CHAR_HEIGHT) != 0 {
        return Err(format!(
            "Input length is not a multiple of {}",
            CHAR_WIDTH * CHAR_HEIGHT
        ));
    }
    let num_letters = bytes.len() / (CHAR_WIDTH * CHAR_HEIGHT);
    let all_width = CHAR_WIDTH * num_letters;
    let mut result = String::with_capacity(num_letters);
    for letter_idx in 0..num_letters {
        result.push(recognize_letter(
            &bytes[(letter_idx * CHAR_WIDTH)..((letter_idx + 1) * CHAR_WIDTH)],
            &bytes[(letter_idx * CHAR_WIDTH + all_width)
                ..((letter_idx + 1) * CHAR_WIDTH + all_width)],
            &bytes[(letter_idx * CHAR_WIDTH + 2 * all_width)
                ..((letter_idx + 1) * CHAR_WIDTH + 2 * all_width)],
            &bytes[(letter_idx * CHAR_WIDTH + 3 * all_width)
                ..((letter_idx + 1) * CHAR_WIDTH + 3 * all_width)],
            &bytes[(letter_idx * CHAR_WIDTH + 4 * all_width)
                ..((letter_idx + 1) * CHAR_WIDTH + 4 * all_width)],
            &bytes[(letter_idx * CHAR_WIDTH + 5 * all_width)
                ..((letter_idx + 1) * CHAR_WIDTH + 5 * all_width)],
        )?);
    }
    Ok(result)
}
