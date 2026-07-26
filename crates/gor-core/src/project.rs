//! Typed operations and models for GitHub Projects.

use crate::client::Client;
use crate::error::GorError;
use crate::repository::RepoSplit;
use serde::{Deserialize, Serialize};

/// A GitHub Project (classic).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Project {
    /// The project number.
    pub number: u64,
    /// The project name.
    pub name: String,
    /// The project state (open/closed).
    pub state: Option<String>,
    /// Visibility (public/private).
    pub visibility: Option<String>,
    /// Any additional fields returned by the API.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// Scope for listing projects.
pub enum ProjectScope {
    /// Organization projects.
    Org(String),
    /// User projects.
    User(String),
    /// Repository projects.
    Repo(RepoSplit),
}

impl ProjectScope {
    fn api_list_path(&self, limit: u32) -> String {
        let per_page = limit.min(100);
        match self {
            Self::Org(org) => format!("/orgs/{org}/projects?per_page={per_page}"),
            Self::User(user) => format!("/users/{user}/projects?per_page={per_page}"),
            Self::Repo(spec) => {
                format!(
                    "/repos/{}/{}/projects?per_page={per_page}",
                    spec.owner, spec.repo
                )
            }
        }
    }
}

/// List projects (classic REST API).
///
/// # Errors
///
/// Returns [`GorError::Http`] on HTTP failures.
pub fn list(client: &Client, scope: &ProjectScope, limit: u32) -> Result<Vec<Project>, GorError> {
    let path = scope.api_list_path(limit);
    let response = client.get(&path)?;
    let status = response.status();
    if !status.is_success() {
        return Err(GorError::InvalidInput(format!(
            "failed to list projects: HTTP {status}"
        )));
    }
    let mut projects: Vec<Project> = response.json().map_err(GorError::Http)?;
    projects.truncate(limit as usize);
    Ok(projects)
}

/// View a project by number.
///
/// # Errors
///
/// Returns [`GorError::NotFound`] if the project does not exist.
pub fn view(client: &Client, number: u64) -> Result<serde_json::Value, GorError> {
    let path = format!("/projects/{number}");
    let response = client.get(&path)?;
    let status = response.status();
    if status == reqwest::StatusCode::NOT_FOUND {
        return Err(GorError::NotFound(format!("project #{number} not found")));
    }
    if !status.is_success() {
        return Err(GorError::InvalidInput(format!(
            "failed to view project: HTTP {status}"
        )));
    }
    response.json().map_err(GorError::Http)
}

/// Add an item to a project.
///
/// # Errors
///
/// Returns [`GorError::Http`] on HTTP failures.
pub fn item_add(
    client: &Client,
    project: u64,
    content_id: u64,
    content_type: &str,
) -> Result<serde_json::Value, GorError> {
    let body = serde_json::json!({
        "content_id": content_id,
        "content_type": content_type,
    });
    let path = format!("/projects/{project}/items");
    let body_bytes = serde_json::to_vec(&body)
        .map_err(|e| GorError::InvalidInput(format!("serialization error: {e}")))?;
    let response = client.request("POST", &path, &[], Some(body_bytes))?;
    let status = response.status();
    if !status.is_success() {
        let err: serde_json::Value = response.json().unwrap_or_default();
        let msg = err["message"].as_str().unwrap_or("add failed");
        return Err(GorError::InvalidInput(format!(
            "failed to add item to project #{project}: {msg}"
        )));
    }
    response.json().map_err(GorError::Http)
}
