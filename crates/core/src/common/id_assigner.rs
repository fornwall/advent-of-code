use std::collections::{hash_map::Entry, HashMap};

pub struct IdAssigner<'a, const MAX_SIZE: u16> {
    id_map: HashMap<&'a str, u16>,
}

impl<'a, const MAX_SIZE: u16> IdAssigner<'a, MAX_SIZE> {
    pub fn new() -> Self {
        Self {
            id_map: HashMap::new(),
        }
    }

    pub fn id_of(&mut self, name: &'a str) -> Result<u16, String> {
        let next_id = self.id_map.len() as u16;
        Ok(match self.id_map.entry(name) {
            Entry::Vacant(entry) => {
                if next_id == MAX_SIZE {
                    return Err("Too many entries".to_string());
                }
                entry.insert(next_id);
                next_id
            }
            Entry::Occupied(e) => *e.get(),
        })
    }

    pub fn get_id(&mut self, name: &str) -> Option<u16> {
        self.id_map.get(name).copied()
    }

    pub fn len(&self) -> usize {
        self.id_map.len()
    }
}
