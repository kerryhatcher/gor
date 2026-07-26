//! Typed operations for GitHub search.

#![allow(clippy::missing_errors_doc)]
use crate::client::Client;
use crate::error::GorError;
use serde::{Deserialize, Serialize};
use std::fmt::Write;

/// A search result item.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SearchResult {
    /// The item's ID.
    pub id: u64,
    /// Any additional fields returned by the API.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// Search repositories.
pub fn search_repos(
    client: &Client,
    query: &str,
    sort: &str,
    order: &str,
    limit: u32,
) -> Result<Vec<serde_json::Value>, GorError> {
    let per_page = limit.min(100);
    let path = format!(
        "/search/repositories?q={}&sort={}&order={}&per_page={per_page}",
        urlencoding(query),
        sort,
        order,
    );
    let response = client.get(&path)?;
    let status = response.status();
    if !status.is_success() {
        return Err(GorError::InvalidInput(format!(
            "search failed: HTTP {status}"
        )));
    }
    let result: serde_json::Value = response.json().map_err(GorError::Http)?;
    let mut items: Vec<serde_json::Value> = result["items"].as_array().cloned().unwrap_or_default();
    items.truncate(limit as usize);
    Ok(items)
}

/// Search code.
pub fn search_code(
    client: &Client,
    query: &str,
    limit: u32,
) -> Result<Vec<serde_json::Value>, GorError> {
    let per_page = limit.min(100);
    let path = format!("/search/code?q={}&per_page={per_page}", urlencoding(query));
    let response = client.get(&path)?;
    let status = response.status();
    if !status.is_success() {
        return Err(GorError::InvalidInput(format!(
            "search failed: HTTP {status}"
        )));
    }
    let result: serde_json::Value = response.json().map_err(GorError::Http)?;
    let mut items: Vec<serde_json::Value> = result["items"].as_array().cloned().unwrap_or_default();
    items.truncate(limit as usize);
    Ok(items)
}

/// Search issues and PRs.
pub fn search_issues(
    client: &Client,
    query: &str,
    limit: u32,
) -> Result<Vec<serde_json::Value>, GorError> {
    let per_page = limit.min(100);
    let path = format!(
        "/search/issues?q={}&per_page={per_page}",
        urlencoding(query)
    );
    let response = client.get(&path)?;
    let status = response.status();
    if !status.is_success() {
        return Err(GorError::InvalidInput(format!(
            "search failed: HTTP {status}"
        )));
    }
    let result: serde_json::Value = response.json().map_err(GorError::Http)?;
    let mut items: Vec<serde_json::Value> = result["items"].as_array().cloned().unwrap_or_default();
    items.truncate(limit as usize);
    Ok(items)
}

/// Search commits.
pub fn search_commits(
    client: &Client,
    query: &str,
    limit: u32,
) -> Result<Vec<serde_json::Value>, GorError> {
    let per_page = limit.min(100);
    let path = format!(
        "/search/commits?q={}&per_page={per_page}",
        urlencoding(query)
    );
    let response = client.get(&path)?;
    let status = response.status();
    if !status.is_success() {
        return Err(GorError::InvalidInput(format!(
            "search failed: HTTP {status}"
        )));
    }
    let result: serde_json::Value = response.json().map_err(GorError::Http)?;
    let mut items: Vec<serde_json::Value> = result["items"].as_array().cloned().unwrap_or_default();
    items.truncate(limit as usize);
    Ok(items)
}

fn urlencoding(s: &str) -> String {
    urlencoding_helper(s).replace(' ', "+")
}

fn urlencoding_helper(s: &str) -> String {
    let mut result = String::new();
    for byte in s.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                result.push(byte as char);
            }
            b' ' => result.push('+'),
            _ => {
                let _ = write!(result, "%{byte:02X}");
            }
        }
    }
    result
}
