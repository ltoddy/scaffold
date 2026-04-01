use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use clap::Parser;
use colored::*;

use crate::cli::Language;
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

    // 确保目录存在
    if !args.path.exists()
        && let Err(e) = fs::create_dir_all(&args.path)
    {
        eprintln!("{} {}", "Failed to create directory:".color(theme.red()), e);
        return;
    }

    match args.language {
        Language::Rust => create_rust_files(&args.path),
        Language::Python => create_python_files(&args.path),
    }
}

fn create_rust_files(root: &Path) {
    let theme = theme::detect();
    println!(
        "{} {}",
        "Creating Rust project configuration files...".color(theme.blue()),
        format!("in {}", root.display()).dimmed()
    );

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

fn create_python_files(root: &Path) {
    let theme = theme::detect();

    println!(
        "{} {}",
        "Creating Python project configuration files...".color(theme.blue()),
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
        let Some(_) = run_uv_init(root, &theme) else {
            return;
        };
    }

    // 执行 uv add --dev ruff ty
    let Some(_) = run_uv_add_dev(root, &theme) else {
        return;
    };

    // 创建 justfile
    create_python_justfile(root, &theme);

    println!(
        "{} {}",
        "Python project initialized successfully!".color(theme.green()).bold(),
        format!("in {}", root.display()).dimmed()
    );
}

fn run_uv_init(root: &Path, theme: &Theme) -> Option<()> {
    println!("{} {}", "Running uv init...".color(theme.cyan()), format!("in {}", root.display()).dimmed());

    let output = Command::new("uv").arg("init").arg("--directory").arg(root).current_dir(root).output().ok()?;

    if output.status.success() {
        println!("{}", "  ✓ uv init completed successfully".color(theme.green()));
        Some(())
    } else {
        eprintln!("{} {}", "Failed to run uv init:".color(theme.red()), String::from_utf8_lossy(&output.stderr));
        eprintln!("{}", "Make sure 'uv' is installed and available in PATH".color(theme.yellow()));
        None
    }
}

fn run_uv_add_dev(root: &Path, theme: &Theme) -> Option<()> {
    println!("{} {}", "Running uv add --dev ruff ty...".color(theme.cyan()), format!("in {}", root.display()).dimmed());

    let output = Command::new("uv").args(["add", "--dev", "ruff", "ty"]).current_dir(root).output().ok()?;

    if output.status.success() {
        println!("{}", "  ✓ Added dev dependencies: ruff, ty".color(theme.green()));
        Some(())
    } else {
        eprintln!(
            "{} {}",
            "Failed to add dev dependencies:".color(theme.red()),
            String::from_utf8_lossy(&output.stderr)
        );
        None
    }
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
