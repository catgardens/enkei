use log::trace;
use std::path::PathBuf;

use crate::board::Board;
use cursive::{
    view::View,
    views::{LinearLayout, NamedView},
};
use directories::ProjectDirs;

/// return struct with default directories
///
/// # Errors
///
/// This function will return an error if a path cannot be formed,
/// e.g. xdg directories do not exist
pub fn project_dirs() -> anyhow::Result<ProjectDirs> {
    ProjectDirs::from("dev", "catgardens", "enkei")
        .ok_or(anyhow::format_err!("can not construct project directories"))
}

pub fn theme_file() -> anyhow::Result<PathBuf> {
    Ok(project_dirs()?.config_dir().join("theme.toml"))
}

pub fn state_file() -> anyhow::Result<PathBuf> {
    Ok(project_dirs()?.data_dir().join("state.json"))
}

pub mod cli;
mod impl_self;
mod impl_view;
mod themes;

pub struct App {
    pub board: Board,
    pub focus: usize,
    pub inner_view: Box<dyn View>,
}

/// initializes the application and start the tui
///
/// # Errors
///
/// This function will return an error if:
/// - a path to config files cannot be constructed
/// - the theme config can not be parsed
pub fn start() -> anyhow::Result<()> {
    let mut app = App::default();
    app.init()?;
    let mut s = cursive::default();

    s.add_global_callback('q', cursive::Cursive::quit);
    s.add_global_callback('a', |s| {
        let item = crate::item::Item {
            name: "new item".to_string(),
            ..Default::default()
        };
        s.call_on_name("Main", |v: &mut App| v.board.add(&item));
    });

    let layout = NamedView::new(
        "Frame",
        LinearLayout::vertical().child(NamedView::new("Main", app)),
    );
    s.add_layer(layout);

    let theme_path = theme_file()?;
    let theme = theme_path
        .to_str()
        .ok_or(anyhow::format_err!("could not get path to theme.toml"))?;

    if PathBuf::from(theme).exists() {
        trace!("parsing theme file: {theme}");
        let content = std::fs::read_to_string(theme)?;
        s.load_toml(&content)
            .map_err(|e| anyhow::format_err!("could not load theme: {:?}", e))?;
    } else {
        s.set_theme(themes::init_theme());
    }

    s.run();

    // save state on every loop, this means that the staet will always be saved
    // even if the application crashes or is killed (hopefully)
    s.call_on_name("Main", |app: &mut App| app.destroy());
    Ok(())
}
