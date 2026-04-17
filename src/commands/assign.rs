use crate::api::ApiClient;
use crate::commands::find_task_id_with_context;
use crate::config::Config;
use anyhow::Result;

pub fn run(
    index: usize,
    assignee: String,
    project: Option<String>,
    assignee_context: Option<String>,
) -> Result<()> {
    let config = Config::load()?;
    let client = ApiClient::new(&config)?;

    let task_id =
        find_task_id_with_context(Some(index), project.as_deref(), assignee_context.as_deref())?;

    let task = client.assign_task(&task_id, &assignee)?;

    if let Some(assigned_user) = task.assignee {
        println!("Task assigned to: {}", assigned_user.name);
    } else {
        println!("Task unassigned (set to no assignee)");
    }

    Ok(())
}
