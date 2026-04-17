use crate::models::Task;
use anyhow::{Context, Result};
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

/// Represents a cached task entry
#[derive(Debug, Clone)]
pub struct CacheEntry {
    pub index: usize,
    pub gid: String,
    pub due_on: String,
    pub name: String,
    pub assignee_name: String,
}

/// Manages task cache for Asana
pub struct TaskCache {
    key: String,
}

impl TaskCache {
    /// Create a new TaskCache with a specific key based on project or assignee
    pub fn new(project: Option<&str>, assignee: Option<&str>) -> Self {
        let key = Self::generate_key(project, assignee);
        Self { key }
    }

    /// Generate cache key from project or assignee context
    fn generate_key(project: Option<&str>, assignee: Option<&str>) -> String {
        if let Some(project_gid) = project {
            format!("project.{}", project_gid)
        } else if let Some(assignee_gid) = assignee {
            if assignee_gid == "me" {
                "me".to_string()
            } else {
                assignee_gid.to_string()
            }
        } else {
            "me".to_string()
        }
    }

    /// Get the cache key
    pub fn key(&self) -> &str {
        &self.key
    }

    /// Get the cache file path
    pub fn path(&self) -> Result<PathBuf> {
        let home = dirs::home_dir().context("Unable to find home directory")?;
        if self.key == "me" {
            Ok(home.join(".asana.cache"))
        } else {
            Ok(home.join(format!(".asana.cache.{}", self.key)))
        }
    }

    /// Check if cache exists
    pub fn exists(&self) -> Result<bool> {
        Ok(self.path()?.exists())
    }

    /// Check if cache is older than the specified duration
    pub fn is_older_than(&self, duration: Duration) -> Result<bool> {
        let cache_path = self.path()?;

        if !cache_path.exists() {
            return Ok(true);
        }

        let metadata = fs::metadata(&cache_path).context("Failed to read cache metadata")?;
        let modified = metadata
            .modified()
            .context("Failed to get cache modification time")?;

        let elapsed = SystemTime::now()
            .duration_since(modified)
            .context("Failed to calculate time elapsed")?;

        Ok(elapsed > duration)
    }

    /// Read cache entries
    pub fn read(&self) -> Result<Vec<CacheEntry>> {
        let cache_path = self.path()?;

        if !cache_path.exists() {
            anyhow::bail!("Cache file not found");
        }

        let file = fs::File::open(cache_path)?;
        let reader = BufReader::new(file);

        let mut entries = Vec::new();
        for line in reader.lines() {
            let line = line?;
            if line.is_empty() {
                continue;
            }

            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() >= 5 {
                let index = parts[0].parse::<usize>().unwrap_or(0);
                let gid = parts[1].to_string();
                let due_on = parts[2].to_string();
                let assignee_name = parts[3].to_string();
                let name = parts[4..].join(":");
                entries.push(CacheEntry {
                    index,
                    gid,
                    due_on,
                    name,
                    assignee_name,
                });
            }
        }

        Ok(entries)
    }

    /// Write tasks to cache
    pub fn write(&self, tasks: &[Task]) -> Result<()> {
        let cache_path = self.path()?;
        let mut content = String::new();

        for (i, task) in tasks.iter().enumerate() {
            let due_on = task.due_on.as_deref().unwrap_or("");
            let assignee_name = task
                .assignee
                .as_ref()
                .map(|a| a.name.as_str())
                .unwrap_or("");
            content.push_str(&format!(
                "{}:{}:{}:{}:{}\n",
                i, task.gid, due_on, assignee_name, task.name
            ));
        }

        fs::write(cache_path, content)?;
        Ok(())
    }

    /// Find task GID by index
    pub fn find_task(&self, index: usize) -> Result<String> {
        let entries = self.read()?;
        entries
            .iter()
            .find(|entry| entry.index == index)
            .map(|entry| entry.gid.clone())
            .ok_or_else(|| anyhow::anyhow!("Task not found at index {}", index))
    }

    /// Read cache with helpful error message if not found
    pub fn read_with_context(
        &self,
        project: Option<&str>,
        assignee: Option<&str>,
    ) -> Result<Vec<CacheEntry>> {
        if !self.exists()? {
            let context_msg = if let Some(proj) = project {
                format!("project {}", proj)
            } else if let Some(asn) = assignee {
                format!("assignee {}", asn)
            } else {
                "your tasks".to_string()
            };

            let hint = if project.is_some() {
                format!(" --project {}", project.unwrap())
            } else if assignee.is_some() && assignee.unwrap() != "me" {
                format!(" --assignee {}", assignee.unwrap())
            } else {
                String::new()
            };

            anyhow::bail!(
                "No cached tasks found for {}.\nPlease run: rustasana tasks{}",
                context_msg,
                hint
            );
        }

        self.read()
    }

    /// Delete the cache file
    pub fn clear(&self) -> Result<()> {
        let cache_path = self.path()?;
        if cache_path.exists() {
            fs::remove_file(cache_path).context("Failed to delete cache file")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_key() {
        assert_eq!(TaskCache::generate_key(None, None), "me");
        assert_eq!(TaskCache::generate_key(None, Some("me")), "me");
        assert_eq!(TaskCache::generate_key(None, Some("123456")), "123456");
        assert_eq!(
            TaskCache::generate_key(Some("proj123"), None),
            "project.proj123"
        );
        assert_eq!(
            TaskCache::generate_key(Some("proj123"), Some("ignored")),
            "project.proj123"
        );
    }
}
