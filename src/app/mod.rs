use crate::board::Board;

pub(crate) mod themes;
mod impl_self;
mod impl_view;

pub struct App {
    pub board: Board,
    pub focus: usize,
}
