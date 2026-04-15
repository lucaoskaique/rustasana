use crate::api::ApiClient;
use crate::config::Config;
use anyhow::Result;

pub fn run() -> Result<()> {
    let config = Config::load()?;
    let client = ApiClient::new(&config)?;

    let me = client.get_me()?;

    println!("Workspaces:");
    for ws in &me.workspaces {
        let marker = if ws.gid == config.workspace { "*" } else { " " };
        println!("{} {} {}", marker, ws.gid, ws.name);
    }

    Ok(())
}
