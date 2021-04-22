pub fn recognize(input: &str) -> Option<char> {
    match input {
        " ██  \n█  █ \n█  █ \n████ \n█  █ \n█  █ " => Some('A'),
        "███  \n█  █ \n███  \n█  █ \n█  █ \n███  " => Some('B'),
        " ██  \n█  █ \n█    \n█    \n█  █ \n ██  " => Some('C'),
        "████ \n█    \n███  \n█    \n█    \n████ " => Some('E'),
        "████ \n█    \n███  \n█    \n█    \n█    " => Some('F'),
        " ██  \n█  █ \n█    \n█ ██ \n█  █ \n ███ " => Some('G'),
        "█  █ \n█  █ \n████ \n█  █ \n█  █ \n█  █ " => Some('H'),
        " ███ \n  █  \n  █  \n  █  \n  █  \n ███ " => Some('I'),
        "  ██ \n   █ \n   █ \n   █ \n█  █ \n ██  " => Some('J'),
        "█  █ \n█ █  \n██   \n█ █  \n█ █  \n█  █ " => Some('K'),
        "█    \n█    \n█    \n█    \n█    \n████ " => Some('L'),
        " ██  \n█  █ \n█  █ \n█  █ \n█  █ \n ██  " => Some('O'),
        "███  \n█  █ \n█  █ \n███  \n█    \n█    " => Some('P'),
        "███  \n█  █ \n█  █ \n███  \n█ █  \n█  █ " => Some('R'),
        " ███ \n█    \n█    \n ██  \n   █ \n███  " => Some('S'),
        "█  █ \n█  █ \n█  █ \n█  █ \n█  █ \n ██  " => Some('U'),
        "█   █\n█   █\n █ █ \n  █  \n  █  \n  █  " => Some('Y'),
        "████ \n   █ \n  █  \n █   \n█    \n████ " => Some('Z'),
        _ => None,
    }
}
