//! Implementation of the `gor secret` subcommand.

#![allow(clippy::print_stdout)]

use crate::cli::SecretCommand;
use crate::cmd::util::truncate;
use crate::render::print_json;
use anyhow::Context;
use gor_core::Client;
use gor_core::repository::detect_remote;
use gor_core::secret::{self, SecretScope};

/// Run the `gor secret` subcommand.
///
/// # Errors
///
/// Returns an error if the command execution fails.
pub fn run(cmd: SecretCommand) -> anyhow::Result<()> {
    match cmd {
        SecretCommand::List {
            org,
            env,
            json,
            hostname,
        } => list(org.as_deref(), env.as_deref(), json, hostname.as_deref()),
        SecretCommand::Set {
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
        SecretCommand::Delete {
            name,
            org,
            env,
            hostname,
        } => delete(&name, org.as_deref(), env.as_deref(), hostname.as_deref()),
    }
}

fn resolve_scope(org: Option<&str>, env: Option<&str>) -> anyhow::Result<SecretScope> {
    if let Some(o) = org {
        return Ok(SecretScope::Org(o.to_string()));
    }
    if let Some(e) = env {
        let spec = detect_remote().ok_or_else(|| {
            anyhow::anyhow!(
                "could not detect repository; specify --repo or run from a repo directory"
            )
        })?;
        return Ok(SecretScope::Environment {
            spec,
            env: e.to_string(),
        });
    }
    let spec = detect_remote().ok_or_else(|| {
        anyhow::anyhow!("could not detect repository; specify --org or run from a repo directory")
    })?;
    Ok(SecretScope::Repo(spec))
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

    let secrets = secret::list(&client, &scope)?;

    if let Some(fields) = json {
        let fields_ref: Option<&[String]> = if fields.is_empty() {
            None
        } else {
            Some(&fields)
        };
        let values: Vec<serde_json::Value> = secrets
            .into_iter()
            .map(|s| serde_json::to_value(s).unwrap_or_default())
            .collect();
        print_json(&values, fields_ref);
        return Ok(());
    }

    if secrets.is_empty() {
        println!("No secrets found.");
        return Ok(());
    }

    println!("{:<30}  UPDATED", "NAME");
    for s in &secrets {
        let name_truncated = truncate(&s.name, 30);
        let updated = s.updated_at.as_deref().unwrap_or("—");
        println!("{name_truncated:<30}  {updated}");
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
        anyhow::bail!("no secret value provided (use --body or --file)");
    };

    secret::set(&client, &scope, name, &value)?;
    println!("Secret '{name}' set.");
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

    secret::delete(&client, &scope, name)?;
    println!("Secret '{name}' deleted.");
    Ok(())
}
