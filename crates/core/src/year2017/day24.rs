use crate::input::Input;

type PieceComponent = u8;
type Piece = (PieceComponent, PieceComponent);

fn score(bridge: &[Piece]) -> u32 {
    bridge.iter().fold(0, |acc, &(start, end)| {
        acc + u32::from(start) + u32::from(end)
    })
}

fn extend(pieces: &Vec<Piece>, last: PieceComponent, part_two: bool) -> Vec<Piece> {
    #![allow(clippy::ptr_arg)]
    pieces
        .iter()
        .enumerate()
        .filter_map(|(idx, &piece)| {
            if piece.0 == last || piece.1 == last {
                let mut pieces_cloned = pieces.clone();
                pieces_cloned.swap_remove(idx);
                let new_last = piece.0 + piece.1 - last;

                let mut bridge = extend(&pieces_cloned, new_last, part_two);
                bridge.push(piece);
                Some(bridge)
            } else {
                None
            }
        })
        .max_by(|a, b| {
            if part_two {
                a.len().cmp(&b.len()).then(score(a).cmp(&score(b)))
            } else {
                score(a).cmp(&score(b))
            }
        })
        .unwrap_or_default()
}

pub fn solve(input: &Input) -> Result<u32, String> {
    let mut pieces = Vec::new();
    for (line_idx, line) in input.text.lines().enumerate() {
        let on_error = || format!("Line {}: Invalid format", line_idx + 1);

        let mut parts = line.split('/');
        let first = parts
            .next()
            .ok_or_else(on_error)?
            .parse::<PieceComponent>()
            .map_err(|_| on_error())?;
        let second = parts
            .next()
            .ok_or_else(on_error)?
            .parse::<PieceComponent>()
            .map_err(|_| on_error())?;
        pieces.push((first, second));
    }

    Ok(score(&extend(&pieces, 0, input.part_values(false, true))))
}

#[test]
pub fn tests() {
    let real_input = include_str!("day24_input.txt");
    test_part_one!(real_input => 1906);
    test_part_two!(real_input => 1824);
}
