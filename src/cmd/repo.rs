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
        RepoCommand::List {
            owner,
            visibility,
            fork,
            language,
            limit,
            json,
            hostname,
        } => list(
            owner.as_deref(),
            &visibility,
            &fork,
            language.as_deref(),
            limit,
            json,
            hostname.as_deref(),
        ),
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

/// Execute `gor repo list`.
///
/// Lists repositories for the authenticated user or a specified owner.
/// Supports filtering by visibility, fork status, and language.
/// Supports table output, JSON output, and pagination.
///
/// # Errors
///
/// Returns an error if the API request fails.
fn list(
    owner: Option<&str>,
    visibility: &str,
    fork: &str,
    language: Option<&str>,
    limit: u32,
    json: Option<Vec<String>>,
    hostname: Option<&str>,
) -> anyhow::Result<()> {
    let host = hostname.unwrap_or("github.com");
    let client = Client::new(host).context("failed to create HTTP client")?;

    // Build the API path based on whether an owner is specified
    let path = match owner {
        Some(owner_name) => {
            // Try users endpoint first; if 404, try orgs endpoint
            let user_path = format!("/users/{owner_name}/repos");
            let response = client
                .get(&user_path)
                .context("failed to fetch repositories")?;
            if response.status() == reqwest::StatusCode::NOT_FOUND {
                format!("/orgs/{owner_name}/repos")
            } else if !response.status().is_success() {
                let status = response.status();
                anyhow::bail!("failed to fetch repositories for '{owner_name}': HTTP {status}");
            } else {
                // We got a valid response — process it inline
                let repos: Vec<serde_json::Value> = response
                    .json()
                    .context("failed to parse repository response")?;
                let filtered = filter_repos(repos, visibility, fork, language, limit);
                output_repos(&filtered, json);
                return Ok(());
            }
        }
        None => "/user/repos".to_string(),
    };

    // Build query parameters
    let mut query_params = vec![
        ("per_page", limit.min(100).to_string()),
        ("type", "all".to_string()),
    ];

    // Add visibility filter
    if visibility != "all" {
        query_params.push(("visibility", visibility.to_string()));
    }

    // Add sort by updated
    query_params.push(("sort", "updated".to_string()));

    let query_string = query_params
        .iter()
        .map(|(k, v)| format!("{k}={v}"))
        .collect::<Vec<_>>()
        .join("&");

    let full_path = format!("{path}?{query_string}");

    let response = client
        .get(&full_path)
        .context("failed to fetch repositories")?;

    let status = response.status();
    if status == reqwest::StatusCode::NOT_FOUND {
        if let Some(owner_name) = owner {
            anyhow::bail!("user or organization '{owner_name}' not found");
        }
        anyhow::bail!("could not fetch repositories");
    }
    if status == reqwest::StatusCode::UNAUTHORIZED || status == reqwest::StatusCode::FORBIDDEN {
        anyhow::bail!("authentication required to list repositories");
    }
    if !status.is_success() {
        anyhow::bail!("failed to list repositories: HTTP {status}");
    }

    let repos: Vec<serde_json::Value> = response
        .json()
        .context("failed to parse repository response")?;

    let filtered = filter_repos(repos, visibility, fork, language, limit);
    output_repos(&filtered, json);
    Ok(())
}

/// Filter repositories by visibility, fork status, and language.
fn filter_repos(
    repos: Vec<serde_json::Value>,
    visibility: &str,
    fork: &str,
    language: Option<&str>,
    limit: u32,
) -> Vec<serde_json::Value> {
    let mut filtered = repos;

    // Client-side visibility filter (API may not support all cases)
    if visibility != "all" {
        filtered.retain(|repo| {
            let is_private = repo["private"].as_bool().unwrap_or(false);
            match visibility {
                "public" => !is_private,
                "private" => is_private,
                _ => true,
            }
        });
    }

    // Fork filter
    match fork {
        "exclude" => filtered.retain(|repo| !repo["fork"].as_bool().unwrap_or(false)),
        "only" => filtered.retain(|repo| repo["fork"].as_bool().unwrap_or(false)),
        _ => {} // "include" — no filter
    }

    // Language filter
    if let Some(lang) = language {
        filtered.retain(|repo| {
            repo["language"]
                .as_str()
                .is_some_and(|l| l.eq_ignore_ascii_case(lang))
        });
    }

    // Apply limit
    filtered.truncate(limit as usize);
    filtered
}

/// Output repositories as either JSON or a formatted table.
fn output_repos(repos: &Vec<serde_json::Value>, json: Option<Vec<String>>) {
    if let Some(fields) = json {
        let fields_ref: Option<&[String]> = if fields.is_empty() {
            None
        } else {
            Some(&fields)
        };
        print_json(repos, fields_ref);
    } else {
        print_repo_list_table(repos);
    }
}

/// Print a formatted repository list table.
///
/// Columns: NAME, DESCRIPTION, VISIBILITY, LANGUAGE, UPDATED
fn print_repo_list_table(repos: &[serde_json::Value]) {
    if repos.is_empty() {
        println!("No repositories found.");
        return;
    }

    // Column widths
    let name_width = 30;
    let desc_width = 50;
    let vis_width = 10;
    let lang_width = 14;
    let date_width = 16;

    // Header
    println!(
        "{:<name_width$}  {:<desc_width$}  {:<vis_width$}  {:<lang_width$}  {:<date_width$}",
        "NAME", "DESCRIPTION", "VISIBILITY", "LANGUAGE", "UPDATED",
    );

    for repo in repos {
        let name = repo["full_name"].as_str().unwrap_or("—");
        let description = repo["description"].as_str().unwrap_or("—");
        let is_private = repo["private"].as_bool().unwrap_or(false);
        let visibility = if is_private { "private" } else { "public" };
        let language = repo["language"].as_str().unwrap_or("—");
        let updated = repo["updated_at"]
            .as_str()
            .map_or_else(|| "—".to_string(), format_date);

        let name_truncated = crate::cmd::util::truncate(name, name_width);
        let desc_truncated = crate::cmd::util::truncate(description, desc_width);

        println!(
            "{name_truncated:<name_width$}  {desc_truncated:<desc_width$}  {visibility:<vis_width$}  {language:<lang_width$}  {updated:<date_width$}",
        );
    }
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

    #[test]
    fn print_repo_list_table_basic() {
        let repos = vec![json!({
            "full_name": "octocat/hello-world",
            "description": "My first repository",
            "private": false,
            "language": "Rust",
            "updated_at": "2024-01-15T10:30:00Z"
        })];
        print_repo_list_table(&repos);
    }

    #[test]
    fn print_repo_list_table_empty() {
        let repos: Vec<serde_json::Value> = vec![];
        print_repo_list_table(&repos);
    }

    #[test]
    fn print_repo_list_table_multiple() {
        let repos = vec![
            json!({
                "full_name": "alice/project-a",
                "description": "First project",
                "private": false,
                "language": "Rust",
                "updated_at": "2024-01-15T10:30:00Z"
            }),
            json!({
                "full_name": "bob/private-repo",
                "description": "A secret project with a very long description that should be truncated",
                "private": true,
                "language": "Python",
                "updated_at": "2024-02-20T12:00:00Z"
            }),
        ];
        print_repo_list_table(&repos);
    }

    #[test]
    fn print_repo_list_table_null_fields() {
        let repos = vec![json!({
            "full_name": "test/repo",
            "description": null,
            "private": false,
            "language": null,
            "updated_at": null
        })];
        print_repo_list_table(&repos);
    }

    #[test]
    fn filter_repos_by_visibility() {
        let repos = vec![
            json!({"full_name": "a/public", "private": false, "fork": false, "language": "Rust"}),
            json!({"full_name": "b/private", "private": true, "fork": false, "language": "Rust"}),
        ];
        let filtered = filter_repos(repos, "public", "include", None, 30);
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0]["full_name"], "a/public");
    }

    #[test]
    fn filter_repos_by_fork_exclude() {
        let repos = vec![
            json!({"full_name": "a/original", "private": false, "fork": false, "language": "Rust"}),
            json!({"full_name": "b/forked", "private": false, "fork": true, "language": "Rust"}),
        ];
        let filtered = filter_repos(repos, "all", "exclude", None, 30);
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0]["full_name"], "a/original");
    }

    #[test]
    fn filter_repos_by_fork_only() {
        let repos = vec![
            json!({"full_name": "a/original", "private": false, "fork": false, "language": "Rust"}),
            json!({"full_name": "b/forked", "private": false, "fork": true, "language": "Rust"}),
        ];
        let filtered = filter_repos(repos, "all", "only", None, 30);
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0]["full_name"], "b/forked");
    }

    #[test]
    fn filter_repos_by_language() {
        let repos = vec![
            json!({"full_name": "a/rust-repo", "private": false, "fork": false, "language": "Rust"}),
            json!({"full_name": "b/python-repo", "private": false, "fork": false, "language": "Python"}),
        ];
        let filtered = filter_repos(repos, "all", "include", Some("Rust"), 30);
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0]["full_name"], "a/rust-repo");
    }

    #[test]
    fn filter_repos_by_limit() {
        let repos: Vec<serde_json::Value> = (0..10)
            .map(|i| json!({"full_name": format!("owner/repo{i}"), "private": false, "fork": false, "language": "Rust"}))
            .collect();
        let filtered = filter_repos(repos, "all", "include", None, 5);
        assert_eq!(filtered.len(), 5);
    }
}
