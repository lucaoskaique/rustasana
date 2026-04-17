use crate::cache::TaskCache;
use crate::context::CommandContext;
use anyhow::Result;
use std::time::Duration;
use unicode_segmentation::UnicodeSegmentation;

const CACHE_DURATION_SECS: u64 = 300; // 5 minutes
const MAX_TASK_NAME_LENGTH: usize = 50;

pub fn run(
    no_cache: bool,
    refresh: bool,
    assignee: Option<String>,
    project: Option<String>,
    all: bool,
) -> Result<()> {
    let cache = TaskCache::new(project.as_deref(), assignee.as_deref());
    let cache_duration = Duration::from_secs(CACHE_DURATION_SECS);

    if no_cache {
        fetch_and_display(false, assignee, project, all)?;
    } else if cache.is_older_than(cache_duration)? || refresh {
        fetch_and_display(true, assignee, project, all)?;
    } else {
        // Use cache
        let entries = cache.read()?;
        for entry in entries {
            let assignee_display = if entry.assignee_name.is_empty() {
                "[unassigned]".to_string()
            } else {
                format!("[@{}]", entry.assignee_name)
            };
            println!(
                "{:2} [ {:10} ] {:50} {}",
                entry.index,
                entry.due_on,
                truncate(&entry.name, MAX_TASK_NAME_LENGTH),
                assignee_display
            );
        }
    }

    Ok(())
}

fn fetch_and_display(
    save_cache: bool,
    assignee: Option<String>,
    project: Option<String>,
    all: bool,
) -> Result<()> {
    let ctx = CommandContext::new()?;

    // Fetch tasks based on filter type
    let tasks = if let Some(ref project_id) = project {
        // Fetch all tasks from a specific project
        match ctx.client.get_project_tasks(project_id, all) {
            Ok(tasks) => tasks,
            Err(e) => {
                let error_msg = e.to_string();
                if error_msg.contains("404") {
                    anyhow::bail!(
                        "Project '{}' not found.\n\
                        Please provide a valid project GID.",
                        project_id
                    );
                } else {
                    return Err(e);
                }
            }
        }
    } else {
        // Determine assignee filter for API
        let assignee_filter = if let Some(ref assignee_id) = assignee {
            Some(assignee_id.as_str())
        } else {
            None // defaults to "me" in API client
        };

        // Fetch tasks with error handling
        match ctx
            .client
            .get_tasks(&ctx.config.workspace, all, assignee_filter)
        {
            Ok(tasks) => tasks,
            Err(e) => {
                let error_msg = e.to_string();
                if error_msg.contains("403") || error_msg.contains("Forbidden") {
                    anyhow::bail!(
                        "Unable to fetch tasks. You may not have permission to view the specified assignee's tasks.\n\
                        Try: rustasana tasks (without --assignee)"
                    );
                } else if error_msg.contains("404") && assignee.is_some() {
                    anyhow::bail!(
                        "Assignee '{}' not found in workspace.\n\
                        Please provide a valid user GID.",
                        assignee.unwrap()
                    );
                } else {
                    return Err(e);
                }
            }
        }
    };

    if tasks.is_empty() {
        if project.is_some() {
            println!("No tasks found in the specified project.");
        } else if assignee.is_some() {
            println!("No tasks found for the specified assignee.");
        } else {
            println!("No tasks assigned to you.");
        }
        return Ok(());
    }

    if save_cache {
        let cache = TaskCache::new(project.as_deref(), assignee.as_deref());
        cache.write(&tasks)?;
    }

    for (i, task) in tasks.iter().enumerate() {
        println!(
            "{:2} [ {:10} ] {:50} {}",
            i,
            task.due_date_str(),
            truncate(&task.name, MAX_TASK_NAME_LENGTH),
            task.format_assignee()
        );
    }

    Ok(())
}

fn truncate(s: &str, max_len: usize) -> String {
    let graphemes: Vec<&str> = s.graphemes(true).collect();
    if graphemes.len() <= max_len {
        format!("{:width$}", s, width = max_len)
    } else {
        let truncated: String = graphemes[..max_len - 3].join("");
        format!("{:width$}...", truncated, width = max_len)
    }
}
