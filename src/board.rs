use serde::{Deserialize, Serialize};

use crate::item::Item;

#[derive(Serialize, Deserialize)]
#[derive(Debug, Default)]
pub struct Board {
    pub items: Vec<Item>,
}

impl Board {
    pub fn load_state() -> Self {
        todo!()
    }
    pub fn save_state() -> Self {
        todo!()
    }
}
