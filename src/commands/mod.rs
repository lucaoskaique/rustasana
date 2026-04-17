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

use crate::cache::TaskCache;
use anyhow::Result;

/// Find task ID from cache using optional context (project/assignee)
pub fn find_task_id_with_context(
    index: Option<usize>,
    project: Option<&str>,
    assignee: Option<&str>,
) -> Result<String> {
    let index = index.unwrap_or(0);
    let cache = TaskCache::new(project, assignee);
    cache.find_task(index)
}
