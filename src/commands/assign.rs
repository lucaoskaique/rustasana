use crate::context::CommandContext;
use anyhow::Result;

pub fn run(
    index: usize,
    assignee: String,
    project: Option<String>,
    assignee_context: Option<String>,
) -> Result<()> {
    let ctx = CommandContext::new()?;
    let task_id = ctx.find_task_id(index, project.as_deref(), assignee_context.as_deref())?;

    let task = ctx.client.assign_task(&task_id, &assignee)?;

    if let Some(assigned_user) = task.assignee {
        println!("Task assigned to: {}", assigned_user.name);
    } else {
        println!("Task unassigned (set to no assignee)");
    }

    Ok(())
}
