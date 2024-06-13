use crate::item::Item;

pub struct Board {
    pub items: Vec<Item>,
}

impl Board {
    pub fn default() -> Self {
        Board {
            items: Vec::new(),
        }
    }
    pub fn load_state() -> Self {
        todo!()
    }
    pub fn save_state() -> Self {
        todo!()
    }
}
