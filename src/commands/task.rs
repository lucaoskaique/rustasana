use crate::api::ApiClient;
use crate::commands::find_task_id_with_context;
use crate::config::Config;
use crate::models::Story;
use anyhow::Result;

pub fn run(
    index: Option<usize>,
    verbose: bool,
    json: bool,
    project: Option<String>,
    assignee: Option<String>,
) -> Result<()> {
    let config = Config::load()?;
    let client = ApiClient::new(&config)?;

    let task_id = find_task_id_with_context(index, project.as_deref(), assignee.as_deref())?;
    let task = client.get_task(&task_id)?;

    if json {
        println!("{}", serde_json::to_string_pretty(&task)?);
        return Ok(());
    }

    // Display task details
    let due_on = task.due_on.as_deref().unwrap_or("");
    println!("\n[ {} ] {}", due_on, task.name);
    println!("{}", "-".repeat(40));

    if let Some(notes) = &task.notes {
        if !notes.is_empty() {
            println!("{}", notes);
            println!("{}", "-".repeat(40));
        }
    }

    // Show attachments
    if let Ok(attachments) = client.get_attachments(&task_id) {
        if !attachments.is_empty() {
            println!("\nAttachments:");
            for (i, attachment) in attachments.iter().enumerate() {
                println!("  [{}] {} ({})", i, attachment.name, attachment.host);
            }
            println!("{}", "-".repeat(40));
        }
    }

    if verbose {
        let stories = client.get_stories(&task_id)?;

        println!();
        for story in stories {
            print_story(&story);
        }
    }

    Ok(())
}

fn print_story(story: &Story) {
    if story.story_type == "comment" {
        if let Some(text) = &story.text {
            println!(
                "> {}\nby {} ({})",
                text, story.created_by.name, story.created_at
            );
            println!("{}", "-".repeat(40));
        }
    } else if let Some(text) = &story.text {
        println!("* {} ({})", text, story.created_at);
        println!("{}", "-".repeat(40));
    }
}
