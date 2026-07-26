//! Typed operations and models for SSH and GPG keys.

#![allow(clippy::missing_errors_doc)]
use crate::client::Client;
use crate::error::GorError;
use serde::{Deserialize, Serialize};

/// An SSH key.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SshKey {
    /// The key ID.
    pub id: u64,
    /// The key title.
    pub title: String,
    /// The full key string.
    pub key: String,
    /// Any additional fields returned by the API.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// A GPG key.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GpgKey {
    /// The key ID string.
    pub key_id: Option<String>,
    /// The key name.
    pub name: Option<String>,
    /// Any additional fields returned by the API.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// List SSH keys for the authenticated user.
pub fn list_ssh(client: &Client) -> Result<Vec<SshKey>, GorError> {
    let response = client.get("/user/keys")?;
    let status = response.status();
    if !status.is_success() {
        return Err(GorError::InvalidInput(format!(
            "failed to list SSH keys: HTTP {status}"
        )));
    }
    response.json().map_err(GorError::Http)
}

/// Add an SSH key.
pub fn add_ssh(client: &Client, title: &str, key_body: &str) -> Result<SshKey, GorError> {
    let body = serde_json::json!({"title": title, "key": key_body});
    let response = client.post("/user/keys", &body)?;
    let status = response.status();
    if !status.is_success() {
        let err: serde_json::Value = response.json().unwrap_or_default();
        let msg = err["message"].as_str().unwrap_or("add failed");
        return Err(GorError::InvalidInput(format!(
            "failed to add SSH key: {msg}"
        )));
    }
    response.json().map_err(GorError::Http)
}

/// Delete an SSH key.
pub fn delete_ssh(client: &Client, key_id: u64) -> Result<(), GorError> {
    let path = format!("/user/keys/{key_id}");
    let response = client.request("DELETE", &path, &[], None)?;
    let status = response.status();
    if !status.is_success() {
        let err: serde_json::Value = response.json().unwrap_or_default();
        let msg = err["message"].as_str().unwrap_or("delete failed");
        return Err(GorError::InvalidInput(format!(
            "failed to delete SSH key: {msg}"
        )));
    }
    Ok(())
}

/// List GPG keys for the authenticated user.
pub fn list_gpg(client: &Client) -> Result<Vec<GpgKey>, GorError> {
    let response = client.get("/user/gpg_keys")?;
    let status = response.status();
    if !status.is_success() {
        return Err(GorError::InvalidInput(format!(
            "failed to list GPG keys: HTTP {status}"
        )));
    }
    response.json().map_err(GorError::Http)
}

/// Add a GPG key.
pub fn add_gpg(client: &Client, armored_key: &str) -> Result<GpgKey, GorError> {
    let body = serde_json::json!({"armored_public_key": armored_key});
    let response = client.post("/user/gpg_keys", &body)?;
    let status = response.status();
    if !status.is_success() {
        let err: serde_json::Value = response.json().unwrap_or_default();
        let msg = err["message"].as_str().unwrap_or("add failed");
        return Err(GorError::InvalidInput(format!(
            "failed to add GPG key: {msg}"
        )));
    }
    response.json().map_err(GorError::Http)
}

/// Delete a GPG key.
pub fn delete_gpg(client: &Client, key_id: &str) -> Result<(), GorError> {
    let path = format!("/user/gpg_keys/{key_id}");
    let response = client.request("DELETE", &path, &[], None)?;
    let status = response.status();
    if !status.is_success() {
        let err: serde_json::Value = response.json().unwrap_or_default();
        let msg = err["message"].as_str().unwrap_or("delete failed");
        return Err(GorError::InvalidInput(format!(
            "failed to delete GPG key: {msg}"
        )));
    }
    Ok(())
}
