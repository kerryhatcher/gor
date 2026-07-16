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
