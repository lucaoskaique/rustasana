use crate::config::Config;
use crate::models::{Attachment, DataWrapper, Me, Story, Task};
use anyhow::{Context, Result};
use reqwest::blocking::Client;
use serde_json::json;
use std::io::Write;

const API_BASE: &str = "https://app.asana.com/api/1.0";
const USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_9_3) \
                          AppleWebKit/537.36 (KHTML, like Gecko) \
                          Chrome/36.0.1985.125 Safari/537.36";
const OPT_FIELDS: &str = "name,completed,due_on,assignee,assignee.name";
const API_PAGE_SIZE: usize = 100;

/// Builder for constructing task query URLs
struct TaskQueryBuilder {
    workspace: String,
    assignee: Option<String>,
    include_completed: bool,
}

impl TaskQueryBuilder {
    fn new(workspace: impl Into<String>) -> Self {
        Self {
            workspace: workspace.into(),
            assignee: None,
            include_completed: false,
        }
    }

    fn assignee(mut self, assignee: impl Into<String>) -> Self {
        self.assignee = Some(assignee.into());
        self
    }

    fn include_completed(mut self, include: bool) -> Self {
        self.include_completed = include;
        self
    }

    fn build(self) -> String {
        let mut params = vec![
            format!("workspace={}", self.workspace),
            format!("opt_fields={}", OPT_FIELDS),
        ];

        if let Some(assignee) = self.assignee {
            params.push(format!("assignee={}", assignee));
        } else {
            params.push("assignee=me".to_string());
        }

        if !self.include_completed {
            params.push("completed_since=now".to_string());
        }

        format!("/tasks?{}", params.join("&"))
    }
}

pub struct ApiClient {
    client: Client,
    token: String,
}

impl ApiClient {
    pub fn new(config: &Config) -> Result<Self> {
        let client = Client::builder()
            .user_agent(USER_AGENT)
            .build()
            .context("Failed to create HTTP client")?;

        Ok(Self {
            client,
            token: config.personal_access_token.clone(),
        })
    }

    fn build_url(&self, path: &str) -> String {
        format!("{}{}", API_BASE, path)
    }

    fn get(&self, path: &str) -> Result<String> {
        let url = self.build_url(path);
        let response = self
            .client
            .get(&url)
            .bearer_auth(&self.token)
            .send()
            .context("Failed to send GET request")?;

        self.handle_response(response)
    }

    fn post(&self, path: &str, data: serde_json::Value) -> Result<String> {
        let url = self.build_url(path);
        let response = self
            .client
            .post(&url)
            .bearer_auth(&self.token)
            .json(&data)
            .send()
            .context("Failed to send POST request")?;

        self.handle_response(response)
    }

    fn put(&self, path: &str, data: serde_json::Value) -> Result<String> {
        let url = self.build_url(path);
        let response = self
            .client
            .put(&url)
            .bearer_auth(&self.token)
            .json(&data)
            .send()
            .context("Failed to send PUT request")?;

        self.handle_response(response)
    }

    fn handle_response(&self, response: reqwest::blocking::Response) -> Result<String> {
        let status = response.status();
        let body = response.text().context("Failed to read response body")?;

        if status.is_success() {
            Ok(body)
        } else {
            anyhow::bail!("API request failed with status {}: {}", status, body)
        }
    }

    pub fn get_me(&self) -> Result<Me> {
        let body = self.get("/users/me")?;
        let wrapper: DataWrapper<Me> =
            serde_json::from_str(&body).context("Failed to parse user data")?;
        Ok(wrapper.data)
    }

    pub fn get_projects(&self, workspace: &str) -> Result<Vec<crate::models::Base>> {
        let path = format!("/workspaces/{}/projects", workspace);
        let body = self.get(&path)?;
        let wrapper: DataWrapper<Vec<crate::models::Base>> =
            serde_json::from_str(&body).context("Failed to parse projects")?;
        Ok(wrapper.data)
    }

    pub fn get_tasks(
        &self,
        workspace: &str,
        with_completed: bool,
        assignee_filter: Option<&str>,
    ) -> Result<Vec<Task>> {
        let mut builder = TaskQueryBuilder::new(workspace).include_completed(with_completed);

        if let Some(assignee) = assignee_filter {
            builder = builder.assignee(assignee);
        }

        let path = builder.build();

        let body = self.get(&path)?;
        let wrapper: DataWrapper<Vec<Task>> =
            serde_json::from_str(&body).context("Failed to parse tasks")?;

        // Sort tasks by due date
        let mut tasks = wrapper.data;
        tasks.sort();

        Ok(tasks)
    }

    pub fn get_project_tasks(&self, project_id: &str, with_completed: bool) -> Result<Vec<Task>> {
        let mut all_tasks = Vec::new();
        let mut offset: Option<String> = None;

        loop {
            let path = if with_completed {
                // Include both completed and incomplete tasks
                if let Some(ref offset_val) = offset {
                    format!(
                        "/projects/{}/tasks?opt_fields={}&limit={}&offset={}",
                        project_id, OPT_FIELDS, API_PAGE_SIZE, offset_val
                    )
                } else {
                    format!(
                        "/projects/{}/tasks?opt_fields={}&limit={}",
                        project_id, OPT_FIELDS, API_PAGE_SIZE
                    )
                }
            } else {
                // Only incomplete tasks
                if let Some(ref offset_val) = offset {
                    format!(
                        "/projects/{}/tasks?opt_fields={}&completed_since=now&limit={}&offset={}",
                        project_id, OPT_FIELDS, API_PAGE_SIZE, offset_val
                    )
                } else {
                    format!(
                        "/projects/{}/tasks?opt_fields={}&completed_since=now&limit={}",
                        project_id, OPT_FIELDS, API_PAGE_SIZE
                    )
                }
            };

            let body = self.get(&path)?;

            // Parse response with next_page info
            let response: serde_json::Value =
                serde_json::from_str(&body).context("Failed to parse project tasks response")?;

            // Extract tasks
            if let Some(data) = response.get("data").cloned() {
                let tasks: Vec<Task> =
                    serde_json::from_value(data).context("Failed to parse tasks from data")?;
                all_tasks.extend(tasks);
            }

            // Check for next page
            if let Some(next_page) = response.get("next_page") {
                if next_page.is_null() {
                    break;
                }
                if let Some(next_offset) = next_page.get("offset").and_then(|o| o.as_str()) {
                    offset = Some(next_offset.to_string());
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        // Sort tasks by due date
        all_tasks.sort();

        Ok(all_tasks)
    }

    pub fn get_task(&self, task_id: &str) -> Result<Task> {
        let path = format!("/tasks/{}", task_id);
        let body = self.get(&path)?;
        let wrapper: DataWrapper<Task> =
            serde_json::from_str(&body).context("Failed to parse task")?;
        Ok(wrapper.data)
    }

    pub fn get_stories(&self, task_id: &str) -> Result<Vec<Story>> {
        let path = format!("/tasks/{}/stories", task_id);
        let body = self.get(&path)?;
        let wrapper: DataWrapper<Vec<Story>> =
            serde_json::from_str(&body).context("Failed to parse stories")?;
        Ok(wrapper.data)
    }

    pub fn add_comment(&self, task_id: &str, text: &str) -> Result<()> {
        let path = format!("/tasks/{}/stories", task_id);
        let data = json!({
            "data": {
                "text": text
            }
        });
        self.post(&path, data)?;
        Ok(())
    }

    pub fn update_task(&self, task_id: &str, key: &str, value: serde_json::Value) -> Result<Task> {
        let path = format!("/tasks/{}", task_id);
        let data = json!({
            "data": {
                key: value
            }
        });
        let body = self.put(&path, data)?;
        let wrapper: DataWrapper<Task> =
            serde_json::from_str(&body).context("Failed to parse updated task")?;
        Ok(wrapper.data)
    }

    pub fn complete_task(&self, task_id: &str) -> Result<Task> {
        self.update_task(task_id, "completed", json!(true))
    }

    pub fn set_due_date(&self, task_id: &str, due_on: &str) -> Result<Task> {
        self.update_task(task_id, "due_on", json!(due_on))
    }

    pub fn assign_task(&self, task_id: &str, assignee: &str) -> Result<Task> {
        // assignee can be a user GID or "me" or "null" for unassigned
        let assignee_value = if assignee == "null" || assignee == "unassigned" {
            json!(null)
        } else {
            json!(assignee)
        };
        self.update_task(task_id, "assignee", assignee_value)
    }

    pub fn get_attachments(&self, task_id: &str) -> Result<Vec<Attachment>> {
        let path = format!("/tasks/{}/attachments", task_id);
        let body = self.get(&path)?;
        let wrapper: DataWrapper<Vec<Attachment>> =
            serde_json::from_str(&body).context("Failed to parse attachments")?;
        Ok(wrapper.data)
    }

    pub fn get_attachment(&self, attachment_id: &str) -> Result<Attachment> {
        let path = format!("/attachments/{}", attachment_id);
        let body = self.get(&path)?;
        let wrapper: DataWrapper<Attachment> =
            serde_json::from_str(&body).context("Failed to parse attachment")?;
        Ok(wrapper.data)
    }

    pub fn download_attachment(&self, url: &str, output_path: &std::path::Path) -> Result<()> {
        let response = self
            .client
            .get(url)
            .bearer_auth(&self.token)
            .send()
            .context("Failed to download attachment")?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to download attachment: {}", response.status());
        }

        let mut file = std::fs::File::create(output_path)
            .with_context(|| format!("Failed to create file: {}", output_path.display()))?;

        let bytes = response
            .bytes()
            .context("Failed to read attachment bytes")?;

        file.write_all(&bytes)
            .context("Failed to write attachment to file")?;

        Ok(())
    }
}
