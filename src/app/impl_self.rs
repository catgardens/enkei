use log::trace;

use crate::Board;
use crate::app::App;

impl App {
    pub fn new() -> Self {
        App {
            board: Board::default(),
            focus: 0,
        }
    }
    pub fn start() -> anyhow::Result<()> {
        trace!("starting tui");
        todo!()
    }
}
