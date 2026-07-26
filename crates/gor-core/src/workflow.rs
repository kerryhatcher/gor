//! Typed operations and models for GitHub Actions workflows.

#![allow(clippy::missing_errors_doc)]
use crate::client::Client;
use crate::error::GorError;
use crate::repository::RepoSplit;
use serde::{Deserialize, Serialize};

/// A GitHub Actions workflow.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Workflow {
    /// The workflow ID.
    pub id: u64,
    /// The workflow name.
    pub name: String,
    /// The workflow state (active/deleted/disabled_inactivity/etc).
    pub state: Option<String>,
    /// Any additional fields returned by the API.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// List workflows in a repository.
pub fn list(client: &Client, spec: &RepoSplit, limit: u32) -> Result<Vec<Workflow>, GorError> {
    let per_page = limit.min(100);
    let path = format!(
        "/repos/{}/{}/actions/workflows?per_page={per_page}",
        spec.owner, spec.repo
    );
    let response = client.get(&path)?;
    let status = response.status();
    if !status.is_success() {
        return Err(GorError::InvalidInput(format!(
            "failed to list workflows: HTTP {status}"
        )));
    }
    let result: serde_json::Value = response.json().map_err(GorError::Http)?;
    let mut workflows: Vec<Workflow> = serde_json::from_value(result["workflows"].clone())
        .map_err(|e| GorError::InvalidInput(format!("failed to parse workflows: {e}")))?;
    workflows.truncate(limit as usize);
    Ok(workflows)
}

/// View a workflow by ID or filename.
pub fn view(
    client: &Client,
    spec: &RepoSplit,
    workflow: &str,
) -> Result<serde_json::Value, GorError> {
    let path = format!(
        "/repos/{}/{}/actions/workflows/{workflow}",
        spec.owner, spec.repo
    );
    let response = client.get(&path)?;
    let status = response.status();
    if status == reqwest::StatusCode::NOT_FOUND {
        return Err(GorError::NotFound(format!(
            "workflow '{workflow}' not found"
        )));
    }
    if !status.is_success() {
        return Err(GorError::InvalidInput(format!("HTTP {status}")));
    }
    response.json().map_err(GorError::Http)
}

/// Enable a workflow by ID or filename.
pub fn enable(client: &Client, spec: &RepoSplit, workflow: &str) -> Result<(), GorError> {
    let path = format!(
        "/repos/{}/{}/actions/workflows/{workflow}/enable",
        spec.owner, spec.repo
    );
    let response = client.request("PUT", &path, &[], None)?;
    let status = response.status();
    if !status.is_success() {
        return Err(GorError::InvalidInput(format!(
            "failed to enable workflow: HTTP {status}"
        )));
    }
    Ok(())
}

/// Disable a workflow by ID or filename.
pub fn disable(client: &Client, spec: &RepoSplit, workflow: &str) -> Result<(), GorError> {
    let path = format!(
        "/repos/{}/{}/actions/workflows/{workflow}/disable",
        spec.owner, spec.repo
    );
    let response = client.request("PUT", &path, &[], None)?;
    let status = response.status();
    if !status.is_success() {
        return Err(GorError::InvalidInput(format!(
            "failed to disable workflow: HTTP {status}"
        )));
    }
    Ok(())
}

/// Trigger a workflow run.
pub fn trigger_run(
    client: &Client,
    spec: &RepoSplit,
    workflow: &str,
    branch: &str,
) -> Result<(), GorError> {
    let body = serde_json::json!({"ref": branch});
    let body_bytes = serde_json::to_vec(&body)
        .map_err(|e| GorError::InvalidInput(format!("serialization error: {e}")))?;
    let path = format!(
        "/repos/{}/{}/actions/workflows/{workflow}/dispatches",
        spec.owner, spec.repo
    );
    let response = client.request("POST", &path, &[], Some(body_bytes))?;
    let status = response.status();
    if !status.is_success() {
        return Err(GorError::InvalidInput(format!(
            "failed to trigger workflow run: HTTP {status}"
        )));
    }
    Ok(())
}
