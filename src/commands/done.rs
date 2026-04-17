use crate::context::CommandContext;
use anyhow::Result;

pub fn run(index: usize, project: Option<String>, assignee: Option<String>) -> Result<()> {
    let ctx = CommandContext::new()?;
    let task_id = ctx.find_task_id(index, project.as_deref(), assignee.as_deref())?;

    ctx.client.complete_task(&task_id)?;
    println!("Task marked as completed!");

    Ok(())
}
