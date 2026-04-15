use crate::api::ApiClient;
use crate::config::Config;
use crate::utils;
use anyhow::Result;

pub fn run() -> Result<()> {
    // Always prompt for token
    println!("visit: http://app.asana.com/-/account_api");
    println!("  Settings > Apps > Manage Developer Apps > Personal Access Tokens");
    println!("  + Create New Personal Access Token\n");

    let token = utils::prompt("paste your Personal Access Token: ")?;

    if token.trim().is_empty() {
        anyhow::bail!("Personal Access Token cannot be empty");
    }

    // Create a temporary config to fetch workspaces
    let temp_config = Config {
        personal_access_token: token.clone(),
        workspace: String::new(),
    };

    let client = ApiClient::new(&temp_config)?;
    let me = client.get_me()?;

    let workspace = if me.workspaces.len() > 1 {
        println!("\n{} workspaces found.", me.workspaces.len());
        for (i, ws) in me.workspaces.iter().enumerate() {
            println!("[{}] {} {}", i, ws.gid, ws.name);
        }

        let index = utils::prompt_number("\nChoose one out of them: ", me.workspaces.len() - 1)?;
        me.workspaces[index].gid.clone()
    } else if me.workspaces.is_empty() {
        anyhow::bail!("No workspaces found for this account");
    } else {
        me.workspaces[0].gid.clone()
    };

    let config = Config {
        personal_access_token: token,
        workspace,
    };

    config.save()?;
    println!("\nConfiguration saved successfully!");

    Ok(())
}
