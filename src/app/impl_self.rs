use crate::app::App;
use crate::board::Board;
use crate::item::Item;

impl App {
    pub fn new() -> Self {
        App {
            board: Board::default(),
            focus: 0,
        }
    }
    /// initializes the application and start the tui
    pub fn start(&mut self) -> anyhow::Result<()> {
        self.init()?;
        self.start_view()?;
        Ok(())
    }
    /// initializes application state
    ///
    /// - loads state from `~/.local/share/enkei/state.json`
    pub fn init(&mut self) -> anyhow::Result<()> {
        self.board = Board::load_state()?;
        Ok(())
    }
}
