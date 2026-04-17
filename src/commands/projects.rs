use crate::context::CommandContext;
use anyhow::Result;

pub fn run() -> Result<()> {
    let ctx = CommandContext::new()?;
    let projects = ctx.client.get_projects(&ctx.config.workspace)?;

    if projects.is_empty() {
        println!("No projects found in this workspace.");
        return Ok(());
    }

    println!("Projects:");
    for project in projects {
        println!("  {} {}", project.gid, project.name);
    }

    Ok(())
}
