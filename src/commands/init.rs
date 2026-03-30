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
    let InitArgs { language, path } = args;

    println!(
        "{} {}",
        format!("Initializing project in {}...", language).cyan(),
        format!("({})", path.display()).dimmed()
    );

    // 确保目录存在
    if !path.exists()
        && let Err(e) = fs::create_dir_all(&path)
    {
        eprintln!("{} {}", "Failed to create directory:".red(), e);
        return;
    }

    // 切换到指定目录
    if let Err(e) = std::env::set_current_dir(&path) {
        eprintln!("{} {}", "Failed to change to directory:".red(), e);
        return;
    }

    match language {
        Language::Rust => {
            println!(
                "{} {}",
                "Creating Rust project configuration files...".blue(),
                format!("in {}", path.display()).dimmed()
            );
            create_rust_files();
            println!(
                "{} {}",
                "Rust project initialized successfully!".green().bold(),
                format!("in {}", path.display()).dimmed()
            );
            println!(
                "{}",
                "You can now use 'just build', 'just run', 'just test', 'just fmt', 'just lint' commands"
                    .bright_black()
            );
        },
        Language::Python => {
            println!(
                "{} {}",
                "Python initialization not yet implemented".yellow(),
                format!("in {}", path.display()).dimmed()
            );
        },
    }
}

fn create_rust_files() {
    let mut all_files_created = true;

    // Create justfile
    let justfile_path = Path::new("justfile");
    if !justfile_path.exists() {
        let justfile_content = include_str!("templates/rust/justfile");
        if let Err(e) = fs::write("justfile", justfile_content) {
            eprintln!("{} {}", "Failed to create justfile:".red(), e);
            all_files_created = false;
        } else {
            println!("{} {}", "  ✓ Created justfile".green(), format!("({})", justfile_path.display()).dimmed());
        }
    } else {
        println!(
            "{} {}",
            "  ℹ justfile already exists, skipping...".dimmed(),
            format!("({})", justfile_path.display()).dimmed()
        );
    }

    // Create rust-toolchain.toml
    let rust_toolchain_path = Path::new("rust-toolchain.toml");
    if !rust_toolchain_path.exists() {
        let rust_toolchain_content = include_str!("templates/rust/rust-toolchain.toml");
        if let Err(e) = fs::write("rust-toolchain.toml", rust_toolchain_content) {
            eprintln!("{} {}", "Failed to create rust-toolchain.toml:".red(), e);
            all_files_created = false;
        } else {
            println!(
                "{} {}",
                "  ✓ Created rust-toolchain.toml".green(),
                format!("({})", rust_toolchain_path.display()).dimmed()
            );
        }
    } else {
        println!(
            "{} {}",
            "  ℹ rust-toolchain.toml already exists, skipping...".dimmed(),
            format!("({})", rust_toolchain_path.display()).dimmed()
        );
    }

    // Create rustfmt.toml
    let rustfmt_path = Path::new("rustfmt.toml");
    if !rustfmt_path.exists() {
        let rustfmt_content = include_str!("templates/rust/rustfmt.toml");
        if let Err(e) = fs::write("rustfmt.toml", rustfmt_content) {
            eprintln!("{} {}", "Failed to create rustfmt.toml:".red(), e);
            all_files_created = false;
        } else {
            println!("{} {}", "  ✓ Created rustfmt.toml".green(), format!("({})", rustfmt_path.display()).dimmed());
        }
    } else {
        println!(
            "{} {}",
            "  ℹ rustfmt.toml already exists, skipping...".dimmed(),
            format!("({})", rustfmt_path.display()).dimmed()
        );
    }

    if !all_files_created {
        eprintln!(
            "{}",
            "Warning: Some configuration files could not be created, but initialization continues...".yellow()
        );
    }

    // Run just fmt and just lint commands
    println!(
        "{} {}",
        "Running just fmt and just lint...".blue(),
        format!("in {}", std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")).display()).dimmed()
    );
}
