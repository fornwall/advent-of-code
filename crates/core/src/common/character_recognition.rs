pub const CHAR_WIDTH: usize = 5;
pub const CHAR_HEIGHT: usize = 6;

pub fn recognize_letter(input: &str) -> Result<char, String> {
    Ok(match input {
        " ██  \n█  █ \n█  █ \n████ \n█  █ \n█  █ " => 'A',
        "███  \n█  █ \n███  \n█  █ \n█  █ \n███  " => 'B',
        " ██  \n█  █ \n█    \n█    \n█  █ \n ██  " => 'C',
        "████ \n█    \n███  \n█    \n█    \n████ " => 'E',
        "████ \n█    \n███  \n█    \n█    \n█    " => 'F',
        " ██  \n█  █ \n█    \n█ ██ \n█  █ \n ███ " => 'G',
        "█  █ \n█  █ \n████ \n█  █ \n█  █ \n█  █ " => 'H',
        " ███ \n  █  \n  █  \n  █  \n  █  \n ███ " => 'I',
        "  ██ \n   █ \n   █ \n   █ \n█  █ \n ██  " => 'J',
        "█  █ \n█ █  \n██   \n█ █  \n█ █  \n█  █ " => 'K',
        "█    \n█    \n█    \n█    \n█    \n████ " => 'L',
        " ██  \n█  █ \n█  █ \n█  █ \n█  █ \n ██  " => 'O',
        "███  \n█  █ \n█  █ \n███  \n█    \n█    " => 'P',
        "███  \n█  █ \n█  █ \n███  \n█ █  \n█  █ " => 'R',
        " ███ \n█    \n█    \n ██  \n   █ \n███  " => 'S',
        "█  █ \n█  █ \n█  █ \n█  █ \n█  █ \n ██  " => 'U',
        "█   █\n█   █\n █ █ \n  █  \n  █  \n  █  " => 'Y',
        "████ \n   █ \n  █  \n █   \n█    \n████ " => 'Z',
        _ => {
            return Err(format!("Unrecognized character:\n{}", input));
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
    let mut result = String::new();
    for letter_idx in 0..num_letters {
        let mut letter_str = String::new();
        for row in 0..CHAR_HEIGHT {
            let start_offset = letter_idx * CHAR_WIDTH + row * num_letters * CHAR_WIDTH;
            let end_offset = start_offset + CHAR_WIDTH;
            for b in &bytes[start_offset..end_offset] {
                letter_str.push(if *b { '█' } else { ' ' });
            }
            if row != CHAR_HEIGHT - 1 {
                letter_str.push('\n');
            }
        }
        result.push(recognize_letter(&letter_str)?);
    }
    Ok(result)
}
