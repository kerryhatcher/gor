//! Implementation of the `gor run` subcommand.
//!
//! Provides workflow run listing.

#![allow(clippy::print_stdout)]

use crate::cli::RunCommand;
use crate::client::Client;
use crate::output::print_json;
use crate::repository;
use anyhow::Context;
use std::fmt::Write;

/// Run the `gor run` subcommand.
///
/// # Errors
///
/// Returns an error if the command execution fails.
pub fn run(cmd: RunCommand) -> anyhow::Result<()> {
    match cmd {
        RunCommand::List {
            workflow,
            branch,
            repo,
            limit,
            json,
            hostname,
        } => list(
            workflow.as_deref(),
            branch.as_deref(),
            repo.as_deref(),
            limit,
            json,
            hostname.as_deref(),
        ),
    }
}

fn list(
    workflow: Option<&str>,
    branch: Option<&str>,
    repo: Option<&str>,
    limit: u32,
    json: Option<Vec<String>>,
    hostname: Option<&str>,
) -> anyhow::Result<()> {
    let spec = match repo {
        Some(s) => repository::parse_repo_spec(s).context("invalid repository spec")?,
        None => repository::detect_remote().ok_or_else(|| {
            anyhow::anyhow!("could not detect repository; specify OWNER/REPO with --repo")
        })?,
    };

    let host = hostname.unwrap_or("github.com");
    let client = Client::new(host).context("failed to create HTTP client")?;

    let mut path = format!(
        "/repos/{}/{}/actions/runs?per_page={}",
        spec.owner,
        spec.repo,
        limit.min(100)
    );

    if let Some(wf) = workflow {
        let _ = write!(path, "&workflow={wf}");
    }
    if let Some(br) = branch {
        let _ = write!(path, "&branch={br}");
    }

    let response = client.get(&path).context("failed to fetch workflow runs")?;
    let status = response.status();
    if !status.is_success() {
        anyhow::bail!("failed to list runs: HTTP {status}");
    }

    let result: serde_json::Value = response.json().context("failed to parse response")?;
    let mut runs: Vec<serde_json::Value> = result["workflow_runs"]
        .as_array()
        .map_or_else(Vec::new, Clone::clone);

    runs.truncate(limit as usize);

    if let Some(fields) = json {
        let fields_ref: Option<&[String]> = if fields.is_empty() {
            None
        } else {
            Some(&fields)
        };
        print_json(&runs, fields_ref);
        return Ok(());
    }

    if runs.is_empty() {
        println!("No workflow runs found.");
        return Ok(());
    }

    println!(
        "{:<8}  {:<10}  {:<12}  {:20}  EVENT",
        "RUN ID", "STATUS", "CONCLUSION", "BRANCH"
    );
    for r in &runs {
        let id = r["id"].as_u64().unwrap_or(0);
        let status = r["status"].as_str().unwrap_or("—");
        let conclusion = r["conclusion"].as_str().unwrap_or("—");
        let branch = r["head_branch"].as_str().unwrap_or("—");
        let event = r["event"].as_str().unwrap_or("—");
        let branch_truncated = crate::cmd::util::truncate(branch, 20);
        println!("{id:<8}  {status:<10}  {conclusion:<12}  {branch_truncated:<20}  {event}");
    }

    Ok(())
}
