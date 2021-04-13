use crate::Input;
use std::collections::HashMap;

#[derive(Eq, PartialEq, Debug)]
enum JsonValue {
    String(String),
    Number(i32),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
    Comma,
    Colon,
    EndOfArray,
    EndOfObject,
    EndOfInput,
}

fn parse(input: &[u8], current_idx: &mut usize) -> JsonValue {
    let next_char = input[*current_idx];
    if *current_idx == input.len() {
        return JsonValue::EndOfInput;
    }
    *current_idx += 1;

    match next_char {
        b'{' => {
            let mut object_map = HashMap::new();
            loop {
                let mut next_key = parse(input, current_idx);
                if next_key == JsonValue::Comma {
                    next_key = parse(input, current_idx);
                }

                if JsonValue::EndOfObject == next_key {
                    return JsonValue::Object(object_map);
                } else if let JsonValue::String(key) = next_key {
                    let next_colon = parse(input, current_idx);
                    assert!(next_colon == JsonValue::Colon);

                    let next_value = parse(input, current_idx);
                    object_map.insert(key, next_value);
                } else {
                    panic!(
                        "Not key or colon in object: {:?} (index={})",
                        next_key, *current_idx
                    );
                }
            }
        }
        b'}' => JsonValue::EndOfObject,
        b':' => JsonValue::Colon,
        b'[' => {
            let mut array = Vec::new();
            loop {
                let next_value = parse(input, current_idx);
                if JsonValue::EndOfArray == next_value {
                    return JsonValue::Array(array);
                } else if JsonValue::Comma == next_value {
                    // Ignore
                } else {
                    array.push(next_value);
                }
            }
        }
        b']' => JsonValue::EndOfArray,
        b',' => JsonValue::Comma,
        b'"' => {
            let mut string = String::new();
            let mut idx = *current_idx;
            loop {
                let read_char = input[idx];
                if read_char == b'"' {
                    *current_idx = idx + 1;
                    return JsonValue::String(string);
                } else {
                    string.push(read_char as char);
                }
                idx += 1;
            }
        }
        b'0'..=b'9' | b'-' => {
            let mut string = String::new();
            string.push(next_char as char);

            let mut idx = *current_idx;
            loop {
                let read_char = if idx == input.len() { b' ' } else { input[idx] };
                if (b'0'..=b'9').contains(&read_char) {
                    string.push(read_char as char);
                } else {
                    *current_idx = idx;
                    let number = string.parse::<i32>().unwrap_or_default();
                    return JsonValue::Number(number);
                }
                idx += 1;
            }
        }
        _ => {
            panic!(
                "Invalid char: '{}' at index={}",
                next_char as char, *current_idx
            );
        }
    }
}

fn sum_json_value(value: &JsonValue, part2: bool) -> i32 {
    match value {
        JsonValue::Number(n) => *n,
        JsonValue::Array(vec) => vec.iter().map(|value| sum_json_value(value, part2)).sum(),
        JsonValue::Object(map) => {
            if part2
                && map
                    .values()
                    .any(|value| value == &JsonValue::String("red".to_string()))
            {
                0
            } else {
                map.values().map(|value| sum_json_value(value, part2)).sum()
            }
        }
        _ => 0,
    }
}

pub fn solve(input: &mut Input) -> Result<i32, String> {
    let mut current_idx = 0_usize;
    let json_value = parse(input.text.as_bytes(), &mut current_idx);
    let sum = sum_json_value(&json_value, input.is_part_two());
    Ok(sum)
}

#[test]
pub fn test_parse() {
    let mut current_idx = 0_usize;
    assert_eq!(JsonValue::Number(1234), parse(b"1234", &mut current_idx));

    current_idx = 0;
    assert_eq!(
        JsonValue::String("1234".to_string()),
        parse(b"\"1234\"", &mut current_idx)
    );

    current_idx = 0;
    assert_eq!(
        JsonValue::Array(vec![
            JsonValue::Number(123),
            JsonValue::String("abc".to_string())
        ]),
        parse(b"[123,\"abc\"]", &mut current_idx)
    );

    current_idx = 0;
    let mut expected_map = HashMap::new();
    expected_map.insert("key1".to_string(), JsonValue::Number(123));
    expected_map.insert("key2".to_string(), JsonValue::String("abc".to_string()));
    expected_map.insert(
        "key3".to_string(),
        JsonValue::Array(vec![
            JsonValue::Number(-345),
            JsonValue::String("abc".to_string()),
        ]),
    );
    assert_eq!(
        JsonValue::Object(expected_map),
        parse(
            b"{\"key1\":123,\"key2\":\"abc\",\"key3\":[-345,\"abc\"]}",
            &mut current_idx
        )
    );
}

#[test]
pub fn tests() {
    use crate::{test_part_one, test_part_two};
    test_part_one!("{\"a\":{\"b\":4},\"c\":-1}" => 3);
    test_part_one!("[1,2,3]" => 6);
    test_part_one!("{\"a\":2,\"b\":4}" => 6);
    test_part_one!("[[[3]]]" => 3);
    test_part_one!("{\"a\":[-1,1]}" => 0);
    test_part_one!("[-1,{\"a\":1}]" => 0);
    test_part_one!("[]" => 0);
    test_part_one!("{}" => 0);

    test_part_two!("[1,{\"c\":\"red\",\"b\":2},3]" => 4);

    let real_input = include_str!("day12_input.txt");
    test_part_one!(real_input => 111_754);
    test_part_two!(real_input => 65_402);
}
