use std::fmt;

use clap::{Parser, Subcommand};

use crate::commands::init::InitArgs;

#[derive(Parser, Debug)]
#[command(name = "scaffold")]
#[command(about = "A scaffolding tool for different programming languages")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Initialize a new project
    Init(InitArgs),
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum Language {
    Rust,
    Python,
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Language::Rust => write!(f, "Rust"),
            Language::Python => write!(f, "Python"),
        }
    }
}
