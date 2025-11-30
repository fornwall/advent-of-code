pub struct IdAssigner<'a, const MAX_SIZE: usize, H: Ord + Eq + ?Sized> {
    id_map: [&'a H; MAX_SIZE],
    ids: [u16; MAX_SIZE],
    assigned_count: u16,
}

impl<'a, const MAX_SIZE: usize, H: Ord + Eq + ?Sized> IdAssigner<'a, MAX_SIZE, H> {
    pub const fn new(default: &'a H) -> Self {
        Self {
            id_map: [default; MAX_SIZE],
            ids: [0; MAX_SIZE],
            assigned_count: 0,
        }
    }

    pub fn id_of(&mut self, name: &'a H) -> Result<u16, String> {
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

    pub fn get_id(&self, name: &H) -> Option<u16> {
        self.id_map[0..(self.assigned_count as usize)]
            .binary_search(&name)
            .map_or(None, |idx| Some(self.ids[idx]))
    }

    pub fn original_value_of(&self, id_to_lookup: u16) -> Option<&'a H> {
        self.ids
            .iter()
            .enumerate()
            .find(|&(_, &id)| id == id_to_lookup)
            .map(|(idx, _)| self.id_map[idx])
    }

    pub const fn len(&self) -> usize {
        self.assigned_count as usize
    }
}

#[allow(clippy::unwrap_used)]
#[test]
fn test() {
    let mut id_assigner = IdAssigner::<100, str>::new("");
    assert_eq!(id_assigner.id_of("hi").unwrap(), 0);
    assert_eq!(id_assigner.id_of("hi").unwrap(), 0);
    assert_eq!(id_assigner.id_of("apa").unwrap(), 1);
    assert_eq!(id_assigner.id_of("apa").unwrap(), 1);
    assert_eq!(id_assigner.id_of("hi").unwrap(), 0);
    assert_eq!(id_assigner.id_of("apa").unwrap(), 1);
}
