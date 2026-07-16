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
        RunCommand::Cancel { id, repo, hostname } => {
            cancel(id, repo.as_deref(), hostname.as_deref())
        }
        RunCommand::Download {
            id,
            repo,
            dir,
            names,
            log,
            hostname,
        } => download(id, repo.as_deref(), &dir, &names, log, hostname.as_deref()),
        RunCommand::Rerun {
            id,
            repo,
            failed_jobs,
            debug,
            hostname,
        } => rerun(id, repo.as_deref(), failed_jobs, debug, hostname.as_deref()),
        RunCommand::Watch {
            id,
            repo,
            interval,
            exit_status,
            hostname,
        } => watch(
            id,
            repo.as_deref(),
            interval,
            exit_status,
            hostname.as_deref(),
        ),
        RunCommand::Delete {
            id,
            repo,
            yes,
            hostname,
        } => delete_run(id, repo.as_deref(), yes, hostname.as_deref()),
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

fn cancel(id: u64, repo: Option<&str>, hostname: Option<&str>) -> anyhow::Result<()> {
    let spec = match repo {
        Some(s) => repository::parse_repo_spec(s).context("invalid repository spec")?,
        None => repository::detect_remote().ok_or_else(|| {
            anyhow::anyhow!("could not detect repository; specify OWNER/REPO with --repo")
        })?,
    };

    let host = hostname.unwrap_or("github.com");
    let client = Client::new(host).context("failed to create HTTP client")?;

    // First check the run status
    let path = format!("/repos/{}/{}/actions/runs/{id}", spec.owner, spec.repo);
    let response = client.get(&path).context("failed to fetch run")?;
    let status = response.status();
    if status == reqwest::StatusCode::NOT_FOUND {
        anyhow::bail!("run #{id} not found");
    }
    if !status.is_success() {
        anyhow::bail!("failed to fetch run: HTTP {status}");
    }

    let run_data: serde_json::Value = response.json().context("failed to parse response")?;
    let run_status = run_data["status"].as_str().unwrap_or("");

    let terminal_states = ["completed", "cancelled", "skipped", "timed_out"];
    if terminal_states.contains(&run_status) {
        let html_url = run_data["html_url"].as_str().unwrap_or("");
        anyhow::bail!("run #{id} is already {run_status} ({html_url})");
    }

    let cancel_path = format!(
        "/repos/{}/{}/actions/runs/{id}/cancel",
        spec.owner, spec.repo
    );
    let cancel_response = client
        .request("POST", &cancel_path, &[], None)
        .context("failed to cancel run")?;

    let cancel_status = cancel_response.status();
    if !cancel_status.is_success() {
        anyhow::bail!("failed to cancel run #{id}: HTTP {cancel_status}");
    }

    let html_url = run_data["html_url"].as_str().unwrap_or("");
    println!("Run #{id} cancelled ({html_url})");
    Ok(())
}

fn download(
    id: u64,
    repo: Option<&str>,
    dir: &str,
    names: &[String],
    log: bool,
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

    if log {
        // Download job logs
        let jobs_path = format!("/repos/{}/{}/actions/runs/{id}/jobs", spec.owner, spec.repo);
        let jobs_response = client.get(&jobs_path).context("failed to fetch jobs")?;
        let jobs_data: serde_json::Value = jobs_response.json().context("failed to parse jobs")?;
        let jobs: Vec<serde_json::Value> = jobs_data["jobs"]
            .as_array()
            .map_or_else(Vec::new, Clone::clone);

        if jobs.is_empty() {
            anyhow::bail!("no jobs found for run #{id}");
        }

        for job in &jobs {
            let job_id = job["id"].as_u64().unwrap_or(0);
            let job_name = job["name"].as_str().unwrap_or("unknown");
            let log_path = format!(
                "/repos/{}/{}/actions/jobs/{job_id}/logs",
                spec.owner, spec.repo
            );
            let log_response = client.get(&log_path).context("failed to download logs")?;
            if log_response.status().is_success() {
                let body = log_response.text().context("failed to read logs")?;
                let filename = format!("{dir}/{job_name}.log");
                std::fs::write(&filename, &body)
                    .with_context(|| format!("failed to write {filename}"))?;
                println!("Downloaded: {filename}");
            }
        }
        return Ok(());
    }

    // Download artifacts
    let artifacts_path = format!(
        "/repos/{}/{}/actions/runs/{id}/artifacts",
        spec.owner, spec.repo
    );
    let artifacts_response = client
        .get(&artifacts_path)
        .context("failed to fetch artifacts")?;
    let artifacts_data: serde_json::Value = artifacts_response
        .json()
        .context("failed to parse artifacts")?;
    let artifacts: Vec<serde_json::Value> = artifacts_data["artifacts"]
        .as_array()
        .map_or_else(Vec::new, Clone::clone);

    let filtered: Vec<&serde_json::Value> = if names.is_empty() {
        artifacts.iter().collect()
    } else {
        artifacts
            .iter()
            .filter(|a| {
                let a_name = a["name"].as_str().unwrap_or("");
                names.iter().any(|n| n == a_name)
            })
            .collect()
    };

    if filtered.is_empty() {
        anyhow::bail!("no artifacts found for run #{id}");
    }

    for artifact in &filtered {
        let artifact_id = artifact["id"].as_u64().unwrap_or(0);
        let artifact_name = artifact["name"].as_str().unwrap_or("unknown");
        let zip_path = format!(
            "/repos/{}/{}/actions/artifacts/{artifact_id}/zip",
            spec.owner, spec.repo
        );
        let zip_response = client
            .get(&zip_path)
            .context("failed to download artifact")?;
        if zip_response.status().is_success() {
            let bytes = zip_response.bytes().context("failed to read artifact")?;
            let filename = format!("{dir}/{artifact_name}.zip");
            std::fs::write(&filename, &bytes)
                .with_context(|| format!("failed to write {filename}"))?;
            println!("Downloaded: {filename}");
        }
    }

    Ok(())
}

fn rerun(
    id: u64,
    repo: Option<&str>,
    failed_jobs: bool,
    debug: bool,
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

    let path = format!(
        "/repos/{}/{}/actions/runs/{id}/rerun",
        spec.owner, spec.repo
    );

    let mut body = serde_json::Map::new();
    if failed_jobs {
        body.insert(
            "enable_debug_logging".to_string(),
            serde_json::Value::Bool(debug),
        );
    }

    let body_bytes = if body.is_empty() {
        None
    } else {
        Some(serde_json::to_vec(&body).context("serialize")?)
    };

    let response = client
        .request("POST", &path, &[], body_bytes)
        .context("failed to rerun workflow")?;

    let status = response.status();
    if !status.is_success() {
        anyhow::bail!("failed to rerun run #{id}: HTTP {status}");
    }

    let result: serde_json::Value = response.json().context("failed to parse response")?;
    let html_url = result["html_url"].as_str().unwrap_or("");
    println!("Run #{id} rerun: {html_url}");
    Ok(())
}

fn watch(
    id: u64,
    repo: Option<&str>,
    interval: u64,
    exit_status: bool,
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

    let terminal_states = ["completed", "cancelled", "skipped", "timed_out"];
    let mut prev_job_states: Vec<(u64, String)> = Vec::new();

    loop {
        let path = format!("/repos/{}/{}/actions/runs/{id}", spec.owner, spec.repo);
        let response = client.get(&path).context("failed to fetch run")?;
        let run_data: serde_json::Value = response.json().context("failed to parse response")?;
        let run_status = run_data["status"].as_str().unwrap_or("");
        let conclusion = run_data["conclusion"].as_str().unwrap_or("");

        // Fetch jobs
        let jobs_path = format!("/repos/{}/{}/actions/runs/{id}/jobs", spec.owner, spec.repo);
        if let Ok(jobs_resp) = client.get(&jobs_path) {
            if let Ok(jobs_data) = jobs_resp.json::<serde_json::Value>() {
                if let Some(jobs) = jobs_data["jobs"].as_array() {
                    for job in jobs {
                        let job_id = job["id"].as_u64().unwrap_or(0);
                        let job_name = job["name"].as_str().unwrap_or("");
                        let job_status = job["status"].as_str().unwrap_or("");
                        let job_conclusion = job["conclusion"].as_str().unwrap_or("");
                        let state_str = format!("{job_status}/{job_conclusion}");

                        let prev = prev_job_states.iter().find(|(jid, _)| *jid == job_id);
                        let changed = prev.is_none_or(|(_, s)| s != &state_str);

                        if changed {
                            println!("  {job_name}: {job_status} ({job_conclusion})");
                            prev_job_states.retain(|(jid, _)| *jid != job_id);
                            prev_job_states.push((job_id, state_str));
                        }
                    }
                }
            }
        }

        if terminal_states.contains(&run_status) {
            let html_url = run_data["html_url"].as_str().unwrap_or("");
            println!("Run #{id}: {run_status} ({conclusion}) — {html_url}");
            if exit_status && conclusion == "failure" {
                std::process::exit(1);
            }
            return Ok(());
        }

        std::thread::sleep(std::time::Duration::from_secs(interval));
    }
}

/// Execute `gor run delete`.
///
/// Deletes a workflow run and its logs.
///
/// # Errors
///
/// Returns an error if the run does not exist or the API request fails.
fn delete_run(
    id: u64,
    repo: Option<&str>,
    yes: bool,
    hostname: Option<&str>,
) -> anyhow::Result<()> {
    if !yes {
        use std::io::Write;
        print!("Are you sure you want to delete run #{id}? [y/N] ");
        std::io::stdout().flush().ok();

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .context("failed to read input")?;
        let input = input.trim().to_lowercase();
        if input != "y" && input != "yes" {
            println!("Cancelled.");
            return Ok(());
        }
    }

    let host = hostname.unwrap_or("github.com");
    let client = Client::new(host).context("failed to create HTTP client")?;

    let spec = if let Some(r) = repo {
        repository::parse_repo_spec(r).with_context(|| format!("invalid repository: {r}"))?
    } else {
        repository::detect_remote().context("could not detect repository from git remote")?
    };

    let path = format!("/repos/{}/{}/actions/runs/{id}", spec.owner, spec.repo);

    let response = client
        .request("DELETE", &path, &[], None)
        .context("failed to delete run")?;

    let status = response.status();
    if !status.is_success() {
        let err_body: serde_json::Value = response.json().unwrap_or_default();
        let msg = err_body["message"].as_str().unwrap_or("delete failed");
        anyhow::bail!("failed to delete run #{id}: {msg}");
    }

    println!("Run #{id} deleted.");
    Ok(())
}
