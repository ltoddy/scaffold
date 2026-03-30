use std::fs;
use std::path::Path;

use crate::cli::Language;

pub fn execute(language: &Language) {
    println!("Initializing project in {}...", language);

    match language {
        Language::Rust => {
            println!("Creating Rust project configuration files...");
            create_rust_files();
            println!("Rust project initialized successfully!");
            println!("You can now use 'just build', 'just run', 'just test', 'just fmt', 'just lint' commands");
        },
        Language::Python => {
            println!("Python initialization not yet implemented");
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
            eprintln!("Failed to create justfile: {}", e);
            all_files_created = false;
        }
    } else {
        println!("justfile already exists, skipping...");
    }

    // Create rust-toolchain.toml
    let rust_toolchain_path = Path::new("rust-toolchain.toml");
    if !rust_toolchain_path.exists() {
        let rust_toolchain_content = include_str!("templates/rust/rust-toolchain.toml");
        if let Err(e) = fs::write("rust-toolchain.toml", rust_toolchain_content) {
            eprintln!("Failed to create rust-toolchain.toml: {}", e);
            all_files_created = false;
        }
    } else {
        println!("rust-toolchain.toml already exists, skipping...");
    }

    // Create rustfmt.toml
    let rustfmt_path = Path::new("rustfmt.toml");
    if !rustfmt_path.exists() {
        let rustfmt_content = include_str!("templates/rust/rustfmt.toml");
        if let Err(e) = fs::write("rustfmt.toml", rustfmt_content) {
            eprintln!("Failed to create rustfmt.toml: {}", e);
            all_files_created = false;
        }
    } else {
        println!("rustfmt.toml already exists, skipping...");
    }

    if !all_files_created {
        eprintln!("Warning: Some configuration files could not be created, but initialization continues...");
    }
}
