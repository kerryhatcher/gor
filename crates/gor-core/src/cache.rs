//! Typed operations and models for GitHub Actions caches.

use crate::client::Client;
use crate::error::GorError;
use crate::repository::RepoSplit;
use serde::{Deserialize, Serialize};

/// A GitHub Actions cache entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Cache {
    /// The cache key.
    pub key: String,
    /// Size in bytes.
    #[serde(rename = "size_in_bytes")]
    pub size_in_bytes: u64,
    /// ISO 8601 creation timestamp.
    #[serde(rename = "created_at")]
    pub created_at: Option<String>,
    /// Any additional fields returned by the API not captured above.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// List caches in a repository.
///
/// # Errors
///
/// Returns [`GorError::Http`] on HTTP failures.
pub fn list(client: &Client, spec: &RepoSplit) -> Result<Vec<Cache>, GorError> {
    let path = format!(
        "/repos/{}/{}/actions/caches?per_page=100",
        spec.owner, spec.repo
    );
    let response = client.get(&path)?;
    let status = response.status();
    if !status.is_success() {
        return Err(GorError::InvalidInput(format!(
            "failed to list caches: HTTP {status}"
        )));
    }
    let result: serde_json::Value = response.json().map_err(GorError::Http)?;
    let caches: Vec<Cache> = serde_json::from_value(result["actions_caches"].clone())
        .map_err(|e| GorError::InvalidInput(format!("failed to parse caches: {e}")))?;
    Ok(caches)
}

/// Options for [`delete`].
#[derive(Debug, Default)]
pub struct DeleteOptions<'a> {
    /// Delete a specific cache key.
    pub key: Option<&'a str>,
    /// Delete caches with a key prefix.
    pub key_prefix: Option<&'a str>,
    /// Filter by Git ref.
    pub ref_: Option<&'a str>,
}

/// Delete caches from a repository.
///
/// # Errors
///
/// Returns [`GorError::Http`] on HTTP failures.
pub fn delete(
    client: &Client,
    spec: &RepoSplit,
    opts: &DeleteOptions<'_>,
) -> Result<u64, GorError> {
    use std::fmt::Write;
    let mut path = format!("/repos/{}/{}/actions/caches", spec.owner, spec.repo);
    if let Some(k) = opts.key {
        let _ = write!(path, "?key={k}");
    } else if let Some(prefix) = opts.key_prefix {
        let _ = write!(path, "?key={prefix}");
    }
    if let Some(r) = opts.ref_ {
        let sep = if path.contains('?') { "&" } else { "?" };
        let _ = write!(path, "{sep}ref={r}");
    }
    let response = client.request("DELETE", &path, &[], None)?;
    let status = response.status();
    if !status.is_success() {
        return Err(GorError::InvalidInput(format!(
            "failed to delete caches: HTTP {status}"
        )));
    }
    let result: serde_json::Value = response.json().map_err(GorError::Http)?;
    Ok(result["total_count"].as_u64().unwrap_or(0))
}
