use crate::board::Board;
use directories::ProjectDirs;

/// return struct with default directories
///
/// # Errors
///
/// This function will return an error if a path cannot be formed,
/// e.g. xdg directories do not exist
pub fn project() -> anyhow::Result<ProjectDirs> {
    ProjectDirs::from("dev", "catgardens", "enkei")
        .ok_or(anyhow::format_err!("can not construct project directories"))
}

pub mod cli;
mod impl_self;
mod impl_view;
mod themes;

#[derive(Debug, Default)]
pub struct App {
    pub board: Board,
    pub focus: usize,
}
