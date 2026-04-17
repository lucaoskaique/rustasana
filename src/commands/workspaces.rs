use crate::context::CommandContext;
use anyhow::Result;

pub fn run() -> Result<()> {
    let ctx = CommandContext::new()?;
    let me = ctx.client.get_me()?;

    println!("Workspaces:");
    for ws in &me.workspaces {
        let marker = if ws.gid == ctx.config.workspace {
            "*"
        } else {
            " "
        };
        println!("{} {} {}", marker, ws.gid, ws.name);
    }

    Ok(())
}
