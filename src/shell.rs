use std::io::{Read, Write};
use std::path::Path;

use portable_pty::{CommandBuilder, PtySize, native_pty_system};

pub fn shell(cmd: &str, current_dir: &Path) {
    let pty_system = native_pty_system();
    let pair = match pty_system.openpty(PtySize { rows: 24, cols: 80, pixel_width: 0, pixel_height: 0 }) {
        Ok(pair) => pair,
        Err(e) => {
            eprintln!("Failed to open PTY: {}", e);
            return;
        },
    };

    let mut cmd_builder = CommandBuilder::new("bash");
    cmd_builder.arg("-c");
    cmd_builder.arg(cmd);
    cmd_builder.cwd(current_dir);

    let mut child = match pair.slave.spawn_command(cmd_builder) {
        Ok(child) => child,
        Err(e) => {
            eprintln!("Failed to spawn command '{}': {}", cmd, e);
            return;
        },
    };

    drop(pair.slave);

    let mut reader = match pair.master.try_clone_reader() {
        Ok(reader) => reader,
        Err(e) => {
            eprintln!("Failed to clone PTY reader: {}", e);
            return;
        },
    };

    let mut buf = [0u8; 1024];
    while let Ok(n) = reader.read(&mut buf) {
        if n == 0 {
            break;
        }
        let _ = std::io::stdout().write_all(&buf[..n]);
        let _ = std::io::stdout().flush();
    }

    if let Err(e) = child.wait() {
        eprintln!("Command failed with error: {}", e);
    }
}
