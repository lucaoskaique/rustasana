use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Base {
    pub gid: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Story {
    pub gid: String,
    pub text: Option<String>,
    #[serde(rename = "type")]
    pub story_type: String,
    pub created_at: String,
    pub created_by: Base,
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
