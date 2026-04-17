use crate::api::ApiClient;
use crate::commands::find_task_id;
use crate::config::Config;
use anyhow::Result;

pub fn run(index: usize, assignee: String) -> Result<()> {
    let config = Config::load()?;
    let client = ApiClient::new(&config)?;

    let task_id = find_task_id(Some(index))?;

    let task = client.assign_task(&task_id, &assignee)?;

    if let Some(assigned_user) = task.assignee {
        println!("Task assigned to: {}", assigned_user.name);
    } else {
        println!("Task unassigned (set to no assignee)");
    }

    Ok(())
}
