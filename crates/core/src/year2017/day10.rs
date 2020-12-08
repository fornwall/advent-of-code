pub fn solution(input_string: &str, part1: bool) -> Result<String, String> {
    const SIZE: usize = 256;
    let mut list: Vec<u8> = (0..SIZE).map(|i| i as u8).collect();

    let mut current_position = 0;
    let mut skip_size = 0;

    let input = if part1 {
        input_string
            .split(',')
            .map(|length| {
                length
                    .parse::<u8>()
                    .map_err(|e| format!("Invalid length: {}", e.to_string()))
            })
            .collect::<Result<Vec<_>, _>>()?
    } else {
        let to_append = [17_u8, 31_u8, 73_u8, 47_u8, 23_u8];
        input_string
            .bytes()
            .chain(to_append.iter().copied())
            .collect()
    };

    for _round in 0..(if part1 { 1 } else { 64 }) {
        for &length in &input {
            let length = length as usize;

            // "Reverse the order of that length of elements in the list, starting with the element at the current position."
            for i in 0..(length / 2) {
                list.swap(
                    (current_position + i) % SIZE,
                    (current_position + length - 1 - i) % SIZE,
                );
            }

            // "Move the current position forward by that length plus the skip size."
            current_position = (current_position + length + skip_size) % SIZE;

            // "Increase the skip size by one."
            skip_size += 1;
        }
    }

    Ok(if part1 {
        (list[0] as u32 * list[1] as u32).to_string()
    } else {
        list.chunks(16)
            .map(|block| block.iter().fold(0, |acc, x| acc ^ x))
            .map(|number| format!("{:02x}", number))
            .collect()
    })
}

pub fn part1(input_string: &str) -> Result<String, String> {
    solution(input_string, true)
}

pub fn part2(input_string: &str) -> Result<String, String> {
    solution(input_string, false)
}

#[test]
fn test_part1() {
    assert_eq!(
        Ok("62238".to_string()),
        part1(include_str!("day10_input.txt"))
    );
}

#[test]
fn test_part2() {
    assert_eq!(
        Ok("2b0c9cc0449507a0db3babd57ad9e8d8".to_string()),
        part2(include_str!("day10_input.txt"))
    );
}
