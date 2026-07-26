//! Implementation of the `gor workflow` subcommand.

#![allow(clippy::print_stdout)]

use crate::cli::WorkflowCommand;
use crate::cmd::util::truncate;
use crate::render::print_json;
use gor_core::Client;
use gor_core::repository::{detect_remote, parse_repo_spec};
use gor_core::workflow::{self};

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
        } => trigger_run(
            &workflow,
            repo.as_deref(),
            branch.as_deref(),
            hostname.as_deref(),
        ),
    }
}

fn resolve_spec(repo: Option<&str>) -> anyhow::Result<gor_core::RepoSplit> {
    match repo {
        Some(s) => Ok(parse_repo_spec(s)?),
        None => detect_remote().ok_or_else(|| {
            anyhow::anyhow!("could not detect repository; specify OWNER/REPO with --repo")
        }),
    }
}

fn client(hostname: Option<&str>) -> anyhow::Result<Client> {
    let host = hostname.unwrap_or("github.com");
    Client::new(host).map_err(|e| anyhow::anyhow!("{e}"))
}

fn list(
    repo: Option<&str>,
    limit: u32,
    json: Option<Vec<String>>,
    hostname: Option<&str>,
) -> anyhow::Result<()> {
    let s = resolve_spec(repo)?;
    let c = client(hostname)?;
    let workflows = workflow::list(&c, &s, limit)?;
    if let Some(fields) = json {
        let fields_ref = if fields.is_empty() {
            None
        } else {
            Some(fields.as_slice())
        };
        let v: Vec<serde_json::Value> = workflows
            .into_iter()
            .map(|w| serde_json::to_value(w).unwrap_or_default())
            .collect();
        print_json(&v, fields_ref);
        return Ok(());
    }
    if workflows.is_empty() {
        println!("No workflows found.");
        return Ok(());
    }
    println!("{:<8}  {:<30}  STATE", "ID", "NAME");
    for w in &workflows {
        let name = truncate(&w.name, 30);
        let state = w.state.as_deref().unwrap_or("—");
        println!("{:<8}  {name:<30}  {state}", w.id);
    }
    Ok(())
}

fn view(
    workflow: &str,
    repo: Option<&str>,
    json: Option<Vec<String>>,
    hostname: Option<&str>,
) -> anyhow::Result<()> {
    let s = resolve_spec(repo)?;
    let c = client(hostname)?;
    let wf = workflow::view(&c, &s, workflow)?;
    if let Some(fields) = json {
        let fields_ref = if fields.is_empty() {
            None
        } else {
            Some(fields.as_slice())
        };
        print_json(&wf, fields_ref);
        return Ok(());
    }
    let name = wf["name"].as_str().unwrap_or("—");
    let state = wf["state"].as_str().unwrap_or("—");
    let path = wf["path"].as_str().unwrap_or("—");
    println!("  Name: {name}");
    println!("  State: {state}");
    println!("  Path: {path}");
    Ok(())
}

fn enable(workflow: &str, repo: Option<&str>, hostname: Option<&str>) -> anyhow::Result<()> {
    let s = resolve_spec(repo)?;
    let c = client(hostname)?;
    workflow::enable(&c, &s, workflow)?;
    println!("Workflow '{workflow}' enabled.");
    Ok(())
}

fn disable(workflow: &str, repo: Option<&str>, hostname: Option<&str>) -> anyhow::Result<()> {
    let s = resolve_spec(repo)?;
    let c = client(hostname)?;
    workflow::disable(&c, &s, workflow)?;
    println!("Workflow '{workflow}' disabled.");
    Ok(())
}

fn trigger_run(
    workflow: &str,
    repo: Option<&str>,
    branch: Option<&str>,
    hostname: Option<&str>,
) -> anyhow::Result<()> {
    let s = resolve_spec(repo)?;
    let c = client(hostname)?;
    workflow::trigger_run(&c, &s, workflow, branch.unwrap_or("main"))?;
    println!("Workflow run triggered for '{workflow}'.");
    Ok(())
}
