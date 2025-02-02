pub struct ArrayDeque<const MAX_SIZE: usize, H: Eq + Copy + Clone + Default> {
    pub elements: [H; MAX_SIZE],
    head: usize,
    tail: usize,
}

impl<const MAX_SIZE: usize, H: Eq + Copy + Clone + Default> ArrayDeque<MAX_SIZE, H> {
    pub fn new() -> Self {
        Self {
            elements: [Default::default(); MAX_SIZE],
            head: 0,
            tail: 0,
        }
    }

    pub fn push_back(&mut self, element: H) -> Result<(), String> {
        let next_tail = (self.tail + 1) % MAX_SIZE;
        if next_tail == self.head {
            return Err("Too many elements pushed".to_string());
        }
        self.elements[self.tail] = element;
        self.tail = (self.tail + 1) % MAX_SIZE;
        Ok(())
    }

    pub const fn pop_front(&mut self) -> Option<H> {
        if self.head == self.tail {
            return None;
        }
        let result = self.elements[self.head];
        self.head = (self.head + 1) % MAX_SIZE;
        Some(result)
    }
}
