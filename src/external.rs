use anyhow::{Context, Result};
use std::env;
use std::fs;
use std::io::Write;
use std::process::Command;

/// Get the user's preferred text editor from environment or default
pub fn get_editor() -> String {
    env::var("EDITOR").unwrap_or_else(|_| {
        if cfg!(target_os = "windows") {
            "notepad".to_string()
        } else {
            "vi".to_string()
        }
    })
}

/// Open a text editor with initial content and return the edited content
pub fn open_editor(initial_content: &str) -> Result<String> {
    let mut temp_file = tempfile::Builder::new()
        .prefix("asana-comment-")
        .suffix(".txt")
        .tempfile()
        .context("Failed to create temporary file")?;

    temp_file
        .write_all(initial_content.as_bytes())
        .context("Failed to write to temporary file")?;

    let path = temp_file.path().to_path_buf();
    let editor = get_editor();

    let status = Command::new(&editor)
        .arg(&path)
        .status()
        .with_context(|| format!("Failed to open editor: {}", editor))?;

    if !status.success() {
        anyhow::bail!("Editor exited with non-zero status");
    }

    let content = fs::read_to_string(&path).context("Failed to read edited content")?;

    Ok(content)
}

/// Get the browser launcher command for the current platform
pub fn get_browser_launcher() -> Result<String> {
    if let Ok(browser) = env::var("BROWSER") {
        return Ok(browser);
    }

    let launcher = if cfg!(target_os = "macos") {
        "open".to_string()
    } else if cfg!(target_os = "windows") {
        "cmd /c start".to_string()
    } else {
        // Linux/Unix
        let candidates = ["xdg-open", "firefox", "chromium", "google-chrome", "opera"];

        candidates
            .iter()
            .find(|&&cmd| Command::new(cmd).arg("--version").output().is_ok())
            .map(|&s| s.to_string())
            .unwrap_or_else(|| "xdg-open".to_string())
    };

    Ok(launcher)
}

/// Open a URL in the user's default browser
pub fn open_url(url: &str) -> Result<()> {
    let launcher = get_browser_launcher()?;

    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/c", "start", url])
            .spawn()
            .context("Failed to open browser")?;
    } else {
        Command::new(launcher)
            .arg(url)
            .spawn()
            .context("Failed to open browser")?;
    }

    Ok(())
}
