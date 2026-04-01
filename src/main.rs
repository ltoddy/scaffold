pub mod cli;
pub mod commands;
pub mod shell;
pub mod theme;

use clap::Parser;

use crate::cli::Cli;
use crate::commands::init;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        cli::Commands::Init(args) => {
            init::execute(args);
        },
    }
}
