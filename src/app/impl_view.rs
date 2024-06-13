use crate::app::App;

use crate::app::themes;

use cursive::views::TextView;

impl App {
    pub fn start(&mut self) -> anyhow::Result<()> {
        let mut s = cursive::default();

        s.add_global_callback('q', cursive::Cursive::quit);

        s.add_layer(TextView::new("Welcome to enkei! Press <q> to quit."));

        // s.load_toml(include_str!("<path_to_theme_file>.toml")).unwrap();
        s.set_theme(themes::init_theme());
        s.run();
        Ok(())
    }
}
