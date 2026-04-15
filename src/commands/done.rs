use crate::api::ApiClient;
use crate::commands::find_task_id;
use crate::config::Config;
use anyhow::Result;

pub fn run(index: usize) -> Result<()> {
    let config = Config::load()?;
    let client = ApiClient::new(&config)?;

    let task_id = find_task_id(Some(index))?;

    client.complete_task(&task_id)?;
    println!("Task marked as completed!");

    Ok(())
}
