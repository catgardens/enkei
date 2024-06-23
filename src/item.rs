use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[derive(Debug, Default, Clone)]
pub enum State {
    #[default]
    New,        // issue
    InProgress, // draft pr
    Ready,      // pr created
    Done,       // pr merged
}
#[derive(Serialize, Deserialize)]
#[derive(Debug, Default, Clone)]
pub enum Priority {
    Low,
    #[default]
    Medium,
    High,
    Urgent,
}
#[derive(Serialize, Deserialize)]
#[derive(Debug, Default, Clone)]
pub enum Size {
    Tiny,
    Small,
    #[default]
    Medium,
    Large,
    Massive,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Default, Clone)]
pub struct Item {
    pub name: String,
    pub desc: String,
    pub state: State,
    pub priority: Priority,
    pub size: Size,
}
