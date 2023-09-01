use std::collections::HashMap;

pub struct IdAssigner<const MAX_SIZE: u16> {
    id_map: HashMap<String, u16>,
}

impl<const MAX_SIZE: u16> IdAssigner<MAX_SIZE> {
    pub fn new() -> Self {
        Self {
            id_map: HashMap::new(),
        }
    }

    pub fn id_of(&mut self, name: &str) -> Result<u16, String> {
        if let Some(&id) = self.id_map.get(name) {
            Ok(id)
        } else {
            if self.len() == MAX_SIZE as usize {
                return Err("Too many entries".to_string());
            }
            let next_id = self.id_map.len() as u16;
            self.id_map.insert(name.to_string(), next_id);
            Ok(next_id)
        }
    }

    pub fn get_id(&mut self, name: &str) -> Option<u16> {
        self.id_map.get(name).copied()
    }

    pub fn len(&self) -> usize {
        self.id_map.len()
    }
}
