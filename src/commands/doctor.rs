use std::path::PathBuf;
use std::process::Command;

use clap::Parser;
use colored::*;

use crate::shell::shell_with_args;
use crate::theme;

#[derive(Parser, Debug)]
pub struct DoctorArgs {
    /// Automatically install missing dependencies
    #[arg(long, short)]
    fix: bool,
}

/// Get the version string of a command (e.g., "cargo 1.80.0").
fn get_version(cmd: &str) -> Option<String> {
    Command::new(cmd)
        .arg("--version")
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map(|s| s.trim().to_string())
}

/// Represents a dependency to check and optionally install.
struct Dependency {
    name: &'static str,
    command: &'static str,
    description: &'static str,
    required: bool,
    install_cmd: Option<&'static str>,
}

impl Dependency {
    fn install(&self, theme: &theme::Theme) -> bool {
        let Some(cmd) = self.install_cmd else {
            println!(
                "    {} No automatic install command available. Please install {} manually.",
                "–".color(theme.yellow()),
                self.name.color(theme.magenta())
            );
            return false;
        };

        let name_colored = self.name.color(theme.magenta());
        println!("    {} Installing {} via: {} ...", "→".color(theme.cyan()), name_colored, cmd.dimmed());

        let workdir = std::env::current_dir().unwrap_or(PathBuf::from("."));

        // For shell commands with pipes or complex syntax, we use sh -c
        if cmd.contains(" | ") || cmd.contains(" && ") {
            shell_with_args("sh", &["-c", cmd], &workdir);
        } else {
            let mut parts = cmd.split_whitespace();
            let program = parts.next().unwrap_or("");
            let args: Vec<&str> = parts.collect();
            shell_with_args(program, &args, &workdir);
        }

        // Verify installation
        if get_version(self.command).is_some() {
            println!("    {} {} installed successfully!", "✓".color(theme.green()), name_colored);
            true
        } else {
            println!("    {} Failed to verify {} installation.", "✗".color(theme.red()), name_colored);
            false
        }
    }
}

pub fn execute(args: DoctorArgs) {
    let theme = theme::detect();
    let fix = args.fix;

    println!("{}", "Checking environment dependencies...".color(theme.cyan()).bold());
    if fix {
        println!("{}", "(Fix mode enabled)".color(theme.yellow()));
    }
    println!();

    let mut dependencies = [
        Dependency {
            name: "cargo",
            command: "cargo",
            description: "Rust package manager",
            required: true,
            install_cmd: Some("curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"),
        },
        Dependency {
            name: "rustc",
            command: "rustc",
            description: "Rust compiler",
            required: true,
            install_cmd: Some("curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"),
        },
        Dependency {
            name: "just",
            command: "just",
            description: "Command runner",
            required: false,
            install_cmd: Some("cargo install just"),
        },
        Dependency {
            name: "uv",
            command: "uv",
            description: "Python package manager",
            required: false,
            install_cmd: Some("curl -LsSf https://astral.sh/uv/install.sh | sh"),
        },
        Dependency {
            name: "ruff",
            command: "ruff",
            description: "Python linter/formatter",
            required: false,
            install_cmd: Some("uv tool install ruff"),
        },
    ];

    let mut all_ok = true;

    for dep in &mut dependencies {
        let name_colored = dep.name.color(theme.magenta());
        let desc = format!("({})", dep.description).dimmed();
        let required_tag = if dep.required {
            "required".color(theme.yellow())
        } else {
            "optional".dimmed()
        };

        if let Some(version) = get_version(dep.command) {
            let version_colored = version.color(theme.green());
            println!(
                "  {} {} {} {}",
                "✓".color(theme.green()),
                name_colored,
                version_colored,
                desc
            );
        } else if fix && dep.install_cmd.is_some() {
            let installed = dep.install(&theme);
            if !installed {
                all_ok = false;
                println!(
                    "  {} {} {} {} {}",
                    "✗".color(theme.red()),
                    name_colored,
                    "installation failed".color(theme.red()),
                    desc,
                    required_tag
                );
            }
        } else {
            if dep.required {
                all_ok = false;
            }
            println!(
                "  {} {} {} {} {}",
                if dep.required { "✗" } else { "–" }.color(if dep.required { theme.red() } else { theme.yellow() }),
                name_colored,
                if dep.required { "not found" } else { "not installed" }.color(if dep.required { theme.red() } else { theme.yellow() }),
                desc,
                required_tag
            );
        }
    }

    println!();

    if all_ok {
        println!("{}", "All required dependencies are satisfied!".color(theme.green()).bold());
    } else {
        println!(
            "{}",
            "Some required dependencies are missing. Please install them to use all features.".color(theme.red()).bold()
        );
        std::process::exit(1);
    }
}
