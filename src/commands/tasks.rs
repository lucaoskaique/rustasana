use crate::api::ApiClient;
use crate::commands::{read_cache, write_cache};
use crate::config::Config;
use crate::utils;
use anyhow::Result;

const CACHE_DURATION_SECS: u64 = 300; // 5 minutes

pub fn run(no_cache: bool, refresh: bool) -> Result<()> {
    if no_cache {
        fetch_and_display(false)?;
    } else if utils::is_cache_older(CACHE_DURATION_SECS)? || refresh {
        fetch_and_display(true)?;
    } else {
        // Use cache
        let entries = read_cache()?;
        for (index, _, due_on, name) in entries {
            println!("{:2} [ {:10} ] {}", index, due_on, name);
        }
    }

    Ok(())
}

fn fetch_and_display(save_cache: bool) -> Result<()> {
    let config = Config::load()?;
    let client = ApiClient::new(&config)?;

    let tasks = client.get_tasks(&config.workspace, false)?;

    if save_cache {
        write_cache(&tasks)?;
    }

    for (i, task) in tasks.iter().enumerate() {
        let due_on = task.due_on.as_deref().unwrap_or("");
        println!("{:2} [ {:10} ] {}", i, due_on, task.name);
    }

    Ok(())
}
