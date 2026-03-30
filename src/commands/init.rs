use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

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
        Language::Python => create_python_files(&args.path),
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

fn create_python_files(root: &Path) {
    println!(
        "{} {}",
        "Creating Python project configuration files...".blue(),
        format!("in {}", root.display()).dimmed()
    );

    // 检查 pyproject.toml 是否存在，如果存在则跳过 uv init
    let pyproject_path = root.join("pyproject.toml");
    if pyproject_path.exists() {
        println!(
            "{} {}",
            "  ℹ pyproject.toml already exists, skipping uv init...".dimmed(),
            format!("({})", pyproject_path.display()).dimmed()
        );
    } else {
        // 执行 uv init
        let Some(_) = run_uv_init(root) else {
            return;
        };
    }

    // 执行 uv add --dev ruff ty
    let Some(_) = run_uv_add_dev(root) else {
        return;
    };

    // 创建 justfile
    create_python_justfile(root);

    println!(
        "{} {}",
        "Python project initialized successfully!".green().bold(),
        format!("in {}", root.display()).dimmed()
    );
}

fn run_uv_init(root: &Path) -> Option<()> {
    println!("{} {}", "Running uv init...".cyan(), format!("in {}", root.display()).dimmed());

    let output = Command::new("uv").arg("init").arg("--directory").arg(root).current_dir(root).output().ok()?;

    if output.status.success() {
        println!("{}", "  ✓ uv init completed successfully".green());
        Some(())
    } else {
        eprintln!("{} {}", "Failed to run uv init:".red(), String::from_utf8_lossy(&output.stderr));
        eprintln!("{}", "Make sure 'uv' is installed and available in PATH".yellow());
        None
    }
}

fn run_uv_add_dev(root: &Path) -> Option<()> {
    println!("{} {}", "Running uv add --dev ruff ty...".cyan(), format!("in {}", root.display()).dimmed());

    let output = Command::new("uv").args(["add", "--dev", "ruff", "ty"]).current_dir(root).output().ok()?;

    if output.status.success() {
        println!("{}", "  ✓ Added dev dependencies: ruff, ty".green());
        Some(())
    } else {
        eprintln!("{} {}", "Failed to add dev dependencies:".red(), String::from_utf8_lossy(&output.stderr));
        None
    }
}

fn create_python_justfile(root: &Path) {
    let justfile_path = root.join("justfile");
    if !justfile_path.exists() {
        let justfile_content = include_str!("templates/python/justfile");
        if let Err(err) = fs::write(&justfile_path, justfile_content) {
            eprintln!("{} {}", "Failed to create justfile:".red(), err);
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
}
