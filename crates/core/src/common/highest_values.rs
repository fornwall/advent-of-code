/// The highest `NUM` values out of the values provided to `on_value(new_value)`.
pub struct HighestValues<const NUM: usize> {
    pub values: [u64; NUM],
}

impl<const NUM: usize> HighestValues<NUM> {
    pub const fn new() -> Self {
        Self { values: [0; NUM] }
    }

    pub fn on_value(&mut self, new_value: u64) {
        self.values.sort_unstable();
        for existing_value in self.values.iter_mut() {
            if *existing_value < new_value {
                *existing_value = new_value;
                return;
            }
        }
    }

    pub fn sum(&self) -> u64 {
        self.values.iter().sum()
    }
}

impl<const NUM: usize> FromIterator<u64> for HighestValues<NUM> {
    fn from_iter<I: IntoIterator<Item = u64>>(iter: I) -> Self {
        let mut highest = Self::new();
        for i in iter {
            highest.on_value(i);
        }
        highest
    }
}
