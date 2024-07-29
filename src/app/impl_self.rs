use crate::app::App;
use crate::board::Board;
use cursive::views::LinearLayout;

impl Default for App {
    fn default() -> Self {
        App {
            board: Board::default(),
            focus: 0,
            inner_view: Box::new(LinearLayout::vertical()),
        }
    }
}

impl App {
    /// initializes application state
    ///
    /// - loads state from `~/.local/share/enkei/state.json` on linux
    /// - or `~/Library/Application Support/dev.catgardens.eneki/state.json` on macos
    pub fn init(&mut self) -> anyhow::Result<()> {
        self.board = Board::load_state()?;
        Ok(())
    }
    /// destroys application state
    ///
    /// - saves state to `~/.local/share/enkei/state.json` on linux
    /// - or `~/Library/Application Support/dev.catgardens.eneki/state.json` on macos
    pub fn destroy(&mut self) -> anyhow::Result<()> {
        self.board.save_state()?;
        Ok(())
    }
}
