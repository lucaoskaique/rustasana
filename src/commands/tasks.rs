use crate::api::ApiClient;
use crate::config::Config;
use crate::utils;
use anyhow::Result;

const CACHE_DURATION_SECS: u64 = 300; // 5 minutes

pub fn run(
    no_cache: bool,
    refresh: bool,
    assignee: Option<String>,
    project: Option<String>,
) -> Result<()> {
    // Determine cache key based on filter mode
    let cache_key = if let Some(ref project_id) = project {
        format!("project.{}", project_id)
    } else if let Some(ref assignee_id) = assignee {
        assignee_id.to_string()
    } else {
        "me".to_string()
    };

    if no_cache {
        fetch_and_display(false, assignee, project)?;
    } else if utils::is_cache_older_with_key(CACHE_DURATION_SECS, &cache_key)? || refresh {
        fetch_and_display(true, assignee, project)?;
    } else {
        // Use cache
        let entries = read_cache_with_key(&cache_key)?;
        for (index, _, due_on, name, assignee_name) in entries {
            let assignee_display = format_assignee(&assignee_name);
            println!(
                "{:2} [ {:10} ] {:50} {}",
                index,
                due_on,
                truncate(&name, 50),
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
) -> Result<()> {
    let config = Config::load()?;
    let client = ApiClient::new(&config)?;

    // Fetch tasks based on filter type
    let tasks = if let Some(ref project_id) = project {
        // Fetch all tasks from a specific project
        match client.get_project_tasks(project_id, false) {
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
        match client.get_tasks(&config.workspace, false, assignee_filter) {
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
        let cache_key = if let Some(ref project_id) = project {
            format!("project.{}", project_id)
        } else if let Some(ref assignee_id) = assignee {
            assignee_id.to_string()
        } else {
            "me".to_string()
        };
        write_cache_with_key(&tasks, &cache_key)?;
    }

    for (i, task) in tasks.iter().enumerate() {
        let due_on = task.due_on.as_deref().unwrap_or("");
        let assignee_name = task
            .assignee
            .as_ref()
            .map(|a| a.name.as_str())
            .unwrap_or("");
        let assignee_display = format_assignee(assignee_name);
        println!(
            "{:2} [ {:10} ] {:50} {}",
            i,
            due_on,
            truncate(&task.name, 50),
            assignee_display
        );
    }

    Ok(())
}

fn format_assignee(assignee_name: &str) -> String {
    if assignee_name.is_empty() {
        "[unassigned]".to_string()
    } else {
        format!("[@{}]", assignee_name)
    }
}

fn truncate(s: &str, max_len: usize) -> String {
    let char_count = s.chars().count();
    if char_count <= max_len {
        format!("{:width$}", s, width = max_len)
    } else {
        let truncated: String = s.chars().take(max_len - 3).collect();
        format!("{:width$}...", truncated, width = max_len)
    }
}

fn read_cache_with_key(key: &str) -> Result<Vec<(usize, String, String, String, String)>> {
    let cache_path = utils::cache_file_with_key(key)?;
    if !cache_path.exists() {
        anyhow::bail!("Cache file not found");
    }

    let file = std::fs::File::open(cache_path)?;
    let reader = std::io::BufReader::new(file);
    use std::io::BufRead;

    let mut entries = Vec::new();
    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() >= 5 {
            let index = parts[0].parse::<usize>().unwrap_or(0);
            let gid = parts[1].to_string();
            let due_on = parts[2].to_string();
            let assignee = parts[3].to_string();
            let name = parts[4..].join(":");
            entries.push((index, gid, due_on, name, assignee));
        }
    }

    Ok(entries)
}

fn write_cache_with_key(tasks: &[crate::models::Task], key: &str) -> Result<()> {
    let cache_path = utils::cache_file_with_key(key)?;
    let mut content = String::new();

    for (i, task) in tasks.iter().enumerate() {
        let due_on = task.due_on.as_deref().unwrap_or("");
        let assignee = task
            .assignee
            .as_ref()
            .map(|a| a.name.as_str())
            .unwrap_or("");
        content.push_str(&format!(
            "{}:{}:{}:{}:{}\n",
            i, task.gid, due_on, assignee, task.name
        ));
    }

    std::fs::write(cache_path, content)?;
    Ok(())
}
