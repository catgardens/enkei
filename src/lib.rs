pub mod item;

pub mod app;

pub struct Board {
    pub items: Vec<item::Item>,
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
