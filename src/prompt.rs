use anyhow::{Context, Result};
use std::io::Write;

/// Prompt the user for text input
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

/// Prompt the user for a number within a range
pub fn prompt_number(message: &str, max: usize) -> Result<usize> {
    loop {
        let input = prompt(message)?;
        match input.parse::<usize>() {
            Ok(n) if n <= max => return Ok(n),
            _ => println!("Invalid input. Please enter a number between 0 and {}", max),
        }
    }
}
