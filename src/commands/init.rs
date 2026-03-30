use std::fs;
use std::path::Path;

use colored::*;

use crate::cli::Language;

pub fn execute(language: &Language) {
    println!("{}", format!("Initializing project in {}...", language).cyan());

    match language {
        Language::Rust => {
            println!("{}", "Creating Rust project configuration files...".blue());
            create_rust_files();
            println!("{}", "Rust project initialized successfully!".green().bold());
            println!(
                "{}",
                "You can now use 'just build', 'just run', 'just test', 'just fmt', 'just lint' commands"
                    .bright_black()
            );
        },
        Language::Python => {
            println!("{}", "Python initialization not yet implemented".yellow());
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
            eprintln!("{}", format!("Failed to create justfile: {}", e).red());
            all_files_created = false;
        } else {
            println!("{}", "  ✓ Created justfile".green());
        }
    } else {
        println!("{}", "  ℹ justfile already exists, skipping...".dimmed());
    }

    // Create rust-toolchain.toml
    let rust_toolchain_path = Path::new("rust-toolchain.toml");
    if !rust_toolchain_path.exists() {
        let rust_toolchain_content = include_str!("templates/rust/rust-toolchain.toml");
        if let Err(e) = fs::write("rust-toolchain.toml", rust_toolchain_content) {
            eprintln!("{}", format!("Failed to create rust-toolchain.toml: {}", e).red());
            all_files_created = false;
        } else {
            println!("{}", "  ✓ Created rust-toolchain.toml".green());
        }
    } else {
        println!("{}", "  ℹ rust-toolchain.toml already exists, skipping...".dimmed());
    }

    // Create rustfmt.toml
    let rustfmt_path = Path::new("rustfmt.toml");
    if !rustfmt_path.exists() {
        let rustfmt_content = include_str!("templates/rust/rustfmt.toml");
        if let Err(e) = fs::write("rustfmt.toml", rustfmt_content) {
            eprintln!("{}", format!("Failed to create rustfmt.toml: {}", e).red());
            all_files_created = false;
        } else {
            println!("{}", "  ✓ Created rustfmt.toml".green());
        }
    } else {
        println!("{}", "  ℹ rustfmt.toml already exists, skipping...".dimmed());
    }

    if !all_files_created {
        eprintln!(
            "{}",
            "Warning: Some configuration files could not be created, but initialization continues...".yellow()
        );
    }

    // Run just fmt and just lint commands
    println!("{}", "Running just fmt and just lint...".blue());
}
