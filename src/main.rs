use clap::{CommandFactory, Parser};
use clap_complete::generate;
use log::trace;

mod cli;
use cli::{Cli, Commands};

mod tui;

fn main() {
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
            std::process::exit(0);
        }
        _ => {
            tui::open();
        }
    }
}
