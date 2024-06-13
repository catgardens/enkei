use crate::Board;

mod impl_self;

pub struct App {
    pub board: Board,
    pub focus: usize,
}
