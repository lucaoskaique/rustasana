use crate::api::ApiClient;
use crate::commands::find_task_id_with_context;
use crate::config::Config;
use anyhow::Result;

pub fn run(index: usize, project: Option<String>, assignee: Option<String>) -> Result<()> {
    let config = Config::load()?;
    let client = ApiClient::new(&config)?;

    let task_id = find_task_id_with_context(Some(index), project.as_deref(), assignee.as_deref())?;

    client.complete_task(&task_id)?;
    println!("Task marked as completed!");

    Ok(())
}
