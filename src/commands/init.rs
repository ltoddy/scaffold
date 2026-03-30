use std::fs;
use std::path::{Path, PathBuf};

use clap::Parser;
use colored::*;

use crate::cli::Language;

#[derive(Parser, Debug)]
pub struct InitArgs {
    /// Programming language (rust or python)
    language: Language,

    /// Project directory path (optional, defaults to current directory)
    #[arg(default_value = ".")]
    path: PathBuf,
}

pub fn execute(args: InitArgs) {
    println!(
        "{} {}",
        format!("Initializing project in {}...", args.language).cyan(),
        format!("({})", args.path.display()).dimmed()
    );

    // 确保目录存在
    if !args.path.exists()
        && let Err(e) = fs::create_dir_all(&args.path)
    {
        eprintln!("{} {}", "Failed to create directory:".red(), e);
        return;
    }

    match args.language {
        Language::Rust => create_rust_files(&args.path),
        Language::Python => {
            println!(
                "{} {}",
                "Python initialization not yet implemented".yellow(),
                format!("in {}", args.path.display()).dimmed()
            );
        },
    }
}

fn create_rust_files(root: &Path) {
    println!("{} {}", "Creating Rust project configuration files...".blue(), format!("in {}", root.display()).dimmed());

    let mut all_files_created = true;

    let files = vec![
        (root.join("rust-toolchain.toml"), include_str!("templates/rust/rust-toolchain.toml")),
        (root.join("rustfmt.toml"), include_str!("templates/rust/rustfmt.toml")),
        (root.join("justfile"), include_str!("templates/rust/justfile")),
    ];

    for (file_path, content) in files {
        if !file_path.exists() {
            if let Err(err) = fs::write(&file_path, content) {
                eprintln!("{} {}", "Failed to create justfile:".red(), err);
                all_files_created = false;
            } else {
                println!("{} {}", "  ✓ Created justfile".green(), format!("({})", file_path.display()).dimmed());
            }
        } else {
            println!(
                "{} {}",
                "  ℹ justfile already exists, skipping...".dimmed(),
                format!("({})", file_path.display()).dimmed()
            );
        }
    }

    if !all_files_created {
        eprintln!(
            "{}",
            "Warning: Some configuration files could not be created, but initialization continues...".yellow()
        );
    } else {
        println!(
            "{} {}",
            "Rust project initialized successfully!".green().bold(),
            format!("in {}", root.display()).dimmed()
        );
    }
}
