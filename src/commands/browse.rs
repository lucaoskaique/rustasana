use crate::context::CommandContext;
use crate::external;
use anyhow::Result;

pub fn run(index: Option<usize>, project: Option<String>, assignee: Option<String>) -> Result<()> {
    let ctx = CommandContext::new()?;
    let task_id = ctx.find_task_id(index.unwrap_or(0), project.as_deref(), assignee.as_deref())?;

    let url = format!(
        "https://app.asana.com/0/{}/{}",
        ctx.config.workspace, task_id
    );

    external::open_url(&url)?;
    println!("Opening task in browser: {}", url);

    Ok(())
}
