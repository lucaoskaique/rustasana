use crate::commands::find_task_id;
use crate::config::Config;
use crate::utils;
use anyhow::Result;

pub fn run(index: Option<usize>) -> Result<()> {
    let config = Config::load()?;
    let task_id = find_task_id(index)?;

    let url = format!("https://app.asana.com/0/{}/{}", config.workspace, task_id);

    utils::open_url(&url)?;
    println!("Opening task in browser: {}", url);

    Ok(())
}
