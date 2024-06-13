use crate::app::App;
use crate::board::Board;

impl App {
    pub fn new() -> Self {
        App {
            board: Board::default(),
            focus: 0,
        }
    }
}
