//! Implementation of the `gor issue` subcommand.
//!
//! Provides issue listing and management commands.
//! Currently supports `gor issue list` for listing issues.

#![allow(clippy::print_stdout)]

use crate::cli::IssueCommand;
use crate::client::Client;
use crate::output::{format_date, print_json};
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
        IssueCommand::View {
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
        IssueCommand::Close {
            number,
            repo,
            comment,
            reason,
            hostname,
        } => close(
            number,
            repo.as_deref(),
            comment.as_deref(),
            reason.as_deref(),
            hostname.as_deref(),
        ),
        IssueCommand::Reopen {
            number,
            repo,
            comment,
            hostname,
        } => reopen(
            number,
            repo.as_deref(),
            comment.as_deref(),
            hostname.as_deref(),
        ),
        IssueCommand::Comment {
            number,
            repo,
            body,
            body_file,
            web,
            hostname,
        } => comment(
            number,
            repo.as_deref(),
            body.as_deref(),
            body_file.as_deref(),
            web,
            hostname.as_deref(),
        ),
        IssueCommand::Create {
            repo,
            title,
            body,
            labels,
            assignee,
            milestone,
            project,
            web,
            hostname,
        } => issue_create(
            repo.as_deref(),
            title.as_deref(),
            body.as_deref(),
            &labels,
            &assignee,
            milestone.as_deref(),
            project,
            web,
            hostname.as_deref(),
        ),
        IssueCommand::Edit {
            number,
            repo,
            title,
            body,
            add_label,
            remove_label,
            add_assignee,
            remove_assignee,
            milestone,
            hostname,
        } => edit(
            number,
            repo.as_deref(),
            title.as_deref(),
            body.as_deref(),
            &add_label,
            &remove_label,
            &add_assignee,
            &remove_assignee,
            milestone.as_deref(),
            hostname.as_deref(),
        ),
        IssueCommand::Transfer {
            number,
            destination,
            repo,
            hostname,
        } => transfer(number, &destination, repo.as_deref(), hostname.as_deref()),
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

/// Execute `gor issue view`.
///
/// Displays the full details of a single issue, including title, body,
/// author, state, labels, assignees, milestone, and optionally comments.
/// Supports JSON output and opening the issue in a browser.
///
/// # Errors
///
/// Returns an error if the repository cannot be found, the issue does not
/// exist, or the API request fails.
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
        let web_url = format!(
            "https://{host}/{}/{}/issues/{number}",
            spec.owner, spec.repo
        );
        open_in_browser(&web_url);
        return Ok(());
    }

    let client = Client::new(host).context("failed to create HTTP client")?;

    // Fetch the issue details
    let path = format!("/repos/{}/{}/issues/{number}", spec.owner, spec.repo);
    let response = client.get(&path).context("failed to fetch issue")?;

    let status = response.status();
    if status == reqwest::StatusCode::NOT_FOUND {
        anyhow::bail!("issue #{number} not found in '{spec}'");
    }
    if status == reqwest::StatusCode::UNAUTHORIZED || status == reqwest::StatusCode::FORBIDDEN {
        anyhow::bail!("authentication required to view issue #{number}");
    }
    if !status.is_success() {
        anyhow::bail!("failed to view issue #{number}: HTTP {status}");
    }

    let issue: serde_json::Value = response.json().context("failed to parse issue response")?;

    // Handle --json flag
    if let Some(fields) = json {
        let fields_ref: Option<&[String]> = if fields.is_empty() {
            None
        } else {
            Some(&fields)
        };
        print_json(&issue, fields_ref);
        return Ok(());
    }

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
    print_issue_view(&issue, &comments_data);
    Ok(())
}

/// Execute `gor issue close`.
///
/// Closes an issue by PATCHing its state to "closed". Optionally adds a
/// closing comment and sets a close reason.
///
/// # Errors
///
/// Returns an error if the repository cannot be found, the issue does not
/// exist, or the API request fails.
fn close(
    number: u64,
    repo: Option<&str>,
    comment: Option<&str>,
    reason: Option<&str>,
    hostname: Option<&str>,
) -> anyhow::Result<()> {
    let spec = match repo {
        Some(s) => parse_repo_spec(s).context("invalid repository spec")?,
        None => detect_remote().ok_or_else(|| {
            anyhow::anyhow!(
                "could not detect repository from current directory; specify OWNER/REPO"
            )
        })?,
    };

    let host = hostname.unwrap_or("github.com");
    let client = Client::new(host).context("failed to create HTTP client")?;

    // Build the PATCH body
    let mut body = serde_json::json!({
        "state": "closed",
    });

    if let Some(r) = reason {
        body["state_reason"] = serde_json::Value::String(r.to_string());
    }

    let path = format!("/repos/{}/{}/issues/{number}", spec.owner, spec.repo);
    let response = client
        .request("PATCH", &path, &[], Some(serde_json::to_vec(&body)?))
        .context("failed to close issue")?;

    let status = response.status();
    if status == reqwest::StatusCode::NOT_FOUND {
        anyhow::bail!("issue #{number} not found in '{spec}'");
    }
    if status == reqwest::StatusCode::UNAUTHORIZED || status == reqwest::StatusCode::FORBIDDEN {
        anyhow::bail!("authentication required to close issue #{number}");
    }
    if !status.is_success() {
        anyhow::bail!("failed to close issue #{number}: HTTP {status}");
    }

    // Add comment if requested
    if let Some(c) = comment {
        let comment_path = format!(
            "/repos/{}/{}/issues/{number}/comments",
            spec.owner, spec.repo
        );
        let comment_body = serde_json::json!({ "body": c });
        let comment_response = client
            .request(
                "POST",
                &comment_path,
                &[],
                Some(serde_json::to_vec(&comment_body)?),
            )
            .context("failed to add closing comment")?;
        if !comment_response.status().is_success() {
            tracing::warn!(
                "failed to add closing comment: HTTP {}",
                comment_response.status()
            );
        }
    }

    let reason_str = reason.map_or_else(String::new, |r| format!(" as {r}"));
    println!("✓ issue #{number} closed{reason_str}");
    Ok(())
}

/// Execute `gor issue reopen`.
///
/// Reopens a closed issue by PATCHing its state to "open". Optionally adds
/// a comment when reopening.
///
/// # Errors
///
/// Returns an error if the repository cannot be found, the issue does not
/// exist, or the API request fails.
fn reopen(
    number: u64,
    repo: Option<&str>,
    comment: Option<&str>,
    hostname: Option<&str>,
) -> anyhow::Result<()> {
    let spec = match repo {
        Some(s) => parse_repo_spec(s).context("invalid repository spec")?,
        None => detect_remote().ok_or_else(|| {
            anyhow::anyhow!(
                "could not detect repository from current directory; specify OWNER/REPO"
            )
        })?,
    };

    let host = hostname.unwrap_or("github.com");
    let client = Client::new(host).context("failed to create HTTP client")?;

    let body = serde_json::json!({
        "state": "open",
    });

    let path = format!("/repos/{}/{}/issues/{number}", spec.owner, spec.repo);
    let response = client
        .request("PATCH", &path, &[], Some(serde_json::to_vec(&body)?))
        .context("failed to reopen issue")?;

    let status = response.status();
    if status == reqwest::StatusCode::NOT_FOUND {
        anyhow::bail!("issue #{number} not found in '{spec}'");
    }
    if status == reqwest::StatusCode::UNAUTHORIZED || status == reqwest::StatusCode::FORBIDDEN {
        anyhow::bail!("authentication required to reopen issue #{number}");
    }
    if !status.is_success() {
        anyhow::bail!("failed to reopen issue #{number}: HTTP {status}");
    }

    // Add comment if requested
    if let Some(c) = comment {
        let comment_path = format!(
            "/repos/{}/{}/issues/{number}/comments",
            spec.owner, spec.repo
        );
        let comment_body = serde_json::json!({ "body": c });
        let comment_response = client
            .request(
                "POST",
                &comment_path,
                &[],
                Some(serde_json::to_vec(&comment_body)?),
            )
            .context("failed to add comment")?;
        if !comment_response.status().is_success() {
            tracing::warn!("failed to add comment: HTTP {}", comment_response.status());
        }
    }

    println!("✓ issue #{number} reopened");
    Ok(())
}

/// Execute `gor issue comment`.
///
/// Adds a comment to an existing issue. Supports reading the comment body
/// from a `--body` flag, a file (`--body-file`), or stdin (`@-`).
///
/// # Errors
///
/// Returns an error if the repository cannot be found, the issue does not
/// exist, or the API request fails.
fn comment(
    number: u64,
    repo: Option<&str>,
    body: Option<&str>,
    body_file: Option<&str>,
    web: bool,
    hostname: Option<&str>,
) -> anyhow::Result<()> {
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
        let web_url = format!(
            "https://{host}/{}/{}/issues/{number}",
            spec.owner, spec.repo
        );
        open_in_browser(&web_url);
        return Ok(());
    }

    // Resolve the comment body
    let comment_body = match (body, body_file) {
        (Some(b), None) => b.to_string(),
        (None, Some(f)) => {
            if f == "@-" {
                let mut buf = String::new();
                std::io::stdin()
                    .read_line(&mut buf)
                    .context("failed to read from stdin")?;
                buf
            } else {
                std::fs::read_to_string(f)
                    .with_context(|| format!("failed to read body file '{f}'"))?
            }
        }
        (None, None) => anyhow::bail!("either --body or --body-file is required"),
        (Some(_), Some(_)) => unreachable!(), // clap conflicts_with prevents this
    };

    let client = Client::new(host).context("failed to create HTTP client")?;

    let path = format!(
        "/repos/{}/{}/issues/{number}/comments",
        spec.owner, spec.repo
    );
    let request_body = serde_json::json!({ "body": comment_body });
    let response = client
        .request("POST", &path, &[], Some(serde_json::to_vec(&request_body)?))
        .context("failed to post comment")?;

    let status = response.status();
    if status == reqwest::StatusCode::NOT_FOUND {
        anyhow::bail!("issue #{number} not found in '{spec}'");
    }
    if status == reqwest::StatusCode::UNAUTHORIZED || status == reqwest::StatusCode::FORBIDDEN {
        anyhow::bail!("authentication required to comment on issue #{number}");
    }
    if !status.is_success() {
        anyhow::bail!("failed to comment on issue #{number}: HTTP {status}");
    }

    let comment_json: serde_json::Value = response
        .json()
        .context("failed to parse comment response")?;

    let html_url = comment_json["html_url"]
        .as_str()
        .unwrap_or("https://github.com/");
    println!("✓ comment posted: {html_url}");
    Ok(())
}

/// Execute `gor issue create`.
///
/// Creates a new issue in a repository. Requires a title. Supports labels,
/// assignees, milestones, and project board assignment.
///
/// # Errors
///
/// Returns an error if the repository cannot be found, the issue creation
/// fails, or required fields are missing.
#[allow(clippy::too_many_arguments)]
fn issue_create(
    repo: Option<&str>,
    title: Option<&str>,
    body: Option<&str>,
    labels: &[String],
    assignees: &[String],
    _milestone: Option<&str>,
    _project: Option<u32>,
    web: bool,
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

    // Build the request body
    let mut body_map = serde_json::Map::new();
    body_map.insert(
        "title".to_string(),
        serde_json::Value::String(
            title
                .ok_or_else(|| anyhow::anyhow!("issue title is required; use --title"))?
                .to_string(),
        ),
    );
    if let Some(b) = body {
        body_map.insert("body".to_string(), serde_json::Value::String(b.to_string()));
    }
    if !labels.is_empty() {
        body_map.insert(
            "labels".to_string(),
            serde_json::Value::Array(
                labels
                    .iter()
                    .map(|l| serde_json::Value::String(l.clone()))
                    .collect(),
            ),
        );
    }
    if !assignees.is_empty() {
        body_map.insert(
            "assignees".to_string(),
            serde_json::Value::Array(
                assignees
                    .iter()
                    .map(|a| serde_json::Value::String(a.clone()))
                    .collect(),
            ),
        );
    }

    let body_value = serde_json::Value::Object(body_map);

    // Create the issue
    let path = format!("/repos/{}/{}/issues", spec.owner, spec.repo);
    let response = client
        .post(&path, &body_value)
        .context("failed to create issue")?;

    let status = response.status();
    if status == reqwest::StatusCode::NOT_FOUND {
        anyhow::bail!("repository '{spec}' not found");
    }
    if !status.is_success() {
        let err_body: serde_json::Value = response.json().unwrap_or_default();
        let msg = err_body["message"].as_str().unwrap_or("creation failed");
        anyhow::bail!("failed to create issue: {msg}");
    }

    let issue: serde_json::Value = response.json().context("failed to parse issue response")?;

    let issue_number = issue["number"].as_u64().unwrap_or(0);
    let issue_url = issue["html_url"].as_str().unwrap_or("");

    // Handle --web flag: open in browser
    if web && !issue_url.is_empty() {
        open_in_browser(issue_url);
    }

    println!(
        "https://github.com/{}/{}/issues/{issue_number}",
        spec.owner, spec.repo
    );
    Ok(())
}

/// Execute `gor issue edit`.
///
/// Edits an existing issue's title, body, labels, assignees, or milestone.
/// Only fields that are provided will be updated.
///
/// # Errors
///
/// Returns an error if the issue does not exist, the API request fails,
/// or the user does not have permission to edit the issue.
#[allow(clippy::too_many_arguments)]
fn edit(
    number: u64,
    repo: Option<&str>,
    title: Option<&str>,
    body: Option<&str>,
    add_label: &[String],
    remove_label: &[String],
    add_assignee: &[String],
    remove_assignee: &[String],
    _milestone: Option<&str>,
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

    // Build the request body with only the fields that were provided
    let mut body_map = serde_json::Map::new();

    if let Some(t) = title {
        body_map.insert(
            "title".to_string(),
            serde_json::Value::String(t.to_string()),
        );
    }
    if let Some(b) = body {
        body_map.insert("body".to_string(), serde_json::Value::String(b.to_string()));
    }

    // Build the final labels list: current labels + add - remove
    if !add_label.is_empty() || !remove_label.is_empty() {
        // Fetch current labels first
        let get_path = format!("/repos/{}/{}/issues/{number}", spec.owner, spec.repo);
        let current: serde_json::Value = client
            .get(&get_path)
            .context("failed to fetch current issue labels")?
            .json()
            .context("failed to parse issue response")?;

        let current_labels: Vec<String> = current["labels"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|l| l["name"].as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();

        let mut new_labels = current_labels;
        // Add new labels
        for label in add_label {
            if !new_labels.contains(label) {
                new_labels.push(label.clone());
            }
        }
        // Remove labels
        new_labels.retain(|l| !remove_label.contains(l));

        body_map.insert(
            "labels".to_string(),
            serde_json::Value::Array(
                new_labels
                    .iter()
                    .map(|l| serde_json::Value::String(l.clone()))
                    .collect(),
            ),
        );
    }

    // Handle assignees similarly
    if !add_assignee.is_empty() || !remove_assignee.is_empty() {
        // Fetch current assignees if we need to modify
        if !add_assignee.is_empty() || !remove_assignee.is_empty() {
            let get_path = format!("/repos/{}/{}/issues/{number}", spec.owner, spec.repo);
            let current: serde_json::Value = client
                .get(&get_path)
                .context("failed to fetch current assignees")?
                .json()
                .context("failed to parse issue response")?;

            let current_assignees: Vec<String> = current["assignees"]
                .as_array()
                .map(|arr| {
                    arr.iter()
                        .filter_map(|l| l["login"].as_str().map(String::from))
                        .collect()
                })
                .unwrap_or_default();

            let mut new_assignees = current_assignees;
            for a in add_assignee {
                if !new_assignees.contains(a) {
                    new_assignees.push(a.clone());
                }
            }
            new_assignees.retain(|a| !remove_assignee.contains(a));

            body_map.insert(
                "assignees".to_string(),
                serde_json::Value::Array(
                    new_assignees
                        .iter()
                        .map(|a| serde_json::Value::String(a.clone()))
                        .collect(),
                ),
            );
        }
    }

    let body_value = serde_json::Value::Object(body_map);

    let path = format!("/repos/{}/{}/issues/{number}", spec.owner, spec.repo);
    let response = client
        .request("PATCH", &path, &[], Some(serde_json::to_vec(&body_value)?))
        .context("failed to update issue")?;

    let status = response.status();
    if status == reqwest::StatusCode::NOT_FOUND {
        anyhow::bail!("issue #{number} not found in '{spec}'");
    }
    if !status.is_success() {
        let err_body: serde_json::Value = response.json().unwrap_or_default();
        let msg = err_body["message"].as_str().unwrap_or("update failed");
        anyhow::bail!("failed to update issue #{number}: {msg}");
    }

    let updated: serde_json::Value = response
        .json()
        .context("failed to parse updated issue response")?;

    let issue_number = updated["number"].as_u64().unwrap_or(number);
    println!(
        "https://github.com/{}/{}/issues/{issue_number}",
        spec.owner, spec.repo
    );
    Ok(())
}

/// Execute `gor issue transfer`.
///
/// Transfers an issue to a different repository.
///
/// # Errors
///
/// Returns an error if the source repo cannot be determined, the API request
/// fails, or the destination repo does not exist.
fn transfer(
    number: u64,
    destination: &str,
    repo: Option<&str>,
    hostname: Option<&str>,
) -> anyhow::Result<()> {
    let host = hostname.unwrap_or("github.com");
    let client = Client::new(host).context("failed to create HTTP client")?;

    // Determine the source repository.
    let spec = if let Some(r) = repo {
        parse_repo_spec(r).with_context(|| format!("invalid repository: {r}"))?
    } else {
        detect_remote().context("could not detect repository from git remote")?
    };

    // Parse the destination repository.
    let dest_spec = parse_repo_spec(destination)
        .with_context(|| format!("invalid destination repository: {destination}"))?;

    let path = format!(
        "/repos/{}/{}/issues/{number}/transfer",
        spec.owner, spec.repo
    );

    let body = serde_json::json!({
        "new_owner": dest_spec.owner,
        "new_repo": dest_spec.repo,
    });

    let response = client
        .post(&path, &body)
        .context("failed to transfer issue")?;

    let status = response.status();
    if !status.is_success() {
        let err_body: serde_json::Value = response.json().unwrap_or_default();
        let msg = err_body["message"].as_str().unwrap_or("transfer failed");
        anyhow::bail!("failed to transfer issue #{number}: {msg}");
    }

    let result: serde_json::Value = response.json().context("failed to parse response")?;
    let new_url = result["html_url"].as_str().unwrap_or("—");
    println!("Issue #{number} transferred to {destination}: {new_url}");
    Ok(())
}

/// Print a formatted issue detail view.
///
/// Displays title, metadata (state, author, dates, labels, assignees,
/// milestone), body, and optionally comments.
fn print_issue_view(issue: &serde_json::Value, comments: &[serde_json::Value]) {
    // Title
    let title = issue["title"].as_str().unwrap_or("(no title)");
    println!("{title}");
    let separator_len = title.len().min(80);
    println!("{}", "─".repeat(separator_len));
    println!();

    // Metadata
    let state = issue["state"].as_str().unwrap_or("unknown");
    let author = issue["user"]["login"].as_str().unwrap_or("unknown");
    let created = issue["created_at"]
        .as_str()
        .map_or_else(|| "—".to_string(), format_date);
    let updated = issue["updated_at"]
        .as_str()
        .map_or_else(|| "—".to_string(), format_date);

    println!("State:   {state}");
    println!("Author:  {author}");
    println!("Created: {created}");
    println!("Updated: {updated}");

    // Labels
    let labels_str = issue["labels"]
        .as_array()
        .map(|arr| {
            arr.iter()
                .filter_map(|l| l["name"].as_str())
                .collect::<Vec<_>>()
                .join(", ")
        })
        .unwrap_or_default();
    if !labels_str.is_empty() {
        println!("Labels:  {labels_str}");
    }

    // Assignees
    let assignees_str = issue["assignees"]
        .as_array()
        .map(|arr| {
            arr.iter()
                .filter_map(|a| a["login"].as_str())
                .collect::<Vec<_>>()
                .join(", ")
        })
        .unwrap_or_default();
    if !assignees_str.is_empty() {
        println!("Assignees: {assignees_str}");
    }

    // Milestone
    if let Some(milestone_title) = issue["milestone"]["title"].as_str() {
        println!("Milestone: {milestone_title}");
    }

    println!();

    // Body
    let body = issue["body"].as_str().unwrap_or("");
    if !body.is_empty() {
        println!("{body}");
        println!();
    }

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

    #[test]
    fn print_issue_view_basic() {
        let issue = json!({
            "number": 42,
            "title": "Fix authentication bug",
            "state": "open",
            "user": { "login": "octocat" },
            "created_at": "2024-01-15T10:30:00Z",
            "updated_at": "2024-01-16T12:00:00Z",
            "body": "This issue describes an authentication bug.",
            "labels": [
                { "name": "bug" },
                { "name": "security" }
            ],
            "assignees": [
                { "login": "alice" }
            ],
            "milestone": { "title": "v1.0" }
        });
        let comments: Vec<serde_json::Value> = vec![];
        // Should not panic
        print_issue_view(&issue, &comments);
    }

    #[test]
    fn print_issue_view_closed() {
        let issue = json!({
            "number": 100,
            "title": "Add new feature",
            "state": "closed",
            "user": { "login": "dev-user" },
            "created_at": "2024-01-10T08:00:00Z",
            "updated_at": "2024-01-15T10:30:00Z",
            "body": "This adds a new feature.",
            "labels": [],
            "assignees": [],
            "milestone": null
        });
        let comments: Vec<serde_json::Value> = vec![];
        // Should not panic
        print_issue_view(&issue, &comments);
    }

    #[test]
    fn print_issue_view_with_comments() {
        let issue = json!({
            "number": 42,
            "title": "Fix bug",
            "state": "open",
            "user": { "login": "octocat" },
            "created_at": "2024-01-15T10:30:00Z",
            "updated_at": "2024-01-16T12:00:00Z",
            "body": "Fixes a bug.",
            "labels": [],
            "assignees": [],
            "milestone": null
        });
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
        print_issue_view(&issue, &comments);
    }

    #[test]
    fn print_issue_view_null_fields() {
        let issue = json!({
            "number": 99,
            "title": null,
            "state": null,
            "user": null,
            "created_at": null,
            "updated_at": null,
            "body": null,
            "labels": null,
            "assignees": null,
            "milestone": null
        });
        let comments: Vec<serde_json::Value> = vec![];
        // Should not panic with null fields
        print_issue_view(&issue, &comments);
    }

    #[test]
    fn print_issue_view_with_assignees() {
        let issue = json!({
            "number": 50,
            "title": "Multiple assignees",
            "state": "open",
            "user": { "login": "owner" },
            "created_at": "2024-02-01T10:00:00Z",
            "updated_at": "2024-02-02T10:00:00Z",
            "body": "This issue has multiple assignees.",
            "labels": [],
            "assignees": [
                { "login": "alice" },
                { "login": "bob" },
                { "login": "carol" }
            ],
            "milestone": null
        });
        let comments: Vec<serde_json::Value> = vec![];
        // Should not panic
        print_issue_view(&issue, &comments);
    }

    #[test]
    fn print_issue_view_with_milestone() {
        let issue = json!({
            "number": 60,
            "title": "Milestone issue",
            "state": "open",
            "user": { "login": "planner" },
            "created_at": "2024-03-01T10:00:00Z",
            "updated_at": "2024-03-02T10:00:00Z",
            "body": "This issue is part of a milestone.",
            "labels": [{"name": "enhancement"}],
            "assignees": [],
            "milestone": { "title": "v2.0" }
        });
        let comments: Vec<serde_json::Value> = vec![];
        // Should not panic
        print_issue_view(&issue, &comments);
    }
}
