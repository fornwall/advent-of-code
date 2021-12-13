pub fn recognize(input: &str) -> Result<char, String> {
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
