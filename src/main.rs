use clap::{CommandFactory, Parser};
use clap_complete::generate;
use log::trace;

mod app;
mod board;
mod item;

use app::cli::{Cli, Commands};

fn main() -> anyhow::Result<()> {
    saku_logger::init();
    trace!("logger init");

    let args = Cli::parse();
    //println!("{:?}", args);

    match args.command {
        // Generate the completions and exit immediately
        Some(Commands::Completions { shell }) => {
            let mut cmd = Cli::command();
            let name = cmd.get_name().to_string();
            eprintln!("Generating completions for {shell}");
            generate(shell, &mut cmd, name, &mut std::io::stdout());
            Ok(())
        }
        _ => {
            let mut app = app::App::new();
            app.start()?;
            Ok(())
        }
    }
}
