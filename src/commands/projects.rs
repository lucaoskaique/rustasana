use crate::api::ApiClient;
use crate::config::Config;
use anyhow::Result;

pub fn run() -> Result<()> {
    let config = Config::load()?;
    let client = ApiClient::new(&config)?;

    let projects = client.get_projects(&config.workspace)?;

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
