use std::fs;
use std::path::{Path, PathBuf};

use clap::Parser;
use colored::*;

use crate::cli::Language;
use crate::shell::shell;
use crate::theme::{self, Theme};

#[derive(Parser, Debug)]
pub struct InitArgs {
    /// Programming language (rust or python)
    language: Language,

    /// Project directory path (optional, defaults to current directory)
    #[arg(default_value = ".")]
    path: PathBuf,
}

pub fn execute(args: InitArgs) {
    let theme = theme::detect();

    println!(
        "{} {}",
        format!("Initializing project in {}...", args.language).color(theme.cyan()),
        format!("({})", args.path.display()).dimmed()
    );

    if !args.path.exists()
        && let Err(e) = fs::create_dir_all(&args.path)
    {
        eprintln!("{} {}", "Failed to create directory:".color(theme.red()), e);
        return;
    }

    match args.language {
        Language::Rust => init_rust_files(&args.path),
        Language::Python => init_python_files(&args.path),
    }
}

fn init_rust_files(root: &Path) {
    let theme = theme::detect();
    println!(
        "{} {}",
        "Creating Rust project configuration files...".color(theme.blue()),
        format!("in {}", root.display()).dimmed()
    );

    let cargo_toml_path = root.join("Cargo.toml");
    if !cargo_toml_path.exists() {
        println!("{} {}", "Running cargo init...".color(theme.cyan()), format!("in {}", root.display()).dimmed());

        shell("cargo init", root);
    } else {
        println!(
            "{} {}",
            "  ℹ Cargo.toml already exists, skipping cargo init...".dimmed(),
            format!("({})", cargo_toml_path.display()).dimmed()
        );
    }

    let mut all_files_created = true;

    let files = vec![
        ("rust-toolchain.toml", root.join("rust-toolchain.toml"), include_str!("templates/rust/rust-toolchain.toml")),
        ("rustfmt.toml", root.join("rustfmt.toml"), include_str!("templates/rust/rustfmt.toml")),
        ("justfile", root.join("justfile"), include_str!("templates/rust/justfile")),
    ];

    for (filename, file_path, content) in files {
        let colored_filename = filename.color(theme.magenta());

        if !file_path.exists() {
            if let Err(err) = fs::write(&file_path, content) {
                eprintln!("{} {} {}", "Failed to create".color(theme.red()), colored_filename, err);
                all_files_created = false;
            } else {
                println!(
                    "{} {} {}",
                    "  ✓ Created".color(theme.green()),
                    colored_filename,
                    format!("({})", file_path.display()).dimmed()
                );
            }
        } else {
            println!(
                "{} {} {} {}",
                "  ℹ".dimmed(),
                colored_filename,
                "already exists, skipping...".dimmed(),
                format!("({})", file_path.display()).dimmed()
            );
        }
    }

    if !all_files_created {
        eprintln!(
            "{}",
            "Warning: Some configuration files could not be created, but initialization continues..."
                .color(theme.yellow())
        );
    } else {
        println!(
            "{} {}",
            "Rust project initialized successfully!".color(theme.green()).bold(),
            format!("in {}", root.display()).dimmed()
        );
    }
}

fn init_python_files(root: &Path) {
    let theme = theme::detect();

    println!(
        "{} {}",
        "Creating Python project configuration files...".color(theme.blue()),
        format!("in {}", root.display()).dimmed()
    );

    let pyproject_path = root.join("pyproject.toml");
    if pyproject_path.exists() {
        println!(
            "{} {}",
            "  ℹ pyproject.toml already exists, skipping uv init...".dimmed(),
            format!("({})", pyproject_path.display()).dimmed()
        );
    } else {
        run_uv_init(root, &theme);
    }

    run_uv_add_dev(root, &theme);

    create_python_justfile(root, &theme);

    println!(
        "{} {}",
        "Python project initialized successfully!".color(theme.green()).bold(),
        format!("in {}", root.display()).dimmed()
    );
}

fn run_uv_init(root: &Path, theme: &Theme) {
    println!("{} {}", "Running uv init...".color(theme.cyan()), format!("in {}", root.display()).dimmed());

    shell(&format!("uv init --directory {}", root.to_str().unwrap_or(".")), root);
}

fn run_uv_add_dev(root: &Path, theme: &Theme) {
    println!("{} {}", "Running uv add --dev ruff ty...".color(theme.cyan()), format!("in {}", root.display()).dimmed());

    shell("uv add --dev ruff ty", root);
}

fn create_python_justfile(root: &Path, theme: &Theme) {
    let justfile_path = root.join("justfile");
    let colored_filename = "justfile".color(theme.magenta());

    if !justfile_path.exists() {
        let justfile_content = include_str!("templates/python/justfile");
        if let Err(err) = fs::write(&justfile_path, justfile_content) {
            eprintln!("{} {} {}", "Failed to create".color(theme.red()), colored_filename, err);
        } else {
            println!(
                "{} {} {}",
                "  ✓ Created".color(theme.green()),
                colored_filename,
                format!("({})", justfile_path.display()).dimmed()
            );
        }
    } else {
        println!(
            "{} {} {} {}",
            "  ℹ".dimmed(),
            colored_filename,
            "already exists, skipping...".dimmed(),
            format!("({})", justfile_path.display()).dimmed()
        );
    }
}
