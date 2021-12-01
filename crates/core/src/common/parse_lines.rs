use std::str::FromStr;

pub fn parse_lines<T: FromStr>(input: &str) -> Result<Vec<T>, String> {
    input
        .lines()
        .enumerate()
        .map(|(line_idx, line)| {
            line.parse::<T>()
                .map_err(|_| format!("Line {}: Not a valid integer", line_idx + 1))
        })
        .collect()
}
