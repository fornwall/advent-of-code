pub struct IdAssigner<const MAX_SIZE: usize, H: Ord + Eq + Copy + Clone> {
    id_map: [H; MAX_SIZE],
    ids: [u16; MAX_SIZE],
    assigned_count: u16,
}

impl<const MAX_SIZE: usize, H: Ord + Eq + Copy + Clone> IdAssigner<MAX_SIZE, H> {
    pub const fn new(default: H) -> Self {
        Self {
            id_map: [default; MAX_SIZE],
            ids: [0; MAX_SIZE],
            assigned_count: 0,
        }
    }

    pub fn id_of(&mut self, name: H) -> Result<u16, String> {
        Ok(
            match self.id_map[0..(self.assigned_count as usize)].binary_search(&name) {
                Ok(idx) => self.ids[idx],
                Err(idx) => {
                    self.id_map
                        .copy_within(idx..self.assigned_count as usize, idx + 1);
                    self.ids
                        .copy_within(idx..self.assigned_count as usize, idx + 1);
                    let new_id = self.assigned_count;
                    self.assigned_count += 1;
                    self.id_map[idx] = name;
                    self.ids[idx] = new_id;
                    new_id
                }
            },
        )
    }

    pub fn get_id(&mut self, name: &H) -> Option<u16> {
        if let Ok(idx) = self.id_map[0..(self.assigned_count as usize)].binary_search(name) {
            Some(self.ids[idx])
        } else {
            None
        }
    }

    pub const fn len(&self) -> usize {
        self.assigned_count as usize
    }
}

#[allow(clippy::unwrap_used)]
#[test]
fn test() {
    let mut id_assigner = IdAssigner::<100, u16>::new(1);
    assert_eq!(id_assigner.id_of(1).unwrap(), 0);
    assert_eq!(id_assigner.id_of(1).unwrap(), 0);
    assert_eq!(id_assigner.id_of(2).unwrap(), 1);
    assert_eq!(id_assigner.id_of(2).unwrap(), 1);
    assert_eq!(id_assigner.id_of(1).unwrap(), 0);
    assert_eq!(id_assigner.id_of(2).unwrap(), 1);
}
