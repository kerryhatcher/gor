//! Typed operations and models for GitHub repository labels.
//!
//! Provides functions to list, create, update, delete, and clone labels
//! on GitHub repositories, returning typed [`Label`] structs instead of
//! raw JSON values.

use crate::client::Client;
use crate::error::GorError;
use crate::repository::RepoSplit;
use crate::util::urlencode_label_name;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// A GitHub repository label.
///
/// Fields are hand-picked for gor's usage. Unknown fields from the API
/// are captured in [`extra`](Self::extra) via `#[serde(flatten)]`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Label {
    /// The label's display name.
    pub name: String,
    /// The hex color code (without `#`).
    pub color: String,
    /// An optional description of the label.
    #[serde(default)]
    pub description: Option<String>,
    /// Any additional fields returned by the API not in this struct.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// Options for [`list`].
#[derive(Debug, Default)]
pub struct ListOptions {
    /// Filter labels whose name contains this substring (case-insensitive).
    pub search: Option<String>,
    /// Maximum number of labels to return.
    pub limit: u32,
}

/// Options for [`create`].
#[derive(Debug, Default)]
pub struct CreateOptions {
    /// The hex color code (without `#`). Defaults to `"ededed"`.
    pub color: Option<String>,
    /// An optional description.
    pub description: Option<String>,
}

/// Options for [`update`].
#[derive(Debug, Default)]
pub struct UpdateOptions {
    /// New name for the label.
    pub new_name: Option<String>,
    /// New hex color code.
    pub color: Option<String>,
    /// New description.
    pub description: Option<String>,
}

/// Result of a [`clone_from`] operation.
#[derive(Debug, Clone)]
pub struct CloneResult {
    /// Number of labels created in the target repo.
    pub created: u32,
    /// Number of labels updated in the target repo.
    pub updated: u32,
    /// Number of labels skipped (already exist and `force` was false, or an error occurred).
    pub skipped: u32,
}

/// List labels in a repository.
///
/// # Errors
///
/// Returns [`GorError::NotFound`] if the repository does not exist,
/// or [`GorError::Http`] on HTTP failures.
pub fn list(client: &Client, spec: &RepoSplit, opts: &ListOptions) -> Result<Vec<Label>, GorError> {
    let path = format!(
        "/repos/{}/{}/labels?per_page={}",
        spec.owner,
        spec.repo,
        opts.limit.min(100)
    );
    let response = client.get(&path)?;

    let status = response.status();
    if status == reqwest::StatusCode::NOT_FOUND {
        return Err(GorError::NotFound(format!("repository '{spec}' not found")));
    }
    if !status.is_success() {
        if let Err(e) = response.error_for_status_ref() {
            return Err(GorError::Http(e));
        }
    }

    let mut labels: Vec<Label> = response.json().map_err(GorError::Http)?;

    // Client-side search filter
    if let Some(ref query) = opts.search {
        let query_lower = query.to_lowercase();
        labels.retain(|label| label.name.to_lowercase().contains(&query_lower));
    }

    // Apply limit
    labels.truncate(opts.limit as usize);

    Ok(labels)
}

/// Create a label in a repository.
///
/// # Errors
///
/// Returns [`GorError::InvalidInput`] if the label already exists,
/// [`GorError::NotFound`] if the repository does not exist,
/// or [`GorError::Http`] on HTTP failures.
pub fn create(
    client: &Client,
    spec: &RepoSplit,
    name: &str,
    opts: &CreateOptions,
) -> Result<Label, GorError> {
    let color_value = opts.color.as_deref().unwrap_or("ededed");

    let mut body_map = serde_json::Map::new();
    body_map.insert(
        "name".to_string(),
        serde_json::Value::String(name.to_string()),
    );
    body_map.insert(
        "color".to_string(),
        serde_json::Value::String(color_value.to_string()),
    );
    if let Some(ref desc) = opts.description {
        body_map.insert(
            "description".to_string(),
            serde_json::Value::String(desc.clone()),
        );
    }

    let path = format!("/repos/{}/{}/labels", spec.owner, spec.repo);
    let body_value = serde_json::Value::Object(body_map);
    let response = client.post(&path, &body_value)?;

    let status = response.status();
    if status == reqwest::StatusCode::UNPROCESSABLE_ENTITY {
        return Err(GorError::InvalidInput(format!(
            "label '{name}' already exists in '{spec}'"
        )));
    }
    if !status.is_success() {
        let err_body: serde_json::Value = response.json().unwrap_or_default();
        let msg = err_body["message"].as_str().unwrap_or("creation failed");
        return Err(GorError::InvalidInput(format!(
            "failed to create label '{name}': {msg}"
        )));
    }

    response.json().map_err(GorError::Http)
}

/// Update a label in a repository.
///
/// # Errors
///
/// Returns [`GorError::NotFound`] if the label does not exist,
/// or [`GorError::Http`] on HTTP failures.
pub fn update(
    client: &Client,
    spec: &RepoSplit,
    name: &str,
    opts: &UpdateOptions,
) -> Result<Label, GorError> {
    let mut body_map = serde_json::Map::new();
    if let Some(ref new_name) = opts.new_name {
        body_map.insert(
            "new_name".to_string(),
            serde_json::Value::String(new_name.clone()),
        );
    }
    if let Some(ref c) = opts.color {
        body_map.insert("color".to_string(), serde_json::Value::String(c.clone()));
    }
    if let Some(ref desc) = opts.description {
        body_map.insert(
            "description".to_string(),
            serde_json::Value::String(desc.clone()),
        );
    }

    if body_map.is_empty() {
        return Err(GorError::InvalidInput(
            "no changes specified; use --rename, --color, or --description".to_string(),
        ));
    }

    let path = format!(
        "/repos/{}/{}/labels/{}",
        spec.owner,
        spec.repo,
        urlencode_label_name(name)
    );
    let body_value = serde_json::Value::Object(body_map);
    let body_bytes = serde_json::to_vec(&body_value)
        .map_err(|e| GorError::InvalidInput(format!("failed to serialize request body: {e}")))?;
    let response = client.request("PATCH", &path, &[], Some(body_bytes))?;

    let status = response.status();
    if status == reqwest::StatusCode::NOT_FOUND {
        return Err(GorError::NotFound(format!(
            "label '{name}' not found in '{spec}'"
        )));
    }
    if !status.is_success() {
        let err_body: serde_json::Value = response.json().unwrap_or_default();
        let msg = err_body["message"].as_str().unwrap_or("edit failed");
        return Err(GorError::InvalidInput(format!(
            "failed to edit label '{name}': {msg}"
        )));
    }

    response.json().map_err(GorError::Http)
}

/// Delete a label from a repository.
///
/// # Errors
///
/// Returns [`GorError::NotFound`] if the label does not exist,
/// or [`GorError::Http`] on HTTP failures.
pub fn delete(client: &Client, spec: &RepoSplit, name: &str) -> Result<(), GorError> {
    let path = format!(
        "/repos/{}/{}/labels/{}",
        spec.owner,
        spec.repo,
        urlencode_label_name(name)
    );
    let response = client.request("DELETE", &path, &[], None)?;

    let status = response.status();
    if status == reqwest::StatusCode::NOT_FOUND {
        return Err(GorError::NotFound(format!(
            "label '{name}' not found in '{spec}'"
        )));
    }
    if !status.is_success() {
        if let Err(e) = response.error_for_status_ref() {
            return Err(GorError::Http(e));
        }
    }

    Ok(())
}

/// Clone labels from one repository to another.
///
/// Fetches all labels from `source` and creates or updates them in `target`.
/// When `force` is `true`, existing labels in the target are updated.
///
/// # Errors
///
/// Returns [`GorError::NotFound`] if either repository does not exist,
/// or [`GorError::Http`] on HTTP failures.
pub fn clone_from(
    client: &Client,
    source: &RepoSplit,
    target: &RepoSplit,
    force: bool,
) -> Result<CloneResult, GorError> {
    // Fetch source labels
    let source_path = format!(
        "/repos/{}/{}/labels?per_page=100",
        source.owner, source.repo
    );
    let resp = client.get(&source_path)?;
    let status = resp.status();
    if status == reqwest::StatusCode::NOT_FOUND {
        return Err(GorError::NotFound(format!(
            "source repository '{source}' not found"
        )));
    }
    if !status.is_success() {
        if let Err(e) = resp.error_for_status_ref() {
            return Err(GorError::Http(e));
        }
    }
    let source_labels: Vec<Label> = resp.json().map_err(GorError::Http)?;

    // Fetch existing target labels for conflict detection
    let target_path = format!(
        "/repos/{}/{}/labels?per_page=100",
        target.owner, target.repo
    );
    let existing: Vec<Label> = client
        .get(&target_path)
        .ok()
        .and_then(|r| r.json().ok())
        .unwrap_or_default();
    let existing_names: HashSet<&str> = existing.iter().map(|l| l.name.as_str()).collect();

    let mut result = CloneResult {
        created: 0,
        updated: 0,
        skipped: 0,
    };

    for label in &source_labels {
        if existing_names.contains(label.name.as_str()) {
            if force {
                // Update existing label
                let opts = UpdateOptions {
                    color: Some(label.color.clone()),
                    description: label.description.clone(),
                    ..Default::default()
                };
                if update(client, target, &label.name, &opts).is_ok() {
                    result.updated += 1;
                } else {
                    result.skipped += 1;
                }
            } else {
                result.skipped += 1;
            }
        } else {
            // Create new label
            let opts = CreateOptions {
                color: Some(label.color.clone()),
                description: label.description.clone(),
            };
            if create(client, target, &label.name, &opts).is_ok() {
                result.created += 1;
            } else {
                result.skipped += 1;
            }
        }
    }

    Ok(result)
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn label_deserialization_basic() {
        let json = json!({
            "name": "bug",
            "color": "d73a4a",
            "description": "Something isn't working"
        });
        let label: Label = serde_json::from_value(json).expect("valid label");
        assert_eq!(label.name, "bug");
        assert_eq!(label.color, "d73a4a");
        assert_eq!(
            label.description.as_deref(),
            Some("Something isn't working")
        );
    }

    #[test]
    fn label_deserialization_no_description() {
        let json = json!({
            "name": "bug",
            "color": "d73a4a"
        });
        let label: Label = serde_json::from_value(json).expect("valid label");
        assert_eq!(label.name, "bug");
        assert_eq!(label.color, "d73a4a");
        assert!(label.description.is_none());
    }

    #[test]
    fn label_deserialization_extra_fields() {
        let json = json!({
            "name": "bug",
            "color": "d73a4a",
            "default": true,
            "url": "https://api.github.com/repos/o/r/labels/bug"
        });
        let label: Label = serde_json::from_value(json).expect("valid label");
        assert_eq!(label.name, "bug");
        assert!(label.extra.contains_key("default"));
        assert!(label.extra.contains_key("url"));
    }

    #[test]
    fn label_serialization() {
        let label = Label {
            name: "bug".to_string(),
            color: "d73a4a".to_string(),
            description: Some("Bug report".to_string()),
            extra: serde_json::Map::new(),
        };
        let json = serde_json::to_value(&label).expect("valid json");
        assert_eq!(json["name"], "bug");
        assert_eq!(json["color"], "d73a4a");
        assert_eq!(json["description"], "Bug report");
    }

    #[test]
    fn clone_result_default() {
        let r = CloneResult {
            created: 0,
            updated: 0,
            skipped: 0,
        };
        assert_eq!(r.created, 0);
        assert_eq!(r.updated, 0);
        assert_eq!(r.skipped, 0);
    }

    #[test]
    fn list_options_default() {
        let opts = ListOptions::default();
        assert!(opts.search.is_none());
        assert_eq!(opts.limit, 0);
    }

    #[test]
    fn create_options_default() {
        let opts = CreateOptions::default();
        assert!(opts.color.is_none());
        assert!(opts.description.is_none());
    }

    #[test]
    fn update_options_default() {
        let opts = UpdateOptions::default();
        assert!(opts.new_name.is_none());
        assert!(opts.color.is_none());
        assert!(opts.description.is_none());
    }
}
