//! Implementation of the `gor pr` subcommand.
//!
//! Provides pull request listing, viewing, and management commands.
//! Currently supports `gor pr list` for listing pull requests.

#![allow(clippy::print_stdout)]

use crate::cli::PrCommand;
use crate::client::Client;
use crate::output::{format_date, print_json};
use crate::repository::{detect_remote, parse_repo_spec};
use anyhow::Context;
use std::collections::BTreeMap;

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
        PrCommand::View {
            number,
            repo,
            web,
            comments,
            json,
            hostname,
        } => view(
            number,
            repo.as_deref(),
            web,
            comments,
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

/// Execute `gor pr view`.
///
/// Displays the full details of a single pull request, including title, body,
/// author, state, branch information, labels, review status, merge status, and
/// CI check status. Supports JSON output and opening the PR in a browser.
///
/// # Errors
///
/// Returns an error if the repository cannot be found, the PR does not exist,
/// or the API request fails.
#[allow(clippy::too_many_arguments)]
fn view(
    number: u64,
    repo: Option<&str>,
    web: bool,
    comments: bool,
    json: Option<Vec<String>>,
    hostname: Option<&str>,
) -> anyhow::Result<()> {
    // Resolve the repo spec
    let spec = match repo {
        Some(s) => parse_repo_spec(s).context("invalid repository spec")?,
        None => detect_remote().ok_or_else(|| {
            anyhow::anyhow!(
                "could not detect repository from current directory; specify OWNER/REPO"
            )
        })?,
    };

    let host = hostname.unwrap_or("github.com");

    // Handle --web flag: open in browser
    if web {
        let web_url = format!("https://{host}/{}/{}/pull/{number}", spec.owner, spec.repo);
        open_in_browser(&web_url);
        return Ok(());
    }

    let client = Client::new(host).context("failed to create HTTP client")?;

    // Fetch the PR details
    let path = format!("/repos/{}/{}/pulls/{number}", spec.owner, spec.repo);
    let response = client.get(&path).context("failed to fetch pull request")?;

    let status = response.status();
    if status == reqwest::StatusCode::NOT_FOUND {
        anyhow::bail!("pull request #{number} not found in '{spec}'");
    }
    if status == reqwest::StatusCode::UNAUTHORIZED || status == reqwest::StatusCode::FORBIDDEN {
        anyhow::bail!("authentication required to view pull request #{number}");
    }
    if !status.is_success() {
        anyhow::bail!("failed to view pull request #{number}: HTTP {status}");
    }

    let pr: serde_json::Value = response
        .json()
        .context("failed to parse pull request response")?;

    // Handle --json flag
    if let Some(fields) = json {
        let fields_ref: Option<&[String]> = if fields.is_empty() {
            None
        } else {
            Some(&fields)
        };
        print_json(&pr, fields_ref);
        return Ok(());
    }

    // Fetch reviews for review status
    let reviews_path = format!("/repos/{}/{}/pulls/{number}/reviews", spec.owner, spec.repo);
    let reviews: Vec<serde_json::Value> = client
        .get(&reviews_path)
        .ok()
        .and_then(|r| r.json().ok())
        .unwrap_or_default();

    // Fetch CI check status
    let head_sha = pr["head"]["sha"].as_str().unwrap_or("");
    let ci_status = if head_sha.is_empty() {
        None
    } else {
        let status_path = format!(
            "/repos/{}/{}/commits/{head_sha}/status",
            spec.owner, spec.repo
        );
        client.get(&status_path).ok().and_then(|r| r.json().ok())
    };

    // Fetch comments if --comments flag is set
    let comments_data = if comments {
        let comments_path = format!(
            "/repos/{}/{}/issues/{number}/comments",
            spec.owner, spec.repo
        );
        client
            .get(&comments_path)
            .ok()
            .and_then(|r| r.json().ok())
            .unwrap_or_default()
    } else {
        Vec::<serde_json::Value>::new()
    };

    // Print formatted view
    print_pr_view(&pr, &reviews, ci_status.as_ref(), &comments_data);
    Ok(())
}

/// Print a formatted pull request detail view.
///
/// Displays title, metadata, body, review status, merge status, CI checks,
/// and optionally comments.
fn print_pr_view(
    pr: &serde_json::Value,
    reviews: &[serde_json::Value],
    ci_status: Option<&serde_json::Value>,
    comments: &[serde_json::Value],
) {
    // Title
    let title = pr["title"].as_str().unwrap_or("(no title)");
    println!("{title}");
    let separator_len = title.len().min(80);
    println!("{}", "─".repeat(separator_len));
    println!();

    // Metadata
    let state = pr["state"].as_str().unwrap_or("unknown");
    let display_state = if pr["merged_at"].as_str().is_some() {
        "merged"
    } else {
        state
    };
    let author = pr["user"]["login"].as_str().unwrap_or("unknown");
    let created = pr["created_at"]
        .as_str()
        .map_or_else(|| "—".to_string(), format_date);
    let updated = pr["updated_at"]
        .as_str()
        .map_or_else(|| "—".to_string(), format_date);
    let base_branch = pr["base"]["ref"].as_str().unwrap_or("?");
    let head_branch = pr["head"]["ref"].as_str().unwrap_or("?");

    let labels_str = pr["labels"]
        .as_array()
        .map(|arr| {
            arr.iter()
                .filter_map(|l| l["name"].as_str())
                .collect::<Vec<_>>()
                .join(", ")
        })
        .unwrap_or_default();

    println!("State:  {display_state}");
    println!("Author: {author}");
    println!("Created: {created}");
    println!("Updated: {updated}");
    println!("Branches: {base_branch} ← {head_branch}");
    if !labels_str.is_empty() {
        println!("Labels: {labels_str}");
    }
    println!();

    // Body
    let body = pr["body"].as_str().unwrap_or("");
    if !body.is_empty() {
        println!("{body}");
        println!();
    }

    // Review status
    print_review_status(reviews);

    // Merge status
    print_merge_status(pr);

    // CI checks
    print_ci_status(ci_status);

    // Comments
    if !comments.is_empty() {
        println!("── Comments ──");
        println!();
        for comment in comments {
            let comment_author = comment["user"]["login"].as_str().unwrap_or("unknown");
            let comment_date = comment["created_at"]
                .as_str()
                .map_or_else(|| "—".to_string(), format_date);
            let comment_body = comment["body"].as_str().unwrap_or("");
            println!("{comment_author} commented on {comment_date}");
            println!();
            println!("{comment_body}");
            println!();
        }
    }
}

/// Print the review status section.
fn print_review_status(reviews: &[serde_json::Value]) {
    // Aggregate the latest review state per reviewer
    // Reviews are ordered oldest-first; we want the latest per user
    let mut latest_state: BTreeMap<&str, &str> = BTreeMap::new();
    for review in reviews {
        let user = review["user"]["login"].as_str();
        let state_val = review["state"].as_str();
        if let (Some(u), Some(s)) = (user, state_val) {
            if s != "COMMENTED" && s != "DISMISSED" {
                // APPROVED or CHANGES_REQUESTED override previous
                latest_state.insert(u, s);
            } else if s == "COMMENTED" && !latest_state.contains_key(u) {
                // Only set COMMENTED if no approval/change request yet
                latest_state.insert(u, s);
            }
        }
    }

    if latest_state.is_empty() {
        return;
    }

    println!("── Review Status ──");

    let mut approved: Vec<&str> = Vec::new();
    let mut changes_requested: Vec<&str> = Vec::new();
    let mut commented: Vec<&str> = Vec::new();

    for (user, state_val) in &latest_state {
        match *state_val {
            "APPROVED" => approved.push(user),
            "CHANGES_REQUESTED" => changes_requested.push(user),
            _ => commented.push(user),
        }
    }

    if !approved.is_empty() {
        println!("Approved by: {}", approved.join(", "));
    }
    if !changes_requested.is_empty() {
        println!("Changes requested by: {}", changes_requested.join(", "));
    }
    if !commented.is_empty() {
        println!("Commented by: {}", commented.join(", "));
    }

    println!();
}

/// Print the merge status section.
fn print_merge_status(pr: &serde_json::Value) {
    println!("── Merge Status ──");

    let mergeable = pr["mergeable"].as_bool();
    let mergeable_status = match mergeable {
        Some(true) => "yes",
        Some(false) => "no (conflicts)",
        None => "unknown (checking)",
    };
    println!("Mergeable: {mergeable_status}");

    if let Some(merged_at) = pr["merged_at"].as_str() {
        let merged_date = format_date(merged_at);
        let merged_by = pr["merged_by"]["login"].as_str().unwrap_or("unknown");
        println!("Merged: yes ({merged_date}) by {merged_by}");
    } else {
        println!("Merged: no");
    }

    println!();
}

/// Print the CI check status section.
fn print_ci_status(ci_status: Option<&serde_json::Value>) {
    let statuses = ci_status
        .and_then(|s| s["statuses"].as_array())
        .cloned()
        .unwrap_or_default();

    if statuses.is_empty() {
        return;
    }

    println!("── CI Checks ──");

    for check in &statuses {
        let name = check["context"].as_str().unwrap_or("?");
        let state_val = check["state"].as_str().unwrap_or("unknown");
        let (icon, display_state) = match state_val {
            "success" => ("✓", "success"),
            "failure" => ("✗", "failure"),
            "pending" => ("○", "pending"),
            _ => ("○", state_val),
        };
        println!("  {icon} {name} ({display_state})");
    }

    println!();
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

    #[test]
    fn print_pr_view_basic() {
        let pr = json!({
            "number": 42,
            "title": "Fix authentication bug",
            "state": "open",
            "merged_at": null,
            "user": { "login": "octocat" },
            "created_at": "2024-01-15T10:30:00Z",
            "updated_at": "2024-01-16T12:00:00Z",
            "body": "This PR fixes the authentication bug.",
            "base": { "ref": "main" },
            "head": { "ref": "fix-auth", "sha": "abc123" },
            "labels": [
                { "name": "bug" },
                { "name": "security" }
            ],
            "mergeable": true,
            "merged_by": null
        });
        let reviews: Vec<serde_json::Value> = vec![];
        let ci_status: Option<serde_json::Value> = None;
        let comments: Vec<serde_json::Value> = vec![];
        // Should not panic
        print_pr_view(&pr, &reviews, ci_status.as_ref(), &comments);
    }

    #[test]
    fn print_pr_view_merged() {
        let pr = json!({
            "number": 100,
            "title": "Add new feature",
            "state": "closed",
            "merged_at": "2024-01-15T10:30:00Z",
            "user": { "login": "dev-user" },
            "created_at": "2024-01-10T08:00:00Z",
            "updated_at": "2024-01-15T10:30:00Z",
            "body": "This adds a new feature.",
            "base": { "ref": "main" },
            "head": { "ref": "feature-branch", "sha": "def456" },
            "labels": [],
            "mergeable": null,
            "merged_by": { "login": "admin" }
        });
        let reviews: Vec<serde_json::Value> = vec![];
        let ci_status: Option<serde_json::Value> = None;
        let comments: Vec<serde_json::Value> = vec![];
        // Should not panic
        print_pr_view(&pr, &reviews, ci_status.as_ref(), &comments);
    }

    #[test]
    fn print_pr_view_with_reviews() {
        let pr = json!({
            "number": 42,
            "title": "Fix bug",
            "state": "open",
            "merged_at": null,
            "user": { "login": "octocat" },
            "created_at": "2024-01-15T10:30:00Z",
            "updated_at": "2024-01-16T12:00:00Z",
            "body": "Fixes a bug.",
            "base": { "ref": "main" },
            "head": { "ref": "fix-bug", "sha": "abc123" },
            "labels": [],
            "mergeable": true,
            "merged_by": null
        });
        let reviews = vec![
            json!({
                "user": { "login": "reviewer1" },
                "state": "APPROVED"
            }),
            json!({
                "user": { "login": "reviewer2" },
                "state": "CHANGES_REQUESTED"
            }),
            json!({
                "user": { "login": "reviewer3" },
                "state": "COMMENTED"
            }),
        ];
        let ci_status: Option<serde_json::Value> = None;
        let comments: Vec<serde_json::Value> = vec![];
        // Should not panic
        print_pr_view(&pr, &reviews, ci_status.as_ref(), &comments);
    }

    #[test]
    fn print_pr_view_with_ci() {
        let pr = json!({
            "number": 42,
            "title": "Fix bug",
            "state": "open",
            "merged_at": null,
            "user": { "login": "octocat" },
            "created_at": "2024-01-15T10:30:00Z",
            "updated_at": "2024-01-16T12:00:00Z",
            "body": "Fixes a bug.",
            "base": { "ref": "main" },
            "head": { "ref": "fix-bug", "sha": "abc123" },
            "labels": [],
            "mergeable": true,
            "merged_by": null
        });
        let reviews: Vec<serde_json::Value> = vec![];
        let ci_status = Some(json!({
            "statuses": [
                { "context": "CI / test", "state": "success" },
                { "context": "CI / lint", "state": "failure" },
                { "context": "CI / build", "state": "pending" }
            ]
        }));
        let comments: Vec<serde_json::Value> = vec![];
        // Should not panic
        print_pr_view(&pr, &reviews, ci_status.as_ref(), &comments);
    }

    #[test]
    fn print_pr_view_with_comments() {
        let pr = json!({
            "number": 42,
            "title": "Fix bug",
            "state": "open",
            "merged_at": null,
            "user": { "login": "octocat" },
            "created_at": "2024-01-15T10:30:00Z",
            "updated_at": "2024-01-16T12:00:00Z",
            "body": "Fixes a bug.",
            "base": { "ref": "main" },
            "head": { "ref": "fix-bug", "sha": "abc123" },
            "labels": [],
            "mergeable": true,
            "merged_by": null
        });
        let reviews: Vec<serde_json::Value> = vec![];
        let ci_status: Option<serde_json::Value> = None;
        let comments = vec![
            json!({
                "user": { "login": "reviewer1" },
                "created_at": "2024-01-16T14:00:00Z",
                "body": "Looks good to me!"
            }),
            json!({
                "user": { "login": "octocat" },
                "created_at": "2024-01-16T15:00:00Z",
                "body": "Thanks for the review!"
            }),
        ];
        // Should not panic
        print_pr_view(&pr, &reviews, ci_status.as_ref(), &comments);
    }

    #[test]
    fn print_pr_view_null_fields() {
        let pr = json!({
            "number": 99,
            "title": null,
            "state": null,
            "merged_at": null,
            "user": null,
            "created_at": null,
            "updated_at": null,
            "body": null,
            "base": null,
            "head": null,
            "labels": null,
            "mergeable": null,
            "merged_by": null
        });
        let reviews: Vec<serde_json::Value> = vec![];
        let ci_status: Option<serde_json::Value> = None;
        let comments: Vec<serde_json::Value> = vec![];
        // Should not panic with null fields
        print_pr_view(&pr, &reviews, ci_status.as_ref(), &comments);
    }

    #[test]
    fn print_review_status_empty() {
        let reviews: Vec<serde_json::Value> = vec![];
        // Should not panic
        print_review_status(&reviews);
    }

    #[test]
    fn print_review_status_with_reviews() {
        let reviews = vec![
            json!({
                "user": { "login": "alice" },
                "state": "APPROVED"
            }),
            json!({
                "user": { "login": "bob" },
                "state": "CHANGES_REQUESTED"
            }),
            json!({
                "user": { "login": "carol" },
                "state": "COMMENTED"
            }),
        ];
        // Should not panic
        print_review_status(&reviews);
    }

    #[test]
    fn print_merge_status_mergeable() {
        let pr = json!({
            "mergeable": true,
            "merged_at": null,
            "merged_by": null
        });
        // Should not panic
        print_merge_status(&pr);
    }

    #[test]
    fn print_merge_status_conflicts() {
        let pr = json!({
            "mergeable": false,
            "merged_at": null,
            "merged_by": null
        });
        // Should not panic
        print_merge_status(&pr);
    }

    #[test]
    fn print_merge_status_merged() {
        let pr = json!({
            "mergeable": null,
            "merged_at": "2024-01-15T10:30:00Z",
            "merged_by": { "login": "admin" }
        });
        // Should not panic
        print_merge_status(&pr);
    }

    #[test]
    fn print_ci_status_empty() {
        let ci_status: Option<serde_json::Value> = None;
        // Should not panic
        print_ci_status(ci_status.as_ref());
    }

    #[test]
    fn print_ci_status_with_checks() {
        let ci_status = Some(json!({
            "statuses": [
                { "context": "CI / test", "state": "success" },
                { "context": "CI / lint", "state": "failure" },
                { "context": "CI / build", "state": "pending" }
            ]
        }));
        // Should not panic
        print_ci_status(ci_status.as_ref());
    }
}
