use crate::api::ApiClient;
use crate::commands::find_task_id_with_context;
use crate::config::Config;
use crate::utils;
use anyhow::Result;

pub fn run(
    index: usize,
    date: &str,
    project: Option<String>,
    assignee: Option<String>,
) -> Result<()> {
    let config = Config::load()?;
    let client = ApiClient::new(&config)?;

    let task_id = find_task_id_with_context(Some(index), project.as_deref(), assignee.as_deref())?;
    let parsed_date = utils::parse_date(date)?;

    client.set_due_date(&task_id, &parsed_date)?;
    println!("Due date set to: {}", parsed_date);

    Ok(())
}
