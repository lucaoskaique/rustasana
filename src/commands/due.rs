use crate::context::CommandContext;
use crate::date_utils;
use anyhow::Result;

pub fn run(
    index: usize,
    date: &str,
    project: Option<String>,
    assignee: Option<String>,
) -> Result<()> {
    let ctx = CommandContext::new()?;
    let task_id = ctx.find_task_id(index, project.as_deref(), assignee.as_deref())?;
    let parsed_date = date_utils::parse_date(date)?;

    ctx.client.set_due_date(&task_id, &parsed_date)?;
    println!("Due date set to: {}", parsed_date);

    Ok(())
}
