use std::fmt;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "scaffold")]
#[command(about = "A scaffolding tool for different programming languages")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new project
    Init {
        /// Programming language (rust or python)
        language: Language,
    },
}

#[derive(clap::ValueEnum, Clone)]
enum Language {
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

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init { language } => {
            println!("Initializing project in {}...", language);
            // 这里可以添加具体的初始化逻辑
        },
    }
}
