use crate::board::Board;
use crate::app::App;

impl App {
    pub fn new() -> Self {
        App {
            board: Board::default(),
            focus: 0,
        }
    }
}
