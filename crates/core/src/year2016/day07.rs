use crate::Input;
use std::collections::HashSet;

fn has_abba(ip: &&str) -> bool {
    let ip_bytes = ip.as_bytes();
    let mut in_hypernet_sequence = false;
    let mut has_abba = false;

    for (idx, &c) in ip_bytes.iter().enumerate() {
        if c == b'[' {
            in_hypernet_sequence = true;
        } else if c == b']' {
            in_hypernet_sequence = false;
        } else if ip_bytes.len() > idx + 3
            && ip_bytes[idx + 3] == c
            && ip_bytes[idx + 1] != c
            && ip_bytes[idx + 1] == ip_bytes[idx + 2]
        {
            if in_hypernet_sequence {
                return false;
            } else {
                has_abba = true;
            }
        }
    }

    has_abba
}

fn supports_ssl(ip: &&str) -> bool {
    let ip_bytes = ip.as_bytes();
    let mut in_hypernet_sequence = false;
    let mut abas = HashSet::new();
    let mut babs = HashSet::new();

    for (idx, &c) in ip_bytes.iter().enumerate() {
        if c == b'[' {
            in_hypernet_sequence = true;
        } else if c == b']' {
            in_hypernet_sequence = false;
        } else if ip_bytes.len() > idx + 2 && ip_bytes[idx + 2] == c && ip_bytes[idx + 1] != c {
            if in_hypernet_sequence {
                abas.insert((c, ip_bytes[idx + 1]));
            } else {
                babs.insert((ip_bytes[idx + 1], c));
            }
        }
    }

    abas.intersection(&babs).count() > 0
}

pub fn solve(input: &mut Input) -> Result<usize, String> {
    Ok(input
        .text
        .lines()
        .filter(if input.is_part_one() {
            has_abba
        } else {
            supports_ssl
        })
        .count())
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    test_part_one!("abba[mnop]qrst" => 1);
    test_part_one!("abcd[bddb]xyyx" => 0);
    test_part_one!("aaaa[qwer]tyui" => 0);
    test_part_one!("ioxxoj[asdfgh]zxcvbn" => 1);

    test_part_two!("aba[bab]xyz" => 1);
    test_part_two!("xyx[xyx]xyx" => 0);
    test_part_two!("aaa[kek]eke" => 1);
    test_part_two!("zazbz[bzb]cdb" => 1);

    let real_input = include_str!("day07_input.txt");
    test_part_one!(real_input => 105);
    test_part_two!(real_input => 258);
}
