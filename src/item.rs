pub enum State {
    New, // issue
    InProgress, // draft pr
    Ready, // pr created
    Done, // pr merged
}
pub enum Priority {
    Low,
    Medium,
    High,
    Urgent,
}

pub enum Size {
    Tiny,
    Small,
    Medium,
    Large,
    Massive,
}

pub struct Item {
    pub name: String,
    pub desc: String,
    pub state: State,
    pub priority: Priority,
    pub size: Size,
}
