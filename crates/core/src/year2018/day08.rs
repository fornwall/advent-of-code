// Returns (size, metadata sum)
fn count_metadata(data: &[i32], start: usize) -> (usize, usize) {
    let num_child_nodes = data[start];
    let metadata_entries = data[start + 1] as usize;

    let mut metadata_sum = 0;
    let mut current_index = start + 2;
    for _ in 0..num_child_nodes {
        let child = count_metadata(data, current_index);
        current_index = child.0;
        metadata_sum += child.1;
    }

    for i in data.iter().skip(current_index).take(metadata_entries) {
        metadata_sum += *i as usize;
    }

    (current_index + metadata_entries, metadata_sum as usize)
}

pub fn part1(input_string: &str) -> String {
    let data: Vec<i32> = input_string
        .split_whitespace()
        .map(|word| word.parse::<i32>().unwrap())
        .collect();
    count_metadata(&data, 0).1.to_string()
}

fn evaluate_node(data: &[i32], start: usize) -> (usize, usize) {
    let num_child_nodes = data[start];
    let metadata_entries = data[start + 1] as usize;

    let mut children_values = Vec::new();
    let mut current_index = start + 2;

    for _ in 0..num_child_nodes {
        let child = evaluate_node(data, current_index);
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

    (current_index + metadata_entries, node_value as usize)
}

pub fn part2(input_string: &str) -> String {
    let data: Vec<i32> = input_string
        .split_whitespace()
        .map(|word| word.parse::<i32>().unwrap())
        .collect();
    evaluate_node(&data, 0).1.to_string()
}

#[test]
fn tests_part1() {
    assert_eq!("138", part1("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2"));
    assert_eq!("47112", part1(include_str!("day08_input.txt")));
}

#[test]
fn tests_part2() {
    assert_eq!("66", part2("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2"));
    assert_eq!("28237", part2(include_str!("day08_input.txt")));
}
