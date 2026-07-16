//! Implementation of the `gor workflow` subcommand.
//!
//! Provides GitHub Actions workflow listing.

#![allow(clippy::print_stdout)]

use crate::cli::WorkflowCommand;
use crate::client::Client;
use crate::output::print_json;
use crate::repository::{detect_remote, parse_repo_spec};
use anyhow::Context;

/// Run the `gor workflow` subcommand.
///
/// # Errors
///
/// Returns an error if the command execution fails.
pub fn run(cmd: WorkflowCommand) -> anyhow::Result<()> {
    match cmd {
        WorkflowCommand::List {
            repo,
            limit,
            json,
            hostname,
        } => list(repo.as_deref(), limit, json, hostname.as_deref()),
        WorkflowCommand::View {
            workflow,
            repo,
            json,
            hostname,
        } => view(&workflow, repo.as_deref(), json, hostname.as_deref()),
        WorkflowCommand::Enable {
            workflow,
            repo,
            hostname,
        } => enable(&workflow, repo.as_deref(), hostname.as_deref()),
        WorkflowCommand::Disable {
            workflow,
            repo,
            hostname,
        } => disable(&workflow, repo.as_deref(), hostname.as_deref()),
        WorkflowCommand::Run {
            workflow,
            repo,
            branch,
            hostname,
        } => trigger_workflow_run(
            &workflow,
            repo.as_deref(),
            branch.as_deref(),
            hostname.as_deref(),
        ),
    }
}

/// Execute `gor workflow list`.
///
/// Lists GitHub Actions workflows in a repository.
///
/// # Errors
///
/// Returns an error if the repository cannot be found or the API request fails.
fn list(
    repo: Option<&str>,
    limit: u32,
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
        "/repos/{}/{}/actions/workflows?per_page={}",
        spec.owner,
        spec.repo,
        limit.min(100)
    );
    let response = client.get(&path).context("failed to fetch workflows")?;

    let status = response.status();
    if status == reqwest::StatusCode::NOT_FOUND {
        anyhow::bail!("repository '{spec}' not found");
    }
    if !status.is_success() {
        anyhow::bail!("failed to list workflows for '{spec}': HTTP {status}");
    }

    let result: serde_json::Value = response
        .json()
        .context("failed to parse workflows response")?;
    let mut workflows: Vec<serde_json::Value> = result["workflows"]
        .as_array()
        .map_or_else(Vec::new, Clone::clone);

    workflows.truncate(limit as usize);

    if let Some(fields) = json {
        let fields_ref: Option<&[String]> = if fields.is_empty() {
            None
        } else {
            Some(&fields)
        };
        print_json(&workflows, fields_ref);
        return Ok(());
    }

    print_workflow_table(&workflows);
    Ok(())
}

/// Execute `gor workflow view`.
///
/// Views a workflow's details and recent runs.
///
/// # Errors
///
/// Returns an error if the repository cannot be found or the API request fails.
fn view(
    workflow: &str,
    repo: Option<&str>,
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

    // Determine if the workflow arg is a numeric ID or a filename/name
    let path = if workflow.parse::<u64>().is_ok() {
        format!(
            "/repos/{}/{}/actions/workflows/{workflow}",
            spec.owner, spec.repo
        )
    } else {
        // URL-encode the filename: replace / with %2F
        let encoded = workflow.replace('/', "%2F");
        format!(
            "/repos/{}/{}/actions/workflows/{encoded}",
            spec.owner, spec.repo
        )
    };

    let response = client.get(&path).context("failed to fetch workflow")?;

    let status = response.status();
    if status == reqwest::StatusCode::NOT_FOUND {
        anyhow::bail!("workflow '{workflow}' not found in repository '{spec}'");
    }
    if !status.is_success() {
        anyhow::bail!("failed to view workflow: HTTP {status}");
    }

    let wf: serde_json::Value = response
        .json()
        .context("failed to parse workflow response")?;

    // Fetch recent runs (up to 5)
    let runs_path = format!(
        "/repos/{}/{}/actions/workflows/{}/runs?per_page=5",
        spec.owner, spec.repo, workflow
    );
    let runs_response = client.get(&runs_path);
    let recent_runs: Vec<serde_json::Value> = runs_response.map_or_else(
        |_| Vec::new(),
        |resp| {
            if resp.status().is_success() {
                resp.json::<serde_json::Value>().map_or_else(
                    |_| Vec::new(),
                    |runs_data| {
                        runs_data["workflow_runs"]
                            .as_array()
                            .map_or_else(Vec::new, Clone::clone)
                    },
                )
            } else {
                Vec::new()
            }
        },
    );

    // --json: output as JSON
    if let Some(fields) = json {
        let fields_ref: Option<&[String]> = if fields.is_empty() {
            None
        } else {
            Some(&fields)
        };
        print_json(&wf, fields_ref);
        return Ok(());
    }

    // Default: print details
    let name = wf["name"].as_str().unwrap_or("—");
    let state = wf["state"].as_str().unwrap_or("—");
    let wf_path = wf["path"].as_str().unwrap_or("—");

    println!("  Name: {name}");
    println!("  State: {state}");
    println!("  Path: {wf_path}");

    // Print trigger events
    if let Some(events) = wf["triggering_events"].as_array() {
        if !events.is_empty() {
            println!("  Events:");
            for event in events {
                let event_str = serde_json::to_string(event).unwrap_or_default();
                println!("    - {event_str}");
            }
        }
    }

    // Print recent runs
    if !recent_runs.is_empty() {
        println!("  Recent runs:");
        println!(
            "    {:<8}  {:<10}  {:<12}  {:<20}  {:<16}",
            "RUN ID", "STATUS", "CONCLUSION", "BRANCH", "TIMESTAMP"
        );
        for run in &recent_runs {
            let run_id = run["id"].as_u64().unwrap_or(0);
            let run_status = run["status"].as_str().unwrap_or("—");
            let conclusion = run["conclusion"].as_str().unwrap_or("—");
            let branch = run["head_branch"].as_str().unwrap_or("—");
            let created = run["created_at"]
                .as_str()
                .map_or_else(|| "—".to_string(), crate::output::format_date);

            println!(
                "    {run_id:<8}  {run_status:<10}  {conclusion:<12}  {branch:<20}  {created:<16}",
            );
        }
    }

    Ok(())
}

/// Print a formatted workflow list table.
fn print_workflow_table(workflows: &[serde_json::Value]) {
    if workflows.is_empty() {
        println!("No workflows found.");
        return;
    }

    let name_width = 40;
    let state_width = 12;
    let id_width = 8;

    println!(
        "{:<id_width$}  {:<name_width$}  {:<state_width$}",
        "ID", "NAME", "STATE",
    );

    for wf in workflows {
        let id = wf["id"].as_u64().unwrap_or(0);
        let name = wf["name"].as_str().unwrap_or("—");
        let state = wf["state"].as_str().unwrap_or("—");

        let name_truncated = crate::cmd::util::truncate(name, name_width);

        println!("{id:<id_width$}  {name_truncated:<name_width$}  {state:<state_width$}");
    }
}

/// Execute `gor workflow enable`.
///
/// Enables a workflow.
///
/// # Errors
///
/// Returns an error if the repository cannot be found or the API request fails.
fn enable(workflow: &str, repo: Option<&str>, hostname: Option<&str>) -> anyhow::Result<()> {
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
        "/repos/{}/{}/actions/workflows/{workflow}/enable",
        spec.owner, spec.repo
    );

    let response = client
        .request("PUT", &path, &[], None)
        .context("failed to enable workflow")?;

    let status = response.status();
    if !status.is_success() {
        anyhow::bail!("failed to enable workflow '{workflow}': HTTP {status}");
    }

    println!("Workflow '{workflow}' is now enabled.");
    Ok(())
}

/// Execute `gor workflow disable`.
///
/// Disables a workflow.
///
/// # Errors
///
/// Returns an error if the repository cannot be found or the API request fails.
fn disable(workflow: &str, repo: Option<&str>, hostname: Option<&str>) -> anyhow::Result<()> {
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
        "/repos/{}/{}/actions/workflows/{workflow}/disable",
        spec.owner, spec.repo
    );

    let response = client
        .request("PUT", &path, &[], None)
        .context("failed to disable workflow")?;

    let status = response.status();
    if !status.is_success() {
        anyhow::bail!("failed to disable workflow '{workflow}': HTTP {status}");
    }

    println!("Workflow '{workflow}' is now disabled.");
    Ok(())
}

/// Execute `gor workflow run`.
///
/// Triggers a workflow dispatch run.
///
/// # Errors
///
/// Returns an error if the repository cannot be found or the API request fails.
fn trigger_workflow_run(
    workflow: &str,
    repo: Option<&str>,
    branch: Option<&str>,
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

    let body_value = serde_json::json!({
        "ref": branch.unwrap_or("main"),
    });
    let body_bytes = serde_json::to_vec(&body_value).context("failed to serialize body")?;

    let path = format!(
        "/repos/{}/{}/actions/workflows/{workflow}/dispatches",
        spec.owner, spec.repo
    );

    let response = client
        .request("POST", &path, &[], Some(body_bytes))
        .context("failed to trigger workflow run")?;

    let status = response.status();
    if !status.is_success() {
        anyhow::bail!("failed to trigger workflow run: HTTP {status}");
    }

    println!("Workflow run triggered for '{workflow}'.");
    Ok(())
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn print_workflow_table_basic() {
        let workflows = vec![
            json!({
                "id": 12345,
                "name": "CI",
                "state": "active"
            }),
            json!({
                "id": 67890,
                "name": "Deploy",
                "state": "active"
            }),
        ];
        print_workflow_table(&workflows);
    }

    #[test]
    fn print_workflow_table_empty() {
        let workflows: Vec<serde_json::Value> = vec![];
        print_workflow_table(&workflows);
    }
}
