use crate::input::Input;
use std::collections::HashMap;

#[allow(clippy::use_self)]
#[derive(Eq, PartialEq, Debug)]
enum JsonValue<'a> {
    String(&'a [u8]),
    Number(i32),
    Array(Vec<JsonValue<'a>>),
    Object(HashMap<&'a [u8], JsonValue<'a>>),
    Comma,
    Colon,
    EndOfArray,
    EndOfObject,
    EndOfInput,
}

fn parse<'a>(input: &'a [u8], current_idx: &mut usize) -> Result<JsonValue<'a>, String> {
    if *current_idx == input.len() {
        return Ok(JsonValue::EndOfInput);
    }
    let next_char = input[*current_idx];
    *current_idx += 1;

    Ok(match next_char {
        b'{' => {
            let mut object_map = HashMap::new();
            loop {
                let mut next_key = parse(input, current_idx)?;
                if next_key == JsonValue::Comma {
                    next_key = parse(input, current_idx)?;
                }

                if JsonValue::EndOfObject == next_key {
                    break JsonValue::Object(object_map);
                } else if let JsonValue::String(key) = next_key {
                    let next_colon = parse(input, current_idx)?;
                    if next_colon != JsonValue::Colon {
                        return Err("Invalid JSON - key not followed by colon".to_string());
                    }

                    let next_value = parse(input, current_idx)?;
                    object_map.insert(key, next_value);
                } else {
                    return Err(format!(
                        "Not key or colon in object: {:?} (index={})",
                        next_key, *current_idx
                    ));
                }
            }
        }
        b'}' => JsonValue::EndOfObject,
        b':' => JsonValue::Colon,
        b'[' => {
            let mut array = Vec::new();
            loop {
                let next_value = parse(input, current_idx)?;
                if JsonValue::EndOfArray == next_value {
                    break JsonValue::Array(array);
                } else if JsonValue::EndOfInput == next_value {
                    return Err("Invalid JSON".to_string());
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
            for (idx, &read_char) in input.iter().enumerate().skip(*current_idx) {
                if read_char == b'"' {
                    let start_idx = *current_idx;
                    *current_idx = idx + 1;
                    return Ok(JsonValue::String(&input[start_idx..idx]));
                }
            }
            return Err("Invalid input - no end of string".to_string());
        }
        b'0'..=b'9' | b'-' => {
            let mut idx = *current_idx;
            let (next_char, sign) = if next_char == b'-' {
                let res = (input[idx], -1);
                idx += 1;
                res
            } else {
                (next_char, 1)
            };
            let mut value = sign * i32::from(next_char - b'0');

            loop {
                let read_char = if idx == input.len() { b' ' } else { input[idx] };
                if read_char.is_ascii_digit() {
                    value = value
                        .checked_mul(10_i32)
                        .and_then(|v| v.checked_add(sign * i32::from(read_char - b'0')))
                        .ok_or("Non-i32 number")?;
                } else {
                    *current_idx = idx;
                    break JsonValue::Number(value);
                }
                idx += 1;
            }
        }
        _ => {
            return Err(format!(
                "Invalid char: '{}' at index={}",
                next_char as char, *current_idx
            ));
        }
    })
}

fn sum_json_value(value: &JsonValue, part2: bool) -> i32 {
    match value {
        JsonValue::Number(n) => *n,
        JsonValue::Array(vec) => vec.iter().map(|value| sum_json_value(value, part2)).sum(),
        JsonValue::Object(map) => {
            if part2
                && map
                    .values()
                    .any(|value| value == &JsonValue::String(b"red"))
            {
                0
            } else {
                map.values().map(|value| sum_json_value(value, part2)).sum()
            }
        }
        _ => 0,
    }
}

pub fn solve(input: &Input) -> Result<i32, String> {
    let mut current_idx = 0_usize;
    let json_value = parse(input.text.as_bytes(), &mut current_idx)?;
    let sum = sum_json_value(&json_value, input.is_part_two());
    Ok(sum)
}

#[test]
pub fn test_parse() {
    let mut current_idx = 0_usize;
    assert_eq!(
        Ok(JsonValue::Number(1234)),
        parse(b"1234", &mut current_idx)
    );

    current_idx = 0;
    assert_eq!(
        Ok(JsonValue::String(b"1234")),
        parse(b"\"1234\"", &mut current_idx)
    );

    current_idx = 0;
    assert_eq!(
        Ok(JsonValue::Number(i32::MAX)),
        parse(b"2147483647", &mut current_idx)
    );

    current_idx = 0;
    assert_eq!(
        Ok(JsonValue::Number(i32::MIN)),
        parse(b"-2147483648", &mut current_idx)
    );

    for input in [
        b"2147483648".as_slice(),
        b"-2147483649".as_slice(),
        b"9000000000".as_slice(),
        b"-9000000000".as_slice(),
    ] {
        current_idx = 0;
        assert_eq!(
            Err("Non-i32 number".to_string()),
            parse(input, &mut current_idx)
        );
    }

    current_idx = 0;
    assert_eq!(
        Ok(JsonValue::Array(vec![
            JsonValue::Number(123),
            JsonValue::String(b"abc")
        ])),
        parse(b"[123,\"abc\"]", &mut current_idx)
    );

    current_idx = 0;
    let mut expected_map = HashMap::new();
    let key1 = b"key1";
    let key2 = b"key2";
    let key3 = b"key3";
    expected_map.insert(&key1[..], JsonValue::Number(123));
    expected_map.insert(&key2[..], JsonValue::String(b"abc"));
    expected_map.insert(
        &key3[..],
        JsonValue::Array(vec![JsonValue::Number(-345), JsonValue::String(b"abc")]),
    );
    assert_eq!(
        Ok(JsonValue::Object(expected_map)),
        parse(
            b"{\"key1\":123,\"key2\":\"abc\",\"key3\":[-345,\"abc\"]}",
            &mut current_idx
        )
    );
}

#[test]
pub fn tests() {
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
