pub mod assign;
pub mod browse;
pub mod comment;
pub mod config;
pub mod done;
pub mod download;
pub mod due;
pub mod install_skill;
pub mod projects;
pub mod task;
pub mod tasks;
pub mod workspaces;

use crate::utils;
use anyhow::Result;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

// Cache-related helper functions
pub fn read_cache_with_context(
    project: Option<&str>,
    assignee: Option<&str>,
) -> Result<Vec<(usize, String, String, String)>> {
    let cache_key = utils::get_cache_key(project, assignee);
    let cache_path = utils::cache_file_with_key(&cache_key)?;

    if !cache_path.exists() {
        let context_msg = if let Some(proj) = project {
            format!("project {}", proj)
        } else if let Some(asn) = assignee {
            format!("assignee {}", asn)
        } else {
            "your tasks".to_string()
        };

        anyhow::bail!(
            "No cached tasks found for {}.\nPlease run: rustasana tasks{}",
            context_msg,
            if project.is_some() {
                format!(" --project {}", project.unwrap())
            } else if assignee.is_some() && assignee.unwrap() != "me" {
                format!(" --assignee {}", assignee.unwrap())
            } else {
                String::new()
            }
        );
    }

    read_cache_from_path(&cache_path)
}

fn read_cache_from_path(cache_path: &PathBuf) -> Result<Vec<(usize, String, String, String)>> {
    let file = fs::File::open(cache_path)?;
    let reader = BufReader::new(file);

    let mut entries = Vec::new();
    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() >= 4 {
            let index = parts[0].parse::<usize>().unwrap_or(0);
            let gid = parts[1].to_string();
            let due_on = parts[2].to_string();
            let name = parts[3..].join(":");
            entries.push((index, gid, due_on, name));
        }
    }

    Ok(entries)
}

pub fn find_task_id_with_context(
    index: Option<usize>,
    project: Option<&str>,
    assignee: Option<&str>,
) -> Result<String> {
    let index = index.unwrap_or(0);

    let entries = read_cache_with_context(project, assignee)?;
    entries
        .iter()
        .find(|(i, _, _, _)| *i == index)
        .map(|(_, gid, _, _)| gid.clone())
        .ok_or_else(|| anyhow::anyhow!("Task not found at index {}", index))
}
