//! Typed operations and models for GitHub Actions variables.

use crate::client::Client;
use crate::error::GorError;
use crate::repository::RepoSplit;
use serde::{Deserialize, Serialize};

/// A GitHub Actions variable.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Variable {
    /// The variable name.
    pub name: String,
    /// The variable value.
    pub value: String,
    /// ISO 8601 timestamp of last update.
    pub updated_at: Option<String>,
    /// ISO 8601 timestamp of creation.
    pub created_at: Option<String>,
    /// Any additional fields returned by the API.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// Scope for a variable operation.
pub enum VariableScope {
    /// Repository-level variable.
    Repo(RepoSplit),
    /// Environment-level variable.
    Environment {
        /// The repository specification.
        spec: RepoSplit,
        /// The environment name.
        env: String,
    },
    /// Organization-level variable.
    Org(String),
}

impl VariableScope {
    fn api_path(&self, name: &str) -> String {
        match self {
            Self::Repo(spec) => {
                format!(
                    "/repos/{}/{}/actions/variables/{name}",
                    spec.owner, spec.repo
                )
            }
            Self::Environment { spec, env } => {
                format!(
                    "/repos/{}/{}/environments/{env}/variables/{name}",
                    spec.owner, spec.repo
                )
            }
            Self::Org(org) => format!("/orgs/{org}/actions/variables/{name}"),
        }
    }

    fn api_list_path(&self) -> String {
        match self {
            Self::Repo(spec) => {
                format!(
                    "/repos/{}/{}/actions/variables?per_page=100",
                    spec.owner, spec.repo
                )
            }
            Self::Environment { spec, env } => {
                format!(
                    "/repos/{}/{}/environments/{env}/variables?per_page=100",
                    spec.owner, spec.repo
                )
            }
            Self::Org(org) => format!("/orgs/{org}/actions/variables?per_page=100"),
        }
    }
}

/// List variables.
///
/// # Errors
///
/// Returns [`GorError::Http`] on HTTP failures.
pub fn list(client: &Client, scope: &VariableScope) -> Result<Vec<Variable>, GorError> {
    let path = scope.api_list_path();
    let response = client.get(&path)?;
    let status = response.status();
    if !status.is_success() {
        return Err(GorError::InvalidInput(format!(
            "failed to list variables: HTTP {status}"
        )));
    }
    let result: serde_json::Value = response.json().map_err(GorError::Http)?;
    let variables: Vec<Variable> = serde_json::from_value(result["variables"].clone())
        .map_err(|e| GorError::InvalidInput(format!("failed to parse variables: {e}")))?;
    Ok(variables)
}

/// Set a variable value.
///
/// # Errors
///
/// Returns [`GorError::Http`] on HTTP failures.
pub fn set(
    client: &Client,
    scope: &VariableScope,
    name: &str,
    value: &str,
) -> Result<(), GorError> {
    let body = serde_json::json!({"value": value});
    let path = scope.api_path(name);
    let body_bytes = serde_json::to_vec(&body)
        .map_err(|e| GorError::InvalidInput(format!("serialization error: {e}")))?;
    let response = client.request("PATCH", &path, &[], Some(body_bytes))?;
    let status = response.status();
    if !status.is_success() {
        return Err(GorError::InvalidInput(format!(
            "failed to set variable '{name}': HTTP {status}"
        )));
    }
    Ok(())
}

/// Delete a variable.
///
/// # Errors
///
/// Returns [`GorError::NotFound`] if the variable does not exist,
/// or [`GorError::Http`] on HTTP failures.
pub fn delete(client: &Client, scope: &VariableScope, name: &str) -> Result<(), GorError> {
    let path = scope.api_path(name);
    let response = client.request("DELETE", &path, &[], None)?;
    let status = response.status();
    if status == reqwest::StatusCode::NOT_FOUND {
        return Err(GorError::NotFound(format!("variable '{name}' not found")));
    }
    if !status.is_success() {
        return Err(GorError::InvalidInput(format!(
            "failed to delete variable '{name}': HTTP {status}"
        )));
    }
    Ok(())
}
