//! Implementation of the `gor repo` subcommand.
//!
//! Provides repository viewing, listing, and management commands.
//! Currently supports `gor repo view` for displaying repository metadata.

#![allow(clippy::print_stdout)]

use crate::cli::RepoCommand;
use crate::client::Client;
use crate::output::{format_count, format_date, print_json};
use crate::repository::{detect_remote, parse_repo_spec};
use anyhow::Context;

/// Run the `gor repo` subcommand.
///
/// # Errors
///
/// Returns an error if the command execution fails.
pub fn run(cmd: RepoCommand) -> anyhow::Result<()> {
    match cmd {
        RepoCommand::View {
            owner_repo,
            web,
            json,
            hostname,
        } => view(owner_repo, web, json, hostname.as_deref()),
    }
}

/// Execute `gor repo view`.
///
/// Displays repository metadata including description, stats, language,
/// license, and other details. Supports JSON output and browser opening.
///
/// # Errors
///
/// Returns an error if the repository cannot be found or the API request fails.
fn view(
    owner_repo: Option<String>,
    web: bool,
    json: Option<Vec<String>>,
    hostname: Option<&str>,
) -> anyhow::Result<()> {
    // Resolve the repo spec
    let spec = match owner_repo {
        Some(s) => parse_repo_spec(&s).context("invalid repository spec")?,
        None => detect_remote().ok_or_else(|| {
            anyhow::anyhow!(
                "could not detect repository from current directory; specify OWNER/REPO"
            )
        })?,
    };

    let host = hostname.unwrap_or("github.com");
    let client = Client::new(host).context("failed to create HTTP client")?;

    let path = format!("/repos/{}/{}", spec.owner, spec.repo);
    let response = client
        .get(&path)
        .context("failed to fetch repository data")?;

    let status = response.status();
    if status == reqwest::StatusCode::NOT_FOUND {
        anyhow::bail!("repository '{spec}' not found");
    }
    if status == reqwest::StatusCode::UNAUTHORIZED || status == reqwest::StatusCode::FORBIDDEN {
        anyhow::bail!("authentication required to view repository '{spec}'");
    }
    if !status.is_success() {
        anyhow::bail!("failed to fetch repository '{spec}': HTTP {status}");
    }

    let repo: serde_json::Value = response
        .json()
        .context("failed to parse repository response")?;

    // Handle --web flag: open in browser
    if web {
        if let Some(url) = repo["html_url"].as_str() {
            open_in_browser(url);
        }
        return Ok(());
    }

    // Handle --json flag
    if let Some(fields) = json {
        let fields_ref: Option<&[String]> = if fields.is_empty() {
            None
        } else {
            Some(&fields)
        };
        print_json(&repo, fields_ref);
        return Ok(());
    }

    // Default: print formatted table
    print_repo_table(&repo);
    Ok(())
}

/// Print a formatted repository information table.
fn print_repo_table(repo: &serde_json::Value) {
    let full_name = repo["full_name"].as_str().unwrap_or("—");
    let description = repo["description"].as_str().unwrap_or("—");
    let html_url = repo["html_url"].as_str().unwrap_or("—");
    let is_private = repo["private"].as_bool().unwrap_or(false);
    let visibility = if is_private { "private" } else { "public" };
    let stars = repo["stargazers_count"].as_u64().unwrap_or(0);
    let forks = repo["forks_count"].as_u64().unwrap_or(0);
    let issues = repo["open_issues_count"].as_u64().unwrap_or(0);
    let language = repo["language"].as_str().unwrap_or("—");
    let license_name = repo["license"]["spdx_id"]
        .as_str()
        .or_else(|| repo["license"]["name"].as_str())
        .unwrap_or("—");
    let default_branch = repo["default_branch"].as_str().unwrap_or("—");
    let pushed_at = repo["pushed_at"].as_str().unwrap_or("—");

    println!("name:        {full_name}");
    println!("description: {description}");
    println!("url:         {html_url}");
    println!("visibility:  {visibility}");
    println!("stars:       {}", format_count(stars));
    println!("forks:       {}", format_count(forks));
    println!("issues:      {}", format_count(issues));
    println!("language:    {language}");
    println!("license:     {license_name}");
    println!("default:     {default_branch}");
    println!("updated:     {}", format_date(pushed_at));
}

/// Open a URL in the default browser using the system's default handler.
fn open_in_browser(url: &str) {
    #[cfg(target_os = "linux")]
    {
        let _ = std::process::Command::new("xdg-open").arg(url).spawn();
    }
    #[cfg(target_os = "macos")]
    {
        let _ = std::process::Command::new("open").arg(url).spawn();
    }
    #[cfg(target_os = "windows")]
    {
        let _ = std::process::Command::new("cmd")
            .args(["/c", "start", url])
            .spawn();
    }
    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    {
        println!("Open {url} in your browser");
    }
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn print_repo_table_basic() {
        let repo = json!({
            "full_name": "octocat/hello-world",
            "description": "My first repository",
            "html_url": "https://github.com/octocat/hello-world",
            "private": false,
            "stargazers_count": 1234,
            "forks_count": 56,
            "open_issues_count": 12,
            "language": "Rust",
            "license": { "spdx_id": "MIT" },
            "default_branch": "main",
            "pushed_at": "2024-01-15T10:30:00Z"
        });
        // Should not panic
        print_repo_table(&repo);
    }

    #[test]
    fn print_repo_table_private() {
        let repo = json!({
            "full_name": "org/private-repo",
            "description": null,
            "html_url": "https://github.com/org/private-repo",
            "private": true,
            "stargazers_count": 0,
            "forks_count": 0,
            "open_issues_count": 0,
            "language": null,
            "license": null,
            "default_branch": "main",
            "pushed_at": "2024-01-15T10:30:00Z"
        });
        // Should not panic with null fields
        print_repo_table(&repo);
    }

    #[test]
    fn print_repo_table_no_license() {
        let repo = json!({
            "full_name": "user/test",
            "description": "A test repo",
            "html_url": "https://github.com/user/test",
            "private": false,
            "stargazers_count": 42,
            "forks_count": 7,
            "open_issues_count": 3,
            "language": "Python",
            "license": null,
            "default_branch": "master",
            "pushed_at": "2023-12-25T00:00:00Z"
        });
        // Should not panic with null license
        print_repo_table(&repo);
    }

    #[test]
    fn open_in_browser_does_not_panic() {
        // Just verify it doesn't panic — actual browser opening is a no-op in tests
        open_in_browser("https://github.com/octocat/hello-world");
    }
}
