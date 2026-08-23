use crate::config::Config;
use std::io::Write;
use std::process::{Command, Stdio};

pub fn copy_to_clipboard(text: &str, config: &Config) -> Result<(), String> {
    let cmd_str = get_clipboard_command(config)?;
    run_clipboard(&cmd_str, text)
}

fn get_clipboard_command(config: &Config) -> Result<String, String> {
    if let Some(cmd) = &config.clipboard_cmd {
        return Ok(cmd.clone());
    }

    #[cfg(target_os = "windows")]
    {
        return Ok("clip".to_string());
    }

    #[cfg(target_os = "macos")]
    {
        return Ok("pbcopy".to_string());
    }

    #[cfg(all(unix, not(target_os = "macos")))]
    {
        return find_linux_cmd();
    }

    #[allow(unreachable_code)]
    {
        Err(
            "Unsupported OS: no default clipboard command. Set clipboard_cmd in config."
                .to_string(),
        )
    }
}

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

    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(text.as_bytes())
        .map_err(|e| format!("write: {e}"))?;
    let status = child.wait().map_err(|e| format!("wait: {e}"))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("{cmd_str} exited with {status}"))
    }
}

#[cfg(all(unix, not(target_os = "macos")))]
fn find_linux_cmd() -> Result<String, String> {
    for cmd in ["xclip -selection clipboard", "wl-copy"] {
        let main = cmd.split_whitespace().next().unwrap();
        if Command::new("sh")
            .arg("-c")
            .arg(format!("command -v {main}"))
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .map(|s| s.success())
            .unwrap_or(false)
        {
            return Ok(cmd.to_string());
        }
    }
    Err("No clipboard tool found (xclip/wl-copy). Set clipboard_cmd in config.".to_string())
}
