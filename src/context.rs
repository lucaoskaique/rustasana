use crate::api::ApiClient;
use crate::cache::TaskCache;
use crate::config::Config;
use anyhow::Result;

/// Common context for commands that need config and API client
pub struct CommandContext {
    pub config: Config,
    pub client: ApiClient,
}

impl CommandContext {
    /// Create a new CommandContext by loading config and creating API client
    pub fn new() -> Result<Self> {
        let config = Config::load()?;
        let client = ApiClient::new(&config)?;
        Ok(Self { config, client })
    }

    /// Find a task ID from the cache using index and optional filters
    pub fn find_task_id(
        &self,
        index: usize,
        project: Option<&str>,
        assignee: Option<&str>,
    ) -> Result<String> {
        let cache = TaskCache::new(project, assignee);
        cache.find_task(index)
    }
}
