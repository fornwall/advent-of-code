use crate::common::md5::Context;
use crate::input::Input;

fn to_hash_chars(hash: &[u8]) -> [u8; 32] {
    let mut hash_chars = [0_u8; 32];
    for i in 0..32 {
        hash_chars[i] = if i % 2 == 0 {
            (hash[i / 2] & 0xF0) >> 4
        } else {
            hash[i / 2] & 0x0F
        };
    }
    hash_chars
}

fn first_triplet(hash: &[u8]) -> Option<u8> {
    let hash_chars = to_hash_chars(hash);
    hash_chars
        .windows(3)
        .find(|w| w[0] == w[1] && w[1] == w[2])
        .map(|w| w[0])
}

fn contains_five_in_a_row(hash: &[u8], desired_char: u8) -> bool {
    let hash_chars = to_hash_chars(hash);
    hash_chars
        .windows(5)
        .any(|w| w[0] == desired_char && w.windows(2).all(|adjacent| adjacent[0] == adjacent[1]))
}

pub fn solve(input: &Input) -> Result<u32, String> {
    let salt = input.text;
    if salt.len() > 8 {
        return Err("Too long salt (max length: 8)".to_string());
    }

    let mut hash_cache = Vec::new();
    let mut orig_hasher = Context::new();
    orig_hasher.consume(salt.as_bytes());

    for i in 0..1000 {
        let mut hasher = orig_hasher.clone();
        hasher.consume(i.to_string().as_bytes());
        if input.is_part_two() {
            for _ in 0..2016 {
                let hash: [u8; 16] = hasher.compute();
                let hash_str = to_hash_chars(&hash)
                    .iter()
                    .map(|&b| if b <= 9 { b'0' + b } else { b'a' + (b - 10) })
                    .collect::<Vec<_>>();
                hasher = Context::new();
                hasher.consume(&hash_str);
            }
        }
        let hash: [u8; 16] = hasher.compute();
        hash_cache.push(hash);
    }

    let mut valid_key_count = 0;
    let mut index = 0;
    loop {
        let current_hash = hash_cache[index % 1000];
        hash_cache[index % 1000] = {
            let content_to_hash = format!("{}{}", salt, index + 1000);
            let mut hasher = Context::new();
            hasher.consume(content_to_hash.as_bytes());
            if input.is_part_two() {
                for _ in 0..2016 {
                    let hash: [u8; 16] = hasher.compute();
                    hasher = Context::new();
                    let hash_str = to_hash_chars(&hash)
                        .iter()
                        .map(|&b| if b <= 9 { b'0' + b } else { b'a' + (b - 10) })
                        .collect::<Vec<_>>();
                    hasher.consume(&hash_str);
                }
            }
            hasher.compute()
        };

        if let Some(triplet_value) = first_triplet(&current_hash)
            && hash_cache
                .iter()
                .any(|hash| contains_five_in_a_row(hash, triplet_value))
        {
            valid_key_count += 1;
            if valid_key_count == 64 {
                return Ok(index as u32);
            }
        }

        if index > 100_000 {
            break;
        }
        index += 1;
    }

    Err("Time out".to_string())
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    let real_input = include_str!("day14_input.txt");
    test_part_one!(real_input => 15168);
    test_part_two!(real_input => 20864);
}
