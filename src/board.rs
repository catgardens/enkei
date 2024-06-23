use serde::{Deserialize, Serialize};

use crate::app::state_file;
use crate::item::Item;

#[derive(Serialize, Deserialize)]
#[derive(Debug, Default)]
pub struct Board {
    pub items: Vec<Item>,
}

impl Board {
    pub fn add(&mut self, item: &Item) -> anyhow::Result<()> {
        self.items.push(item.clone());
        Ok(())
    }
}

impl Board {
    pub fn load_state() -> anyhow::Result<Self> {
        let path = state_file()?;
        if path.exists() {
            let contents = std::fs::read_to_string(&path)?;
            let v = serde_json::from_str(&contents)?;
            return Ok(v);
        }
        Ok(Self::default())
    }
    pub fn save_state(&self) -> anyhow::Result<()> {
        let path = state_file()?;
        let contents = serde_json::to_string(&self)?;
        std::fs::write(path, contents)?;
        Ok(())
    }
}
