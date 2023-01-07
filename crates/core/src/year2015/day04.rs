use crate::common::int_to_ascii::IntToAsciiContext;
use crate::common::md5::Context;
use crate::input::Input;

pub fn solve(input: &Input) -> Result<u32, String> {
    const MAX_INDEX: u32 = 100_000_000;

    let mut ascii_bytes_context = IntToAsciiContext::new();
    let secret_key = input.text.as_bytes();
    let mut hasher = Context::new();
    hasher.consume(secret_key);

    for index in 0..MAX_INDEX {
        let mut index_hasher = hasher.clone();
        index_hasher.consume(ascii_bytes_context.ascii_bytes(index));
        let output: [u8; 16] = index_hasher.compute();

        // Check if hash starts with five/six zeros without converting it to a string:
        if output[..2] == [0, 0] && output[2] <= input.part_values(0x0F, 0) {
            return Ok(index);
        }
    }

    Err(format!("Aborting after {} iterations", MAX_INDEX))
}

#[test]
pub fn tests() {
    use crate::input::{test_part_one, test_part_two};

    let real_input = include_str!("day04_input.txt");
    test_part_one!(real_input => 117_946);
    test_part_two!(real_input => 3_938_038);
}
