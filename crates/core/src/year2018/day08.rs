type InputNumber = u8;

fn solution(input_string: &str, part1: bool) -> Result<usize, String> {
    let data = input_string
        .split_whitespace()
        .map(|word| {
            word.parse::<InputNumber>()
                .map_err(|error| format!("Invalid input: {}", error.to_string()))
        })
        .collect::<Result<Vec<InputNumber>, _>>()?;
    Ok(evaluate_node(&data, 0, part1)?.1)
}

fn evaluate_node(
    data: &[InputNumber],
    start: usize,
    part1: bool,
) -> Result<(usize, usize), String> {
    if data.len() < start + 2 {
        return Err("Invalid input".to_string());
    }

    let mut children_values = Vec::new();
    let mut offset_after_children = start + 2;

    let num_child_nodes = data[start];
    for _ in 0..num_child_nodes {
        let (offset_after_child, child_value) = evaluate_node(data, offset_after_children, part1)?;
        offset_after_children = offset_after_child;
        children_values.push(child_value);
    }

    let metadata_entries = data[start + 1] as usize;
    let node_value = data
        .iter()
        .skip(offset_after_children)
        .take(metadata_entries)
        .map(|&metadata| {
            if part1 || num_child_nodes == 0 {
                metadata as usize
            } else if metadata >= 1 && metadata as usize <= children_values.len() {
                children_values[(metadata - 1) as usize]
            } else {
                0
            }
        })
        .sum::<usize>()
        + if part1 {
            children_values.iter().sum()
        } else {
            0
        };

    let offset_after_current = offset_after_children
        .checked_add(metadata_entries)
        .ok_or("Overflow in computation")?;
    Ok((offset_after_current, node_value as usize))
}

pub fn part1(input_string: &str) -> Result<usize, String> {
    solution(input_string, true)
}

pub fn part2(input_string: &str) -> Result<usize, String> {
    solution(input_string, false)
}

#[test]
fn tests_part1() {
    assert_eq!(Ok(138), part1("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2"));
    assert_eq!(Ok(47112), part1(include_str!("day08_input.txt")));
}

#[test]
fn tests_part2() {
    assert_eq!(Ok(66), part2("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2"));
    assert_eq!(Ok(28237), part2(include_str!("day08_input.txt")));
}
