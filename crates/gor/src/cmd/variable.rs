//! Implementation of the `gor variable` subcommand.

#![allow(clippy::print_stdout)]

use crate::cli::VariableCommand;
use crate::cmd::util::truncate;
use crate::render::print_json;
use anyhow::Context;
use gor_core::Client;
use gor_core::repository::detect_remote;
use gor_core::variable::{self, VariableScope};

/// Run the `gor variable` subcommand.
///
/// # Errors
///
/// Returns an error if the command execution fails.
pub fn run(cmd: VariableCommand) -> anyhow::Result<()> {
    match cmd {
        VariableCommand::List {
            org,
            env,
            json,
            hostname,
        } => list(org.as_deref(), env.as_deref(), json, hostname.as_deref()),
        VariableCommand::Set {
            name,
            body,
            file,
            org,
            env,
            hostname,
        } => set(
            &name,
            body.as_deref(),
            file.as_deref(),
            org.as_deref(),
            env.as_deref(),
            hostname.as_deref(),
        ),
        VariableCommand::Delete {
            name,
            org,
            env,
            hostname,
        } => delete(&name, org.as_deref(), env.as_deref(), hostname.as_deref()),
    }
}

fn resolve_scope(org: Option<&str>, env: Option<&str>) -> anyhow::Result<VariableScope> {
    if let Some(o) = org {
        return Ok(VariableScope::Org(o.to_string()));
    }
    if let Some(e) = env {
        let spec = detect_remote().ok_or_else(|| {
            anyhow::anyhow!(
                "could not detect repository; specify --repo or run from a repo directory"
            )
        })?;
        return Ok(VariableScope::Environment {
            spec,
            env: e.to_string(),
        });
    }
    let spec = detect_remote().ok_or_else(|| {
        anyhow::anyhow!("could not detect repository; specify --org or run from a repo directory")
    })?;
    Ok(VariableScope::Repo(spec))
}

fn build_client(hostname: Option<&str>) -> anyhow::Result<Client> {
    let host = hostname.unwrap_or("github.com");
    Client::new(host).map_err(|e| anyhow::anyhow!("failed to create HTTP client: {e}"))
}

fn list(
    org: Option<&str>,
    env: Option<&str>,
    json: Option<Vec<String>>,
    hostname: Option<&str>,
) -> anyhow::Result<()> {
    let scope = resolve_scope(org, env)?;
    let client = build_client(hostname)?;

    let variables = variable::list(&client, &scope)?;

    if let Some(fields) = json {
        let fields_ref: Option<&[String]> = if fields.is_empty() {
            None
        } else {
            Some(&fields)
        };
        let values: Vec<serde_json::Value> = variables
            .into_iter()
            .map(|v| serde_json::to_value(v).unwrap_or_default())
            .collect();
        print_json(&values, fields_ref);
        return Ok(());
    }

    if variables.is_empty() {
        println!("No variables found.");
        return Ok(());
    }

    println!("{:<30}  {:<40}  UPDATED", "NAME", "VALUE");
    for v in &variables {
        let name_truncated = truncate(&v.name, 30);
        let value_truncated = truncate(&v.value, 40);
        let updated = v.updated_at.as_deref().unwrap_or("—");
        println!("{name_truncated:<30}  {value_truncated:<40}  {updated}");
    }

    Ok(())
}

fn set(
    name: &str,
    body: Option<&str>,
    file: Option<&str>,
    org: Option<&str>,
    env: Option<&str>,
    hostname: Option<&str>,
) -> anyhow::Result<()> {
    let scope = resolve_scope(org, env)?;
    let client = build_client(hostname)?;

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

    variable::set(&client, &scope, name, &value)?;
    println!("Variable '{name}' set.");
    Ok(())
}

fn delete(
    name: &str,
    org: Option<&str>,
    env: Option<&str>,
    hostname: Option<&str>,
) -> anyhow::Result<()> {
    let scope = resolve_scope(org, env)?;
    let client = build_client(hostname)?;

    variable::delete(&client, &scope, name)?;
    println!("Variable '{name}' deleted.");
    Ok(())
}
