use crate::config::Config;
use std::io::Write;
use std::process::{Command, Stdio};

pub fn copy_to_clipboard(text: &str, config: &Config) -> Result<(), String> {
    // Если пользователь явно переопределил команду — уважаем её.
    if let Some(cmd) = &config.clipboard_cmd {
        return run_clipboard(cmd, text);
    }

    let mut cb = arboard::Clipboard::new().map_err(|e| format!("clipboard init: {e}"))?;
    cb.set_text(text.to_string())
        .map_err(|e| format!("clipboard set_text: {e}"))
}

// Оставлено только для случая clipboard_cmd в config.
fn run_clipboard(cmd_str: &str, text: &str) -> Result<(), String> {
    let (shell, flag) = if cfg!(target_os = "windows") {
        ("cmd", "/C")
    } else {
        ("sh", "-c")
    };

    let mut child = Command::new(shell)
        .arg(flag)
        .arg(cmd_str)
        .stdin(Stdio::piped())
        .spawn()
        .map_err(|e| format!("spawn {shell} {flag} {cmd_str}: {e}"))?;

    {
        let mut stdin = child
            .stdin
            .take()
            .ok_or_else(|| "clipboard: no stdin handle".to_string())?;
        stdin
            .write_all(text.as_bytes())
            .map_err(|e| format!("write: {e}"))?;
        // stdin закрывается здесь -> дочерний процесс получит EOF.
    }

    let status = child.wait().map_err(|e| format!("wait: {e}"))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("{cmd_str} exited with {status}"))
    }
}
