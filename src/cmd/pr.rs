//! Implementation of the `gor pr` subcommand.
//!
//! Provides pull request listing, viewing, and management commands.
//! Currently supports `gor pr list` for listing pull requests.

#![allow(clippy::print_stdout)]

use crate::cli::PrCommand;
use crate::client::Client;
use crate::output::print_json;
use crate::repository::{detect_remote, parse_repo_spec};
use anyhow::Context;

/// Run the `gor pr` subcommand.
///
/// # Errors
///
/// Returns an error if the command execution fails.
pub fn run(cmd: PrCommand) -> anyhow::Result<()> {
    match cmd {
        PrCommand::List {
            owner_repo,
            state,
            base,
            head,
            author,
            labels,
            assignee,
            limit,
            web,
            json,
            hostname,
        } => list(
            owner_repo,
            &state,
            base.as_deref(),
            head.as_deref(),
            author.as_deref(),
            &labels,
            assignee.as_deref(),
            limit,
            web,
            json,
            hostname.as_deref(),
        ),
    }
}

/// Execute `gor pr list`.
///
/// Lists pull requests for a repository with filtering by state, base branch,
/// head branch, author, labels, and assignee. Supports table output, JSON
/// output, and opening the PR list in a browser.
///
/// # Errors
///
/// Returns an error if the repository cannot be found or the API request fails.
#[allow(clippy::too_many_arguments)]
fn list(
    owner_repo: Option<String>,
    state: &str,
    base: Option<&str>,
    head: Option<&str>,
    author: Option<&str>,
    labels: &[String],
    assignee: Option<&str>,
    limit: u32,
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

    // Handle --web flag: open in browser
    if web {
        let web_url = format!("https://{host}/{}/{}/pulls", spec.owner, spec.repo);
        open_in_browser(&web_url);
        return Ok(());
    }

    let client = Client::new(host).context("failed to create HTTP client")?;

    // Build query parameters for the API call
    // The GitHub API doesn't support "merged" as a state value; we use "all"
    // and filter client-side for merged PRs.
    let needs_merged_filter = state == "merged";
    let api_state = if needs_merged_filter { "all" } else { state };

    let mut query_params = vec![
        ("state", api_state.to_string()),
        ("per_page", limit.min(100).to_string()),
    ];

    if let Some(b) = base {
        query_params.push(("base", (*b).to_string()));
    }
    if let Some(h) = head {
        query_params.push(("head", (*h).to_string()));
    }

    let query_string = query_params
        .iter()
        .map(|(k, v)| format!("{k}={v}"))
        .collect::<Vec<_>>()
        .join("&");

    let path = format!("/repos/{}/{}/pulls?{query_string}", spec.owner, spec.repo);

    let response = client.get(&path).context("failed to fetch pull requests")?;

    let status = response.status();
    if status == reqwest::StatusCode::NOT_FOUND {
        anyhow::bail!("repository '{spec}' not found");
    }
    if status == reqwest::StatusCode::UNAUTHORIZED || status == reqwest::StatusCode::FORBIDDEN {
        anyhow::bail!("authentication required to list pull requests for '{spec}'");
    }
    if !status.is_success() {
        anyhow::bail!("failed to list pull requests for '{spec}': HTTP {status}");
    }

    let mut prs: Vec<serde_json::Value> = response
        .json()
        .context("failed to parse pull request response")?;

    // Client-side filtering
    if needs_merged_filter {
        prs.retain(|pr| pr["merged_at"].as_str().is_some());
    }
    if let Some(a) = author {
        prs.retain(|pr| {
            pr["user"]["login"]
                .as_str()
                .is_some_and(|login| login.eq_ignore_ascii_case(a))
        });
    }
    if !labels.is_empty() {
        prs.retain(|pr| {
            let pr_labels: Vec<&str> = pr["labels"]
                .as_array()
                .map(|arr| arr.iter().filter_map(|l| l["name"].as_str()).collect())
                .unwrap_or_default();
            labels
                .iter()
                .all(|label| pr_labels.iter().any(|l| l.eq_ignore_ascii_case(label)))
        });
    }
    if let Some(a) = assignee {
        prs.retain(|pr| {
            pr["assignees"].as_array().is_some_and(|arr| {
                arr.iter().any(|assignee| {
                    assignee["login"]
                        .as_str()
                        .is_some_and(|login| login.eq_ignore_ascii_case(a))
                })
            })
        });
    }

    // Handle --json flag
    if let Some(fields) = json {
        let fields_ref: Option<&[String]> = if fields.is_empty() {
            None
        } else {
            Some(&fields)
        };
        print_json(&prs, fields_ref);
        return Ok(());
    }

    // Default: print formatted table
    print_pr_table(&prs);
    Ok(())
}

/// Print a formatted pull request list table.
///
/// Columns: NUMBER, TITLE, AUTHOR, HEAD BRANCH, LABELS, STATE
fn print_pr_table(prs: &[serde_json::Value]) {
    if prs.is_empty() {
        println!("No pull requests found.");
        return;
    }

    // Column widths
    let num_width = 8;
    let title_width = 50;
    let author_width = 14;
    let branch_width = 14;
    let labels_width = 14;
    let state_width = 8;

    // Header
    println!(
        "{:>num_width$}  {:<title_width$}  {:<author_width$}  {:<branch_width$}  {:<labels_width$}  {:<state_width$}",
        "NUMBER", "TITLE", "AUTHOR", "HEAD BRANCH", "LABELS", "STATE",
    );

    for pr in prs {
        let number = pr["number"]
            .as_u64()
            .map_or_else(|| "—".to_string(), |n| n.to_string());
        let title = pr["title"].as_str().unwrap_or("—");
        let author = pr["user"]["login"].as_str().unwrap_or("—");
        let head_branch = pr["head"]["ref"].as_str().unwrap_or("—");
        let state = pr["state"].as_str().unwrap_or("—");

        // Determine display state: if merged_at is set, show "merged"
        let display_state = if pr["merged_at"].as_str().is_some() {
            "merged"
        } else {
            state
        };

        let labels_str = pr["labels"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|l| l["name"].as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            })
            .unwrap_or_default();
        let labels_display = if labels_str.is_empty() {
            "—".to_string()
        } else {
            labels_str
        };

        let title_truncated = crate::cmd::util::truncate(title, title_width);
        let author_truncated = crate::cmd::util::truncate(author, author_width);
        let branch_truncated = crate::cmd::util::truncate(head_branch, branch_width);
        let labels_truncated = crate::cmd::util::truncate(&labels_display, labels_width);

        println!(
            "{number:>num_width$}  {title_truncated:<title_width$}  {author_truncated:<author_width$}  {branch_truncated:<branch_width$}  {labels_truncated:<labels_width$}  {display_state:<state_width$}",
        );
    }
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
    fn print_pr_table_basic() {
        let prs = vec![json!({
            "number": 42,
            "title": "Fix authentication bug in login flow",
            "state": "open",
            "merged_at": null,
            "user": { "login": "octocat" },
            "head": { "ref": "fix-auth" },
            "labels": [
                { "name": "bug" },
                { "name": "security" }
            ]
        })];
        // Should not panic
        print_pr_table(&prs);
    }

    #[test]
    fn print_pr_table_merged() {
        let prs = vec![json!({
            "number": 100,
            "title": "Add new feature",
            "state": "closed",
            "merged_at": "2024-01-15T10:30:00Z",
            "user": { "login": "dev-user" },
            "head": { "ref": "feature-branch" },
            "labels": []
        })];
        // Should not panic; merged PR should show "merged" state
        print_pr_table(&prs);
    }

    #[test]
    fn print_pr_table_empty() {
        let prs: Vec<serde_json::Value> = vec![];
        // Should not panic
        print_pr_table(&prs);
    }

    #[test]
    fn print_pr_table_multiple() {
        let prs = vec![
            json!({
                "number": 1,
                "title": "First PR",
                "state": "open",
                "merged_at": null,
                "user": { "login": "alice" },
                "head": { "ref": "feature-a" },
                "labels": [{"name": "enhancement"}]
            }),
            json!({
                "number": 2,
                "title": "Second PR with a very long title that should be truncated in the table output",
                "state": "open",
                "merged_at": null,
                "user": { "login": "bob" },
                "head": { "ref": "feature-b" },
                "labels": [{"name": "bug"}, {"name": "docs"}]
            }),
        ];
        // Should not panic
        print_pr_table(&prs);
    }

    #[test]
    fn print_pr_table_null_fields() {
        let prs = vec![json!({
            "number": 99,
            "title": null,
            "state": null,
            "merged_at": null,
            "user": null,
            "head": null,
            "labels": null
        })];
        // Should not panic with null fields
        print_pr_table(&prs);
    }

    #[test]
    fn open_in_browser_does_not_panic() {
        // Just verify it doesn't panic — actual browser opening is a no-op in tests
        open_in_browser("https://github.com/octocat/hello-world/pulls");
    }
}
