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
        CodespaceCommand::Delete {
            name,
            repo,
            yes,
            hostname,
        } => delete(&name, repo.as_deref(), yes, hostname.as_deref()),
        CodespaceCommand::Logs {
            name,
            repo,
            json,
            follow,
            hostname,
        } => logs(&name, repo.as_deref(), json, follow, hostname.as_deref()),
        CodespaceCommand::Ssh {
            name,
            repo,
            profile,
            config,
            hostname,
        } => ssh(
            &name,
            repo.as_deref(),
            profile.as_deref(),
            config,
            hostname.as_deref(),
        ),
        CodespaceCommand::Stop {
            name,
            repo,
            all,
            hostname,
        } => stop(name.as_deref(), repo.as_deref(), all, hostname.as_deref()),
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

fn delete(name: &str, repo: Option<&str>, yes: bool, hostname: Option<&str>) -> anyhow::Result<()> {
    let host = hostname.unwrap_or("github.com");
    let client = Client::new(host).context("failed to create HTTP client")?;

    if !yes {
        use std::io::Write;
        let prompt = if let Some(r) = repo {
            format!("Are you sure you want to delete codespace '{name}' in repo '{r}'?")
        } else {
            format!("Are you sure you want to delete codespace '{name}'?")
        };
        print!("{prompt} [y/N] ");
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

    let path = format!("/user/codespaces/{name}");

    let response = client
        .request("DELETE", &path, &[], None)
        .context("failed to delete codespace")?;

    let status = response.status();
    if !status.is_success() {
        anyhow::bail!("failed to delete codespace '{name}': HTTP {status}");
    }

    println!("Codespace '{name}' deleted.");
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

fn logs(
    name: &str,
    _repo: Option<&str>,
    json: Option<Vec<String>>,
    follow: bool,
    hostname: Option<&str>,
) -> anyhow::Result<()> {
    let host = hostname.unwrap_or("github.com");
    let client = Client::new(host).context("failed to create HTTP client")?;

    let path = format!("/user/codespaces/{name}/logs");

    let response = client
        .get(&path)
        .context("failed to fetch codespace logs")?;
    let status = response.status();
    if !status.is_success() {
        anyhow::bail!("failed to fetch logs for '{name}': HTTP {status}");
    }

    let body = response.text().context("failed to read response")?;

    if let Some(fields) = json {
        let parsed: serde_json::Value = serde_json::from_str(&body).unwrap_or_default();
        let fields_ref: Option<&[String]> = if fields.is_empty() {
            None
        } else {
            Some(&fields)
        };
        print_json(&parsed, fields_ref);
        return Ok(());
    }

    if follow {
        println!("Logs for codespace '{name}':");
    }
    println!("{body}");

    if follow {
        tracing::warn!("log following is not yet implemented");
    }

    Ok(())
}

fn ssh(
    name: &str,
    _repo: Option<&str>,
    _profile: Option<&str>,
    config: bool,
    hostname: Option<&str>,
) -> anyhow::Result<()> {
    let host = hostname.unwrap_or("github.com");
    let client = Client::new(host).context("failed to create HTTP client")?;

    let path = format!("/user/codespaces/{name}");

    let response = client
        .get(&path)
        .context("failed to fetch codespace details")?;
    let status = response.status();
    if !status.is_success() {
        anyhow::bail!("failed to fetch codespace '{name}': HTTP {status}");
    }

    let cs: serde_json::Value = response.json().context("failed to parse response")?;
    let state = cs["state"].as_str().unwrap_or("unknown");

    if state != "Available" {
        anyhow::bail!("codespace '{name}' is not available (state: {state})");
    }

    // Fetch SSH connection details
    let ssh_path = format!("/user/codespaces/{name}");
    let ssh_response = client
        .get(&ssh_path)
        .context("failed to fetch SSH details")?;
    let ssh_details: serde_json::Value =
        ssh_response.json().context("failed to parse SSH details")?;

    if config {
        let connection = ssh_details.get("connection").and_then(|c| c.as_object());
        if let Some(conn) = connection {
            println!("Host {name}");
            if let Some(hostname) = conn.get("host").and_then(|v| v.as_str()) {
                println!("  HostName {hostname}");
            }
            if let Some(port) = conn.get("port") {
                println!("  Port {port}");
            }
            if let Some(user) = conn.get("user").and_then(|v| v.as_str()) {
                println!("  User {user}");
            }
        } else {
            println!("# No SSH connection details available for '{name}'");
        }
        return Ok(());
    }

    anyhow::bail!("SSH connection is not yet implemented; use --config to view connection details");
}

fn stop(
    name: Option<&str>,
    _repo: Option<&str>,
    all: bool,
    hostname: Option<&str>,
) -> anyhow::Result<()> {
    let host = hostname.unwrap_or("github.com");
    let client = Client::new(host).context("failed to create HTTP client")?;

    if all {
        // List all codespaces and stop each one
        let response = client
            .get("/user/codespaces")
            .context("failed to list codespaces")?;
        let result: serde_json::Value = response.json().context("failed to parse response")?;
        let spaces: Vec<serde_json::Value> = result["codespaces"]
            .as_array()
            .map_or_else(Vec::new, Clone::clone);

        let mut stopped = 0u32;
        for s in &spaces {
            let cs_name = s["name"].as_str().unwrap_or("");
            let cs_state = s["state"].as_str().unwrap_or("");
            if cs_state == "Shutdown" || cs_state == "Deleted" {
                continue;
            }
            let stop_path = format!("/user/codespaces/{cs_name}/stop");
            let resp = client
                .request("POST", &stop_path, &[], None)
                .context("failed to stop codespace")?;
            if resp.status().is_success() {
                stopped += 1;
            }
        }
        println!("Stopped {stopped} codespace(s).");
        return Ok(());
    }

    let cs_name = name.ok_or_else(|| anyhow::anyhow!("codespace name is required"))?;
    let path = format!("/user/codespaces/{cs_name}/stop");

    let response = client
        .request("POST", &path, &[], None)
        .context("failed to stop codespace")?;

    let status = response.status();
    if !status.is_success() {
        anyhow::bail!("failed to stop codespace '{cs_name}': HTTP {status}");
    }

    println!("Codespace '{cs_name}' stopped.");
    Ok(())
}
