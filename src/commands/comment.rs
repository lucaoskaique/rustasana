use crate::api::ApiClient;
use crate::commands::find_task_id;
use crate::config::Config;
use crate::utils;
use anyhow::Result;

pub fn run(index: usize) -> Result<()> {
    let config = Config::load()?;
    let client = ApiClient::new(&config)?;

    let task_id = find_task_id(Some(index))?;

    // Open editor for comment
    let initial_content = "\n# Write your comment above this line\n# Lines starting with # will be ignored";
    let content = utils::open_editor(initial_content)?;

    // Filter out comment lines and empty lines
    let comment: Vec<&str> = content
        .lines()
        .filter(|line| !line.trim().starts_with('#') && !line.trim().is_empty())
        .collect();

    let comment = comment.join("\n").trim().to_string();

    if comment.is_empty() {
        println!("Comment is empty. Aborted.");
        return Ok(());
    }

    client.add_comment(&task_id, &comment)?;
    println!("Comment added successfully!");

    Ok(())
}
