//! Implementation of the `gor variable` subcommand.
//!
//! Provides variable listing and creation for GitHub Actions.

#![allow(clippy::print_stdout)]

use crate::cli::VariableCommand;
use crate::client::Client;
use crate::output::print_json;
use crate::repository::detect_remote;
use anyhow::Context;

/// Run the `gor variable` subcommand.
///
/// # Errors
///
/// Returns an error if the command execution fails.
pub fn run(cmd: VariableCommand) -> anyhow::Result<()> {
    match cmd {
        VariableCommand::List {
            org,
            json,
            hostname,
        } => list(org.as_deref(), json, hostname.as_deref()),
        VariableCommand::Set {
            name,
            body,
            file,
            org,
            hostname,
        } => set(
            &name,
            body.as_deref(),
            file.as_deref(),
            org.as_deref(),
            hostname.as_deref(),
        ),
        VariableCommand::Delete {
            name,
            org,
            hostname,
        } => delete(&name, org.as_deref(), hostname.as_deref()),
    }
}

fn list(
    org: Option<&str>,
    json: Option<Vec<String>>,
    hostname: Option<&str>,
) -> anyhow::Result<()> {
    let host = hostname.unwrap_or("github.com");
    let client = Client::new(host).context("failed to create HTTP client")?;

    let path = if let Some(o) = org {
        format!("/orgs/{o}/actions/variables?per_page=100")
    } else {
        let spec = detect_remote().ok_or_else(|| {
            anyhow::anyhow!(
                "could not detect repository; specify --org or run from a repo directory"
            )
        })?;
        format!(
            "/repos/{}/{}/actions/variables?per_page=100",
            spec.owner, spec.repo
        )
    };

    let response = client.get(&path).context("failed to fetch variables")?;
    let status = response.status();
    if !status.is_success() {
        anyhow::bail!("failed to list variables: HTTP {status}");
    }

    let result: serde_json::Value = response.json().context("failed to parse response")?;
    let vars: Vec<serde_json::Value> = result["variables"]
        .as_array()
        .map_or_else(Vec::new, Clone::clone);

    if let Some(fields) = json {
        let fields_ref: Option<&[String]> = if fields.is_empty() {
            None
        } else {
            Some(&fields)
        };
        print_json(&vars, fields_ref);
        return Ok(());
    }

    if vars.is_empty() {
        println!("No variables found.");
        return Ok(());
    }

    println!("{:<30}  VALUE", "NAME");
    for v in &vars {
        let name = v["name"].as_str().unwrap_or("—");
        let value = v["value"].as_str().unwrap_or("—");
        let name_truncated = crate::cmd::util::truncate(name, 30);
        let value_truncated = crate::cmd::util::truncate(value, 40);
        println!("{name_truncated:<30}  {value_truncated}");
    }

    Ok(())
}

fn delete(name: &str, org: Option<&str>, hostname: Option<&str>) -> anyhow::Result<()> {
    let host = hostname.unwrap_or("github.com");
    let client = Client::new(host).context("failed to create HTTP client")?;

    let path = if let Some(o) = org {
        format!("/orgs/{o}/actions/variables/{name}")
    } else {
        let spec = detect_remote().ok_or_else(|| {
            anyhow::anyhow!(
                "could not detect repository; specify --org or run from a repo directory"
            )
        })?;
        format!(
            "/repos/{}/{}/actions/variables/{name}",
            spec.owner, spec.repo
        )
    };

    let response = client
        .request("DELETE", &path, &[], None)
        .context("failed to delete variable")?;

    let status = response.status();
    if status == 404 {
        anyhow::bail!("variable '{name}' not found");
    }
    if !status.is_success() {
        anyhow::bail!("failed to delete variable '{name}': HTTP {status}");
    }

    println!("Variable '{name}' deleted.");
    Ok(())
}

fn set(
    name: &str,
    body: Option<&str>,
    file: Option<&str>,
    org: Option<&str>,
    hostname: Option<&str>,
) -> anyhow::Result<()> {
    let host = hostname.unwrap_or("github.com");
    let client = Client::new(host).context("failed to create HTTP client")?;

    let value = if let Some(b) = body {
        b.to_string()
    } else if let Some(f) = file {
        std::fs::read_to_string(f)
            .with_context(|| format!("failed to read file: {f}"))?
            .trim()
            .to_string()
    } else {
        anyhow::bail!("no variable value provided (use --body or --file)");
    };

    let body_value = serde_json::json!({"value": value});

    let path = if let Some(o) = org {
        format!("/orgs/{o}/actions/variables/{name}")
    } else {
        let spec = detect_remote().ok_or_else(|| {
            anyhow::anyhow!(
                "could not detect repository; specify --org or run from a repo directory"
            )
        })?;
        format!(
            "/repos/{}/{}/actions/variables/{name}",
            spec.owner, spec.repo
        )
    };

    let response = client
        .request(
            "PATCH",
            &path,
            &[],
            Some(serde_json::to_vec(&body_value).context("serialize")?),
        )
        .context("failed to set variable")?;

    let status = response.status();
    if !status.is_success() {
        anyhow::bail!("failed to set variable '{name}': HTTP {status}");
    }

    println!("Variable '{name}' set.");
    Ok(())
}
