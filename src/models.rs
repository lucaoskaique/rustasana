use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt;

// Type aliases for better code clarity
pub type TaskId = String;
pub type Gid = String;
pub type WorkspaceId = String;
pub type ProjectId = String;
pub type UserId = String;

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct Base {
    pub gid: String,
    pub name: String,
}

impl fmt::Display for Base {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} (GID: {})", self.name, self.gid)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct CustomField {
    pub gid: String,
    pub name: String,
    pub display_value: Option<String>,
    #[serde(rename = "type")]
    pub field_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attachment {
    pub gid: String,
    pub name: String,
    pub created_at: String,
    pub download_url: Option<String>,
    pub view_url: Option<String>,
    pub permanent_url: Option<String>,
    pub host: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct Task {
    pub gid: String,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub modified_at: Option<String>,
    pub name: String,
    #[serde(default)]
    pub notes: Option<String>,
    #[serde(default)]
    pub assignee: Option<Base>,
    pub completed: bool,
    #[serde(default)]
    pub assignee_status: Option<String>,
    #[serde(default)]
    pub completed_at: Option<String>,
    #[serde(default)]
    pub due_on: Option<String>,
    #[serde(default)]
    pub tags: Option<Vec<Base>>,
    #[serde(default)]
    pub custom_fields: Option<Vec<CustomField>>,
    #[serde(default)]
    pub workspace: Option<Base>,
    #[serde(default)]
    pub parent: Option<Base>,
    #[serde(default)]
    pub projects: Option<Vec<Base>>,
    #[serde(default)]
    pub followers: Option<Vec<Base>>,
}

impl Task {
    /// Compare tasks by due date (earlier dates come first, no date comes last)
    pub fn cmp_by_due_date(&self, other: &Self) -> Ordering {
        match (&self.due_on, &other.due_on) {
            (Some(a), Some(b)) => a.cmp(b),
            (Some(_), None) => Ordering::Less,
            (None, Some(_)) => Ordering::Greater,
            (None, None) => Ordering::Equal,
        }
    }

    /// Get assignee name or empty string
    pub fn assignee_name(&self) -> &str {
        self.assignee
            .as_ref()
            .map(|a| a.name.as_str())
            .unwrap_or("")
    }

    /// Get due date string or empty string
    pub fn due_date_str(&self) -> &str {
        self.due_on.as_deref().unwrap_or("")
    }

    /// Format assignee for display: [@name] or [unassigned]
    pub fn format_assignee(&self) -> String {
        let name = self.assignee_name();
        if name.is_empty() {
            "[unassigned]".to_string()
        } else {
            format!("[@{}]", name)
        }
    }
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cmp_by_due_date(other)
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}] {} {}",
            self.due_date_str(),
            self.name,
            self.format_assignee()
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Story {
    pub gid: String,
    pub text: Option<String>,
    #[serde(rename = "type")]
    pub story_type: String,
    pub created_at: String,
    pub created_by: Base,
}

impl fmt::Display for Story {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = self.text.as_deref().unwrap_or("[no text]");
        write!(f, "{}: {}", self.created_by.name, text)
    }
}

#[derive(Debug, Deserialize)]
pub struct DataWrapper<T> {
    pub data: T,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Me {
    pub gid: String,
    pub name: String,
    pub email: String,
    pub workspaces: Vec<Base>,
}
