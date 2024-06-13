use crate::board::Board;

pub mod cli;
mod impl_self;
mod impl_view;
mod themes;

pub struct App {
    pub board: Board,
    pub focus: usize,
}
