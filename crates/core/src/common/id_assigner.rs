use std::collections::{hash_map::Entry, HashMap};
use std::hash::Hash;

pub struct IdAssigner<'a, const MAX_SIZE: u16, H: Hash + PartialOrd + Eq + ?Sized> {
    id_map: HashMap<&'a H, u16>,
}

impl<'a, const MAX_SIZE: u16, H: Hash + Hash + PartialOrd + Eq  + ?Sized> IdAssigner<'a, MAX_SIZE, H> {
    pub fn new() -> Self {
        Self {
            id_map: HashMap::new(),
        }
    }

    pub fn id_of(&mut self, name: &'a H) -> Result<u16, String> {
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

    pub fn get_id(&mut self, name: &H) -> Option<u16> {
        self.id_map.get(name).copied()
    }

    pub fn len(&self) -> usize {
        self.id_map.len()
    }
}
