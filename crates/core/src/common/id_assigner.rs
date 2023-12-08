use std::hash::Hash;

pub struct IdAssigner<'a, const MAX_SIZE: usize, H: Hash + Eq + ?Sized> {
    id_map: std::collections::hash_map::HashMap<&'a H, u16>,
    //id_map: [&'a H; MAX_SIZE],
    //assigned_count: u16,
}

impl<'a, const MAX_SIZE: usize, H: Hash + Eq + ?Sized> IdAssigner<'a, MAX_SIZE, H> {
    pub fn new(default: &'a H) -> Self {
        Self {
            id_map: Default::default(),
            //id_map: [default; MAX_SIZE],
            //assigned_count: 0,
        }
    }

    pub fn id_of(&mut self, name: &'a H) -> Result<u16, String> {
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

        /*
                for (idx, &el) in self.id_map.iter().enumerate() {
            if el == name {
                return Ok(idx as u16);
            }
        }
        if usize::from(self.assigned_count) == MAX_SIZE {
            return Err("Too many entries".to_string());
        }
        let new_id = self.assigned_count;
        self.assigned_count += 1;
        self.id_map[new_id as usize] = name;
        Ok(new_id)

                     */
    }

    pub fn get_id(&mut self, name: &H) -> Option<u16> {
        self.id_map.get(name).copied()
        /*
        for (idx, &el) in self.id_map.iter().enumerate() {
            if el == name {
                return Some(idx as u16);
            }
        }
        None
             */
    }

    pub fn len(&self) -> usize {
        self.id_map.len()
        //self.assigned_count as usize
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
