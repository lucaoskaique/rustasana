use anyhow::{Context, Result};
use chrono::{Local, NaiveDate};
use std::env;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use std::time::SystemTime;

pub fn cache_file() -> Result<PathBuf> {
    let home = dirs::home_dir().context("Unable to find home directory")?;
    Ok(home.join(".asana.cache"))
}

pub fn cache_file_with_key(key: &str) -> Result<PathBuf> {
    let home = dirs::home_dir().context("Unable to find home directory")?;
    if key == "me" {
        Ok(home.join(".asana.cache"))
    } else {
        Ok(home.join(format!(".asana.cache.{}", key)))
    }
}

pub fn is_cache_older(duration_secs: u64) -> Result<bool> {
    let cache_path = cache_file()?;

    if !cache_path.exists() {
        return Ok(true);
    }

    let metadata = fs::metadata(&cache_path).context("Failed to read cache metadata")?;
    let modified = metadata
        .modified()
        .context("Failed to get cache modification time")?;

    let elapsed = SystemTime::now()
        .duration_since(modified)
        .context("Failed to calculate time elapsed")?;

    Ok(elapsed.as_secs() > duration_secs)
}

pub fn is_cache_older_with_key(duration_secs: u64, key: &str) -> Result<bool> {
    let cache_path = cache_file_with_key(key)?;

    if !cache_path.exists() {
        return Ok(true);
    }

    let metadata = fs::metadata(&cache_path).context("Failed to read cache metadata")?;
    let modified = metadata
        .modified()
        .context("Failed to get cache modification time")?;

    let elapsed = SystemTime::now()
        .duration_since(modified)
        .context("Failed to calculate time elapsed")?;

    Ok(elapsed.as_secs() > duration_secs)
}

pub fn get_editor() -> String {
    env::var("EDITOR").unwrap_or_else(|_| {
        if cfg!(target_os = "windows") {
            "notepad".to_string()
        } else {
            "vi".to_string()
        }
    })
}

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

pub fn parse_date(date_str: &str) -> Result<String> {
    match date_str.to_lowercase().as_str() {
        "today" => Ok(Local::now().format("%Y-%m-%d").to_string()),
        "tomorrow" => Ok((Local::now() + chrono::Duration::days(1))
            .format("%Y-%m-%d")
            .to_string()),
        _ => {
            // Validate the date format
            NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
                .context("Invalid date format. Use YYYY-MM-DD, 'today', or 'tomorrow'")?;
            Ok(date_str.to_string())
        }
    }
}

pub fn prompt(message: &str) -> Result<String> {
    print!("{}", message);
    std::io::stdout()
        .flush()
        .context("Failed to flush stdout")?;

    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .context("Failed to read input")?;

    Ok(input.trim().to_string())
}

pub fn prompt_number(message: &str, max: usize) -> Result<usize> {
    loop {
        let input = prompt(message)?;
        match input.parse::<usize>() {
            Ok(n) if n <= max => return Ok(n),
            _ => println!("Invalid input. Please enter a number between 0 and {}", max),
        }
    }
}
