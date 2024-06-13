use crate::board::Board;
use directories::ProjectDirs;

pub fn project() -> anyhow::Result<ProjectDirs> {
    ProjectDirs::from("dev", "catgardens", "enkei")
        .ok_or(anyhow::format_err!("can not construct project directories"))
}

pub mod cli;
mod impl_self;
mod impl_view;
mod themes;

pub struct App {
    pub board: Board,
    pub focus: usize,
}
