use clap::{Parser, Subcommand};
use clap_complete::Shell;

#[derive(Debug, Parser)]
#[command(name = env!("CARGO_PKG_NAME"), about, version, author)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Create shell completion for enkei
    #[command(arg_required_else_help = true)]
    Completions {
        /// The shell that completions should be generated for
        shell: Shell,
    },
}
