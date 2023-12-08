use std::hash::Hash;

pub struct IdAssigner<'a, const MAX_SIZE: usize, H: Ord + Eq + ?Sized> {
    //id_map: std::collections::hash_map::HashMap<&'a H, u16>,
    id_map: [&'a H; MAX_SIZE],
    ids: [u16; MAX_SIZE],
    assigned_count: u16,
}

impl<'a, const MAX_SIZE: usize, H: Ord + Eq + ?Sized> IdAssigner<'a, MAX_SIZE, H> {
    pub fn new(default: &'a H) -> Self {
        Self {
            //id_map: Default::default(),
            id_map: [default; MAX_SIZE],
            ids: [0; MAX_SIZE],
            assigned_count: 0,
        }
    }

    pub fn id_of(&mut self, name: &'a H) -> Result<u16, String> {
        /*
        let next_id = self.id_map.len() as u16;
        Ok(match self.id_map.entry(name) {
            std::collections::hash_map::Entry::Vacant(entry) => {
                if usize::from(next_id) == MAX_SIZE {
                    return Err("Too many entries".to_string());
                }
                entry.insert(next_id);
                next_id
            }
            std::collections::hash_map::Entry::Occupied(e) => *e.get(),
        })
         */

        Ok(
            match (&self.id_map[0..(self.assigned_count as usize)]).binary_search(&name) {
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
        //self.id_map.get(name).copied()

        if let Ok(idx) = (&self.id_map[0..(self.assigned_count as usize)]).binary_search(&name) {
            Some(self.ids[idx])
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        //self.id_map.len()
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
