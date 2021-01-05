use crate::input::Input;
use std::collections::HashSet;

fn extend(pieces: &[(u8, u8)], used: &HashSet<usize>, last: u8, part_two: bool) -> (u32, u32) {
    if pieces.len() == used.len() {
        return (0, 0);
    }

    let mut u = used.clone();
    pieces
        .iter()
        .enumerate()
        // Only keep unused pieces that can be attached to `last` piece:
        .filter(|&(idx, piece)| (piece.0 == last || piece.1 == last) && !used.contains(&idx))
        .map(|(idx, piece)| {
            u.insert(idx);
            let new_last = piece.0 + piece.1 - last;
            let (length, strength) = extend(pieces, &u, new_last, part_two);
            u.remove(&idx);
            (
                length + u32::from(part_two),
                strength + u32::from(piece.0) + u32::from(piece.1),
            )
        })
        .max()
        .unwrap_or((0, 0))
}

pub fn solve(input: &mut Input) -> Result<u32, String> {
    let mut pieces = Vec::new();
    for (line_idx, line) in input.text.lines().enumerate() {
        let on_error = || format!("Line {}: Invalid format", line_idx + 1);

        let mut parts = line.split('/');
        let first = parts
            .next()
            .ok_or_else(on_error)?
            .parse::<u8>()
            .map_err(|_| on_error())?;
        let second = parts
            .next()
            .ok_or_else(on_error)?
            .parse::<u8>()
            .map_err(|_| on_error())?;
        pieces.push((first, second));
    }

    Ok(extend(&pieces, &HashSet::new(), 0, input.part_values(false, true)).1)
}

#[test]
pub fn tests() {
    use crate::{test_part_one, test_part_two};

    let real_input = include_str!("day24_input.txt");
    test_part_one!(real_input => 1906);
    test_part_two!(real_input => 1824);
}
