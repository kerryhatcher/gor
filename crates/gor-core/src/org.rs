//! Typed operations and models for GitHub organizations.

use crate::client::Client;
use crate::error::GorError;
use serde::{Deserialize, Serialize};

/// A GitHub organization.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Organization {
    /// The organization's login name.
    pub login: String,
    /// Optional description.
    #[serde(default)]
    pub description: Option<String>,
    /// Any additional fields returned by the API not captured above.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// List organizations for the authenticated user.
///
/// # Errors
///
/// Returns [`GorError::Http`] on HTTP failures.
pub fn list(client: &Client, limit: u32) -> Result<Vec<Organization>, GorError> {
    let path = format!("/user/orgs?per_page={}", limit.min(100));
    let response = client.get(&path)?;
    let status = response.status();
    if !status.is_success() {
        return Err(GorError::InvalidInput(format!(
            "failed to list organizations: HTTP {status}"
        )));
    }
    let mut orgs: Vec<Organization> = response.json().map_err(GorError::Http)?;
    orgs.truncate(limit as usize);
    Ok(orgs)
}

/// View an organization by name.
///
/// # Errors
///
/// Returns [`GorError::NotFound`] if the organization does not exist,
/// or [`GorError::Http`] on HTTP failures.
pub fn view(client: &Client, org: &str) -> Result<serde_json::Value, GorError> {
    let path = format!("/orgs/{org}");
    let response = client.get(&path)?;
    let status = response.status();
    if status == reqwest::StatusCode::NOT_FOUND {
        return Err(GorError::NotFound(format!(
            "organization '{org}' not found"
        )));
    }
    if !status.is_success() {
        return Err(GorError::InvalidInput(format!(
            "failed to view organization: HTTP {status}"
        )));
    }
    response.json().map_err(GorError::Http)
}
