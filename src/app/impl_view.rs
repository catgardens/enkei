use crate::app::{themes, App};
use std::path::PathBuf;

use cursive::views::TextView;
use log::trace;

use super::project;

impl App {
    pub fn start(&mut self) -> anyhow::Result<()> {
        let mut s = cursive::default();

        s.add_global_callback('q', cursive::Cursive::quit);

        s.add_layer(TextView::new("Welcome to enkei! Press <q> to quit."));

        let them_file = project()?.config_dir().join("theme.toml");
        let theme_file = them_file.to_str().ok_or(anyhow::format_err!("could not get path to theme.toml"))?;
        if PathBuf::from(theme_file).exists() {
            trace!("parsing theme file: {theme_file}");
            let content = std::fs::read_to_string(theme_file)?;
            s.load_toml(&content).map_err(|e| anyhow::format_err!("could not load theme: {:?}", e))?;
        } else {
            s.set_theme(themes::init_theme());
        }
        s.run();
        Ok(())
    }
}