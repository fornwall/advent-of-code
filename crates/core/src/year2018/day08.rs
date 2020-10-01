// Returns (size, metadata sum)
fn count_metadata(data: &[i32], start: usize) -> Result<(usize, usize), String> {
    if data.len() < start + 2 {
        return Err("Invalid input".to_string());
    }
    let num_child_nodes = data[start];
    let metadata_entries = data[start + 1] as usize;

    let mut metadata_sum: usize = 0;
    let mut current_index = start + 2;
    for _ in 0..num_child_nodes {
        let child = count_metadata(data, current_index)?;
        current_index = child.0;
        metadata_sum += child.1;
    }

    for i in data.iter().skip(current_index).take(metadata_entries) {
        metadata_sum = metadata_sum
            .checked_add(*i as usize)
            .ok_or("Overflow in computation")?;
    }

    Ok((current_index + metadata_entries, metadata_sum as usize))
}

fn parse_input(input_string: &str) -> Result<Vec<i32>, String> {
    input_string
        .split_whitespace()
        .map(|word| {
            word.parse::<i32>()
                .map_err(|error| format!("Invalid input: {}", error.to_string()))
        })
        .collect()
}

pub fn part1(input_string: &str) -> Result<usize, String> {
    let data = parse_input(input_string)?;
    Ok(count_metadata(&data, 0)?.1)
}

fn evaluate_node(data: &[i32], start: usize) -> Result<(usize, usize), String> {
    if data.len() < start + 2 {
        return Err("Invalid input".to_string());
    }
    let num_child_nodes = data[start];
    let metadata_entries = data[start + 1] as usize;

    let mut children_values = Vec::new();
    let mut current_index = start + 2;

    for _ in 0..num_child_nodes {
        let child = evaluate_node(data, current_index)?;
        current_index = child.0;
        children_values.push(child.1);
    }

    let node_value =
        data.iter()
            .skip(current_index)
            .take(metadata_entries)
            .fold(0, |sum, &metadata| {
                if num_child_nodes == 0 {
                    sum + metadata as usize
                } else if metadata >= 1 && metadata as usize <= children_values.len() {
                    sum + children_values[(metadata - 1) as usize]
                } else {
                    sum
                }
            });

    let new_index = current_index
        .checked_add(metadata_entries)
        .ok_or("Overflow in computation")?;
    Ok((new_index, node_value as usize))
}

pub fn part2(input_string: &str) -> Result<usize, String> {
    let data = parse_input(input_string)?;
    Ok(evaluate_node(&data, 0)?.1)
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
