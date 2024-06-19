#[derive(Debug, Default)]
pub enum State {
    #[default]
    New,        // issue
    InProgress, // draft pr
    Ready,      // pr created
    Done,       // pr merged
}
#[derive(Debug, Default)]
pub enum Priority {
    Low,
    #[default]
    Medium,
    High,
    Urgent,
}
#[derive(Debug, Default)]
pub enum Size {
    Tiny,
    Small,
    #[default]
    Medium,
    Large,
    Massive,
}

#[derive(Debug, Default)]
pub struct Item {
    pub name: String,
    pub desc: String,
    pub state: State,
    pub priority: Priority,
    pub size: Size,
}
