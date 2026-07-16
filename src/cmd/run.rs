//! Implementation of the `gor run` subcommand.
//!
//! Provides workflow run listing.

#![allow(clippy::print_stdout, clippy::option_if_let_else)]

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
        RunCommand::View {
            id,
            repo,
            web,
            log,
            log_failed,
            json,
            hostname,
        } => view(
            id,
            repo.as_deref(),
            web,
            log,
            log_failed,
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

fn view(
    id: u64,
    repo: Option<&str>,
    web: bool,
    log: Option<u64>,
    log_failed: bool,
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

    let path = format!("/repos/{}/{}/actions/runs/{id}", spec.owner, spec.repo);
    let response = client.get(&path).context("failed to fetch run")?;

    let status = response.status();
    if status == reqwest::StatusCode::NOT_FOUND {
        anyhow::bail!("run #{id} not found");
    }
    if !status.is_success() {
        anyhow::bail!("failed to view run: HTTP {status}");
    }

    let run_data: serde_json::Value = response.json().context("failed to parse response")?;

    if web {
        if let Some(url) = run_data["html_url"].as_str() {
            println!("Open {url} in your browser");
            return Ok(());
        }
    }

    // Fetch jobs
    let jobs_path = format!("/repos/{}/{}/actions/runs/{id}/jobs", spec.owner, spec.repo);
    let jobs_response = client.get(&jobs_path);
    let jobs: Vec<serde_json::Value> = if let Ok(resp) = jobs_response {
        if resp.status().is_success() {
            if let Ok(jobs_data) = resp.json::<serde_json::Value>() {
                jobs_data["jobs"]
                    .as_array()
                    .map_or_else(Vec::new, Clone::clone)
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        }
    } else {
        Vec::new()
    };

    // Handle --log: show logs for a specific job
    if let Some(job_id) = log {
        let log_path = format!(
            "/repos/{}/{}/actions/jobs/{job_id}/logs",
            spec.owner, spec.repo
        );
        let log_response = client.get(&log_path);
        if let Ok(resp) = log_response {
            if resp.status().is_success() {
                if let Ok(body) = resp.text() {
                    print!("{body}");
                }
            }
        }
        return Ok(());
    }

    // Handle --log-failed: show logs for failed jobs
    if log_failed {
        for job in &jobs {
            let conclusion = job["conclusion"].as_str().unwrap_or("");
            if conclusion == "failure" || conclusion == "cancelled" {
                let job_id = job["id"].as_u64().unwrap_or(0);
                let job_name = job["name"].as_str().unwrap_or("—");
                println!(":: {job_name} (failed) ::");
                let log_path = format!(
                    "/repos/{}/{}/actions/jobs/{job_id}/logs",
                    spec.owner, spec.repo
                );
                let log_response = client.get(&log_path);
                if let Ok(resp) = log_response {
                    if resp.status().is_success() {
                        if let Ok(body) = resp.text() {
                            print!("{body}");
                        }
                    }
                }
                println!();
            }
        }
        return Ok(());
    }

    if let Some(fields) = json {
        let fields_ref: Option<&[String]> = if fields.is_empty() {
            None
        } else {
            Some(&fields)
        };
        print_json(&run_data, fields_ref);
        return Ok(());
    }

    // Default: print run details and jobs
    let run_status = run_data["status"].as_str().unwrap_or("—");
    let conclusion = run_data["conclusion"].as_str().unwrap_or("—");
    let event = run_data["event"].as_str().unwrap_or("—");
    let branch = run_data["head_branch"].as_str().unwrap_or("—");
    let created = run_data["created_at"].as_str().unwrap_or("—");

    println!("  Status: {run_status}");
    println!("  Conclusion: {conclusion}");
    println!("  Event: {event}");
    println!("  Branch: {branch}");
    println!("  Created: {created}");

    if !jobs.is_empty() {
        println!("  Jobs:");
        println!(
            "    {:<8}  {:<30}  {:<10}  {:<12}",
            "JOB ID", "NAME", "STATUS", "CONCLUSION"
        );
        for job in &jobs {
            let job_id = job["id"].as_u64().unwrap_or(0);
            let job_name = job["name"].as_str().unwrap_or("—");
            let job_status = job["status"].as_str().unwrap_or("—");
            let job_conclusion = job["conclusion"].as_str().unwrap_or("—");
            let name_truncated = crate::cmd::util::truncate(job_name, 30);
            println!(
                "    {job_id:<8}  {name_truncated:<30}  {job_status:<10}  {job_conclusion:<12}"
            );
        }
    }

    Ok(())
}
