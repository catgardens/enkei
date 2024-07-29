use log::{error, info};
use serde::{Deserialize, Serialize};
use std::fs;

use crate::app::state_file;
use crate::item::Item;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Board {
    pub items: Vec<Item>,
}

impl Board {
    pub fn add(&mut self, item: &Item) {
        self.items.push(item.clone());
    }

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

        if let Some(parent) = path.parent() {
            if let Err(e) = fs::create_dir_all(parent) {
                error!("failed to create directories: {:?}", e);
                return Err(e.into());
            }
        }

        match fs::write(path, contents) {
            Ok(()) => {
                info!("state saved");
            }
            Err(e) => {
                error!("failed to write state to file: {:?}", e);
                return Err(e.into());
            }
        }

        Ok(())
    }
}
