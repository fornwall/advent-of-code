pub struct IntToAsciiContext {
    digits: [u8; 10],
}

impl IntToAsciiContext {
    pub const fn new() -> Self {
        Self { digits: [0; 10] }
    }
    pub fn ascii_bytes(&mut self, value: u32) -> &[u8] {
        let mut current_value = value;
        let mut current_idx = 0;
        while current_value > 0 || current_idx == 0 {
            self.digits[current_idx] = (current_value % 10 + 48) as u8;
            current_idx += 1;
            current_value /= 10;
        }
        self.digits[0..current_idx].reverse();
        &self.digits[0..current_idx]
    }
}

#[test]
fn to_ascii() {
    let mut context = IntToAsciiContext::new();
    assert_eq!(context.ascii_bytes(1), &[b'1']);
    assert_eq!(context.ascii_bytes(0), &[b'0']);
    assert_eq!(context.ascii_bytes(134), b"134");
    assert_eq!(context.ascii_bytes(4_294_967_295), b"4294967295");
}
