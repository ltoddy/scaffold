use std::path::Path;
use std::process::{Command, Stdio};

pub fn shell(cmd: &str, current_dir: &Path) {
    let mut parts = cmd.split_whitespace();
    let program = match parts.next() {
        Some(p) => p,
        None => return,
    };

    shell_with_args(program, &parts.collect::<Vec<_>>(), current_dir);
}

pub fn shell_with_args(program: &str, args: &[&str], current_dir: &Path) {
    let mut child = match Command::new(program)
        .args(args)
        .current_dir(current_dir)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
    {
        Ok(child) => child,
        Err(e) => {
            eprintln!("Failed to spawn command '{} {}': {}", program, args.join(" "), e);
            return;
        },
    };

    if let Err(e) = child.wait() {
        eprintln!("Command failed with error: {}", e);
    }
}
