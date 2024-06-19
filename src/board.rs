use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::{app::project, item::Item};

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
    fn state_path() -> anyhow::Result<PathBuf> {
        Ok(project()?.data_dir().join("state.json"))
    }
    pub fn load_state() -> anyhow::Result<Self> {
        let path = Self::state_path()?;
        if path.exists() {
            let contents = std::fs::read_to_string(&path)?;
            let v = serde_json::from_str(&contents)?;
            return Ok(v);
        }
        Ok(Self::default())
    }
    pub fn save_state(&self) -> anyhow::Result<()> {
        let path = Self::state_path()?;
        let contents = serde_json::to_string(&self)?;
        std::fs::write(path, contents)?;
        Ok(())
    }
}
