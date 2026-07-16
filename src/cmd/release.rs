//! Implementation of the `gor release` subcommand.
//!
//! Provides release listing functionality.
//! Currently supports `gor release list` for listing repository releases.

#![allow(clippy::print_stdout)]

use crate::cli::ReleaseCommand;
use crate::client::Client;
use crate::output::{format_date, print_json};
use crate::repository::{detect_remote, parse_repo_spec};
use anyhow::Context;

/// Run the `gor release` subcommand.
///
/// # Errors
///
/// Returns an error if the command execution fails.
pub fn run(cmd: ReleaseCommand) -> anyhow::Result<()> {
    match cmd {
        ReleaseCommand::List {
            repo,
            limit,
            exclude_drafts,
            exclude_prereleases,
            json,
            hostname,
        } => list(
            repo.as_deref(),
            limit,
            exclude_drafts,
            exclude_prereleases,
            json,
            hostname.as_deref(),
        ),
    }
}

/// Execute `gor release list`.
///
/// Lists releases for a repository. Supports filtering by draft and prerelease
/// status, limiting results, and JSON output with field selection.
///
/// # Errors
///
/// Returns an error if the repository cannot be found or the API request fails.
fn list(
    repo: Option<&str>,
    limit: u32,
    exclude_drafts: bool,
    exclude_prereleases: bool,
    json: Option<Vec<String>>,
    hostname: Option<&str>,
) -> anyhow::Result<()> {
    let spec = match repo {
        Some(s) => parse_repo_spec(s).context("invalid repository spec")?,
        None => detect_remote().ok_or_else(|| {
            anyhow::anyhow!(
                "could not detect repository from current directory; specify OWNER/REPO with --repo"
            )
        })?,
    };

    let host = hostname.unwrap_or("github.com");
    let client = Client::new(host).context("failed to create HTTP client")?;

    let path = format!(
        "/repos/{}/{}/releases?per_page={}",
        spec.owner,
        spec.repo,
        limit.min(100)
    );
    let response = client.get(&path).context("failed to fetch releases")?;

    let status = response.status();
    if status == reqwest::StatusCode::NOT_FOUND {
        anyhow::bail!("repository '{spec}' not found");
    }
    if !status.is_success() {
        anyhow::bail!("failed to list releases for '{spec}': HTTP {status}");
    }

    let mut releases: Vec<serde_json::Value> = response
        .json()
        .context("failed to parse releases response")?;

    // Client-side filtering
    if exclude_drafts {
        releases.retain(|r| !r["draft"].as_bool().unwrap_or(false));
    }
    if exclude_prereleases {
        releases.retain(|r| !r["prerelease"].as_bool().unwrap_or(false));
    }

    // Apply limit
    releases.truncate(limit as usize);

    // Handle --json flag
    if let Some(fields) = json {
        let fields_ref: Option<&[String]> = if fields.is_empty() {
            None
        } else {
            Some(&fields)
        };
        print_json(&releases, fields_ref);
        return Ok(());
    }

    // Default: print formatted table
    print_release_table(&releases);
    Ok(())
}

/// Print a formatted release list table.
///
/// Columns: TAG NAME, RELEASE NAME, PUBLISHED, STATUS
fn print_release_table(releases: &[serde_json::Value]) {
    if releases.is_empty() {
        println!("No releases found.");
        return;
    }

    let tag_width = 20;
    let name_width = 40;
    let date_width = 16;
    let status_width = 12;

    println!(
        "{:<tag_width$}  {:<name_width$}  {:<date_width$}  {:<status_width$}",
        "TAG", "NAME", "PUBLISHED", "STATUS",
    );

    for release in releases {
        let tag = release["tag_name"].as_str().unwrap_or("—");
        let name = release["name"].as_str().unwrap_or("—");
        let published = release["published_at"]
            .as_str()
            .map_or_else(|| "—".to_string(), format_date);
        let is_draft = release["draft"].as_bool().unwrap_or(false);
        let is_prerelease = release["prerelease"].as_bool().unwrap_or(false);
        let status = if is_draft {
            "Draft"
        } else if is_prerelease {
            "Pre-release"
        } else {
            "Latest"
        };

        let tag_truncated = crate::cmd::util::truncate(tag, tag_width);
        let name_truncated = crate::cmd::util::truncate(name, name_width);

        println!(
            "{tag_truncated:<tag_width$}  {name_truncated:<name_width$}  {published:<date_width$}  {status:<status_width$}",
        );
    }
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn print_release_table_basic() {
        let releases = vec![json!({
            "tag_name": "v1.0.0",
            "name": "First Release",
            "published_at": "2024-01-15T10:30:00Z",
            "draft": false,
            "prerelease": false
        })];
        print_release_table(&releases);
    }

    #[test]
    fn print_release_table_empty() {
        let releases: Vec<serde_json::Value> = vec![];
        print_release_table(&releases);
    }

    #[test]
    fn print_release_table_multiple() {
        let releases = vec![
            json!({
                "tag_name": "v1.0.0",
                "name": "First Release",
                "published_at": "2024-01-15T10:30:00Z",
                "draft": false,
                "prerelease": false
            }),
            json!({
                "tag_name": "v2.0.0-beta",
                "name": "Beta Release",
                "published_at": "2024-03-01T00:00:00Z",
                "draft": false,
                "prerelease": true
            }),
            json!({
                "tag_name": "v2.0.0-wip",
                "name": "Work in Progress",
                "published_at": null,
                "draft": true,
                "prerelease": false
            }),
        ];
        print_release_table(&releases);
    }

    #[test]
    fn print_release_table_null_fields() {
        let releases = vec![json!({
            "tag_name": null,
            "name": null,
            "published_at": null,
            "draft": null,
            "prerelease": null
        })];
        print_release_table(&releases);
    }
}
