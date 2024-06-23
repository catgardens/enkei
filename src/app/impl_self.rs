use crate::app::App;
use crate::board::Board;
use cursive::views::LinearLayout;

impl App {
    pub fn new() -> Self {
        App {
            board: Board::default(),
            focus: 0,
            inner_view: Box::new(LinearLayout::vertical()),
        }
    }
    /// initializes application state
    ///
    /// - loads state from `~/.local/share/enkei/state.json`
    pub fn init(&mut self) -> anyhow::Result<()> {
        self.board = Board::load_state()?;
        Ok(())
    }
    /// destroys application state
    ///
    /// - saves state to `~/.local/share/enkei/state.json`
    pub fn destroy(&mut self) -> anyhow::Result<()> {
        self.board.save_state()?;
        Ok(())
    }
}
