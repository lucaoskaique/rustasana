use crate::commands::find_task_id_with_context;
use crate::config::Config;
use crate::utils;
use anyhow::Result;

pub fn run(index: Option<usize>, project: Option<String>, assignee: Option<String>) -> Result<()> {
    let config = Config::load()?;
    let task_id = find_task_id_with_context(index, project.as_deref(), assignee.as_deref())?;

    let url = format!("https://app.asana.com/0/{}/{}", config.workspace, task_id);

    utils::open_url(&url)?;
    println!("Opening task in browser: {}", url);

    Ok(())
}
