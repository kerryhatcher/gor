//! Typed operations and models for GitHub Actions secrets.

use crate::client::Client;
use crate::error::GorError;
use crate::repository::RepoSplit;
use serde::{Deserialize, Serialize};

/// A GitHub Actions secret.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Secret {
    /// The secret name.
    pub name: String,
    /// ISO 8601 timestamp of last update.
    pub updated_at: Option<String>,
    /// ISO 8601 timestamp of creation.
    pub created_at: Option<String>,
    /// Any additional fields returned by the API.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// Scope for a secret operation.
pub enum SecretScope {
    /// Repository-level secret.
    Repo(RepoSplit),
    /// Environment-level secret within a repository.
    Environment {
        /// The repository specification.
        spec: RepoSplit,
        /// The environment name.
        env: String,
    },
    /// Organization-level secret.
    Org(String),
}

impl SecretScope {
    fn api_path(&self, name: &str) -> String {
        match self {
            Self::Repo(spec) => {
                format!("/repos/{}/{}/actions/secrets/{name}", spec.owner, spec.repo)
            }
            Self::Environment { spec, env } => {
                format!(
                    "/repos/{}/{}/environments/{env}/secrets/{name}",
                    spec.owner, spec.repo
                )
            }
            Self::Org(org) => format!("/orgs/{org}/actions/secrets/{name}"),
        }
    }

    fn api_list_path(&self) -> String {
        match self {
            Self::Repo(spec) => {
                format!(
                    "/repos/{}/{}/actions/secrets?per_page=100",
                    spec.owner, spec.repo
                )
            }
            Self::Environment { spec, env } => {
                format!(
                    "/repos/{}/{}/environments/{env}/secrets?per_page=100",
                    spec.owner, spec.repo
                )
            }
            Self::Org(org) => format!("/orgs/{org}/actions/secrets?per_page=100"),
        }
    }
}

/// List secrets.
///
/// # Errors
///
/// Returns [`GorError::Http`] on HTTP failures.
pub fn list(client: &Client, scope: &SecretScope) -> Result<Vec<Secret>, GorError> {
    let path = scope.api_list_path();
    let response = client.get(&path)?;
    let status = response.status();
    if !status.is_success() {
        return Err(GorError::InvalidInput(format!(
            "failed to list secrets: HTTP {status}"
        )));
    }
    let result: serde_json::Value = response.json().map_err(GorError::Http)?;
    let secrets: Vec<Secret> = serde_json::from_value(result["secrets"].clone())
        .map_err(|e| GorError::InvalidInput(format!("failed to parse secrets: {e}")))?;
    Ok(secrets)
}

/// Set a secret value.
///
/// # Errors
///
/// Returns [`GorError::InvalidInput`] if no value is provided,
/// or [`GorError::Http`] on HTTP failures.
pub fn set(client: &Client, scope: &SecretScope, name: &str, value: &str) -> Result<(), GorError> {
    let body = serde_json::json!({"encrypted_value": value});
    let path = scope.api_path(name);
    let body_bytes = serde_json::to_vec(&body)
        .map_err(|e| GorError::InvalidInput(format!("serialization error: {e}")))?;
    let response = client.request("PUT", &path, &[], Some(body_bytes))?;
    let status = response.status();
    if !status.is_success() {
        return Err(GorError::InvalidInput(format!(
            "failed to set secret '{name}': HTTP {status}"
        )));
    }
    Ok(())
}

/// Delete a secret.
///
/// # Errors
///
/// Returns [`GorError::NotFound`] if the secret does not exist,
/// or [`GorError::Http`] on HTTP failures.
pub fn delete(client: &Client, scope: &SecretScope, name: &str) -> Result<(), GorError> {
    let path = scope.api_path(name);
    let response = client.request("DELETE", &path, &[], None)?;
    let status = response.status();
    if status == reqwest::StatusCode::NOT_FOUND {
        return Err(GorError::NotFound(format!("secret '{name}' not found")));
    }
    if !status.is_success() {
        return Err(GorError::InvalidInput(format!(
            "failed to delete secret '{name}': HTTP {status}"
        )));
    }
    Ok(())
}
