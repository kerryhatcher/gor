//! Implementation of the `gor codespace` subcommand.
//!
//! Provides codespace listing and creation.

#![allow(clippy::print_stdout, clippy::option_if_let_else)]

use crate::cli::CodespaceCommand;
use crate::client::Client;
use crate::output::print_json;
use anyhow::Context;

/// Run the `gor codespace` subcommand.
///
/// # Errors
///
/// Returns an error if the command execution fails.
pub fn run(cmd: CodespaceCommand) -> anyhow::Result<()> {
    match cmd {
        CodespaceCommand::List {
            repo,
            json,
            hostname,
        } => list(repo.as_deref(), json, hostname.as_deref()),
        CodespaceCommand::Create {
            repo,
            branch,
            machine,
            hostname,
        } => create(
            &repo,
            branch.as_deref(),
            machine.as_deref(),
            hostname.as_deref(),
        ),
    }
}

fn list(
    repo: Option<&str>,
    json: Option<Vec<String>>,
    hostname: Option<&str>,
) -> anyhow::Result<()> {
    let host = hostname.unwrap_or("github.com");
    let client = Client::new(host).context("failed to create HTTP client")?;

    let path = if let Some(r) = repo {
        format!("/user/codespaces?repository_id={r}")
    } else {
        "/user/codespaces".to_string()
    };

    let response = client.get(&path).context("failed to fetch codespaces")?;
    let status = response.status();
    if !status.is_success() {
        anyhow::bail!("failed to list codespaces: HTTP {status}");
    }

    let result: serde_json::Value = response.json().context("failed to parse response")?;
    let spaces: Vec<serde_json::Value> = result["codespaces"]
        .as_array()
        .map_or_else(Vec::new, Clone::clone);

    if let Some(fields) = json {
        let fields_ref: Option<&[String]> = if fields.is_empty() {
            None
        } else {
            Some(&fields)
        };
        print_json(&spaces, fields_ref);
        return Ok(());
    }

    if spaces.is_empty() {
        println!("No codespaces found.");
        return Ok(());
    }

    println!(
        "{:<24}  {:<20}  {:<15}  BRANCH",
        "NAME", "REPOSITORY", "STATE"
    );
    for s in &spaces {
        let name = s["name"].as_str().unwrap_or("—");
        let repo_name = s["repository"]["full_name"].as_str().unwrap_or("—");
        let state = s["state"].as_str().unwrap_or("—");
        let branch = s["git_status"]["branch"].as_str().unwrap_or("—");
        let name_truncated = crate::cmd::util::truncate(name, 24);
        let repo_truncated = crate::cmd::util::truncate(repo_name, 20);
        println!("{name_truncated:<24}  {repo_truncated:<20}  {state:<15}  {branch}");
    }

    Ok(())
}

fn create(
    repo: &str,
    branch: Option<&str>,
    machine: Option<&str>,
    hostname: Option<&str>,
) -> anyhow::Result<()> {
    let host = hostname.unwrap_or("github.com");
    let client = Client::new(host).context("failed to create HTTP client")?;

    let mut body_map = serde_json::Map::new();
    body_map.insert(
        "repository_id".to_string(),
        serde_json::Value::String(repo.to_string()),
    );

    if let Some(b) = branch {
        body_map.insert("git_status".to_string(), serde_json::json!({"branch": b}));
    }

    if let Some(m) = machine {
        body_map.insert(
            "machine".to_string(),
            serde_json::Value::String(m.to_string()),
        );
    }

    let body_value = serde_json::Value::Object(body_map);
    let body_bytes = serde_json::to_vec(&body_value).context("failed to serialize body")?;

    let response = client
        .request("POST", "/user/codespaces", &[], Some(body_bytes))
        .context("failed to create codespace")?;

    let status = response.status();
    if !status.is_success() {
        let err_body: serde_json::Value = response.json().unwrap_or_default();
        let msg = err_body["message"].as_str().unwrap_or("create failed");
        anyhow::bail!("failed to create codespace: {msg}");
    }

    let result: serde_json::Value = response.json().context("failed to parse response")?;
    let name = result["name"].as_str().unwrap_or("—");
    println!("Codespace '{name}' created.");
    Ok(())
}
