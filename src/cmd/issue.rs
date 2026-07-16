//! Implementation of the `gor issue` subcommand.
//!
//! Provides issue listing and management commands.
//! Currently supports `gor issue list` for listing issues.

#![allow(clippy::print_stdout)]

use crate::cli::IssueCommand;
use crate::client::Client;
use crate::output::print_json;
use crate::repository::{detect_remote, parse_repo_spec};
use anyhow::Context;

/// Run the `gor issue` subcommand.
///
/// # Errors
///
/// Returns an error if the command execution fails.
pub fn run(cmd: IssueCommand) -> anyhow::Result<()> {
    match cmd {
        IssueCommand::List {
            owner_repo,
            state,
            labels,
            assignee,
            author,
            mention,
            milestone,
            limit,
            web,
            json,
            hostname,
        } => list(
            owner_repo,
            &state,
            &labels,
            assignee.as_deref(),
            author.as_deref(),
            mention.as_deref(),
            milestone.as_deref(),
            limit,
            web,
            json,
            hostname.as_deref(),
        ),
    }
}

/// Execute `gor issue list`.
///
/// Lists issues for a repository with filtering by state, labels, assignee,
/// author, mention, and milestone. Supports table output, JSON output, and
/// opening the issue list in a browser.
///
/// # Errors
///
/// Returns an error if the repository cannot be found or the API request fails.
#[allow(clippy::too_many_arguments)]
fn list(
    owner_repo: Option<String>,
    state: &str,
    labels: &[String],
    assignee: Option<&str>,
    author: Option<&str>,
    mention: Option<&str>,
    milestone: Option<&str>,
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
        let web_url = format!("https://{host}/{}/{}/issues", spec.owner, spec.repo);
        open_in_browser(&web_url);
        return Ok(());
    }

    let client = Client::new(host).context("failed to create HTTP client")?;

    // Build query parameters for the API call
    let mut query_params = vec![
        ("state", state.to_string()),
        ("per_page", limit.min(100).to_string()),
    ];

    if !labels.is_empty() {
        query_params.push(("labels", labels.join(",")));
    }
    if let Some(a) = assignee {
        query_params.push(("assignee", (*a).to_string()));
    }
    if let Some(a) = author {
        // GitHub Issues API uses "creator" for filtering by author
        query_params.push(("creator", (*a).to_string()));
    }
    if let Some(m) = mention {
        query_params.push(("mentioned", (*m).to_string()));
    }
    if let Some(m) = milestone {
        query_params.push(("milestone", (*m).to_string()));
    }

    let query_string = query_params
        .iter()
        .map(|(k, v)| format!("{k}={v}"))
        .collect::<Vec<_>>()
        .join("&");

    let path = format!("/repos/{}/{}/issues?{query_string}", spec.owner, spec.repo);

    let response = client.get(&path).context("failed to fetch issues")?;

    let status = response.status();
    if status == reqwest::StatusCode::NOT_FOUND {
        anyhow::bail!("repository '{spec}' not found");
    }
    if status == reqwest::StatusCode::UNAUTHORIZED || status == reqwest::StatusCode::FORBIDDEN {
        anyhow::bail!("authentication required to list issues for '{spec}'");
    }
    if !status.is_success() {
        anyhow::bail!("failed to list issues for '{spec}': HTTP {status}");
    }

    let mut issues: Vec<serde_json::Value> =
        response.json().context("failed to parse issue response")?;

    // Client-side filtering for fields the API may not fully support
    if let Some(a) = author {
        issues.retain(|issue| {
            issue["user"]["login"]
                .as_str()
                .is_some_and(|login| login.eq_ignore_ascii_case(a))
        });
    }
    if !labels.is_empty() {
        issues.retain(|issue| {
            let issue_labels: Vec<&str> = issue["labels"]
                .as_array()
                .map(|arr| arr.iter().filter_map(|l| l["name"].as_str()).collect())
                .unwrap_or_default();
            labels
                .iter()
                .all(|label| issue_labels.iter().any(|l| l.eq_ignore_ascii_case(label)))
        });
    }
    if let Some(a) = assignee {
        issues.retain(|issue| {
            issue["assignees"].as_array().is_some_and(|arr| {
                arr.iter().any(|assignee| {
                    assignee["login"]
                        .as_str()
                        .is_some_and(|login| login.eq_ignore_ascii_case(a))
                })
            })
        });
    }
    if let Some(m) = mention {
        issues.retain(|issue| {
            let body = issue["body"].as_str().unwrap_or("");
            body.to_lowercase().contains(&m.to_lowercase())
        });
    }
    if let Some(m) = milestone {
        issues.retain(|issue| {
            issue["milestone"]["title"]
                .as_str()
                .is_some_and(|title| title.eq_ignore_ascii_case(m))
        });
    }

    // Handle --json flag
    if let Some(fields) = json {
        let fields_ref: Option<&[String]> = if fields.is_empty() {
            None
        } else {
            Some(&fields)
        };
        print_json(&issues, fields_ref);
        return Ok(());
    }

    // Default: print formatted table
    print_issue_table(&issues);
    Ok(())
}

/// Print a formatted issue list table.
///
/// Columns: NUMBER, TITLE, AUTHOR, LABELS, STATE
fn print_issue_table(issues: &[serde_json::Value]) {
    if issues.is_empty() {
        println!("No issues found.");
        return;
    }

    // Column widths
    let num_width = 8;
    let title_width = 50;
    let author_width = 14;
    let labels_width = 14;
    let state_width = 8;

    // Header
    println!(
        "{:>num_width$}  {:<title_width$}  {:<author_width$}  {:<labels_width$}  {:<state_width$}",
        "NUMBER", "TITLE", "AUTHOR", "LABELS", "STATE",
    );

    for issue in issues {
        let number = issue["number"]
            .as_u64()
            .map_or_else(|| "—".to_string(), |n| n.to_string());
        let title = issue["title"].as_str().unwrap_or("—");
        let author = issue["user"]["login"].as_str().unwrap_or("—");
        let state = issue["state"].as_str().unwrap_or("—");

        let labels_str = issue["labels"]
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
        let labels_truncated = crate::cmd::util::truncate(&labels_display, labels_width);

        println!(
            "{number:>num_width$}  {title_truncated:<title_width$}  {author_truncated:<author_width$}  {labels_truncated:<labels_width$}  {state:<state_width$}",
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
    fn print_issue_table_basic() {
        let issues = vec![json!({
            "number": 42,
            "title": "Fix authentication bug in login flow",
            "state": "open",
            "user": { "login": "octocat" },
            "labels": [
                { "name": "bug" },
                { "name": "security" }
            ]
        })];
        // Should not panic
        print_issue_table(&issues);
    }

    #[test]
    fn print_issue_table_closed() {
        let issues = vec![json!({
            "number": 100,
            "title": "Add new feature",
            "state": "closed",
            "user": { "login": "dev-user" },
            "labels": []
        })];
        // Should not panic; closed issue should show "closed" state
        print_issue_table(&issues);
    }

    #[test]
    fn print_issue_table_empty() {
        let issues: Vec<serde_json::Value> = vec![];
        // Should not panic
        print_issue_table(&issues);
    }

    #[test]
    fn print_issue_table_multiple() {
        let issues = vec![
            json!({
                "number": 1,
                "title": "First issue",
                "state": "open",
                "user": { "login": "alice" },
                "labels": [{"name": "enhancement"}]
            }),
            json!({
                "number": 2,
                "title": "Second issue with a very long title that should be truncated in the table output",
                "state": "closed",
                "user": { "login": "bob" },
                "labels": [{"name": "bug"}, {"name": "docs"}]
            }),
        ];
        // Should not panic
        print_issue_table(&issues);
    }

    #[test]
    fn print_issue_table_null_fields() {
        let issues = vec![json!({
            "number": 99,
            "title": null,
            "state": null,
            "user": null,
            "labels": null
        })];
        // Should not panic with null fields
        print_issue_table(&issues);
    }

    #[test]
    fn open_in_browser_does_not_panic() {
        // Just verify it doesn't panic — actual browser opening is a no-op in tests
        open_in_browser("https://github.com/octocat/hello-world/issues");
    }
}
