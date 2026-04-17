use crate::context::CommandContext;
use crate::external;
use anyhow::Result;

pub fn run(index: usize, project: Option<String>, assignee: Option<String>) -> Result<()> {
    let ctx = CommandContext::new()?;
    let task_id = ctx.find_task_id(index, project.as_deref(), assignee.as_deref())?;

    // Open editor for comment
    let initial_content =
        "\n# Write your comment above this line\n# Lines starting with # will be ignored";
    let content = external::open_editor(initial_content)?;

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

    ctx.client.add_comment(&task_id, &comment)?;
    println!("Comment added successfully!");

    Ok(())
}
