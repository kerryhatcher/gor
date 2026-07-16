//! Implementation of the `gor secret` subcommand.
//!
//! Provides secret listing, creation, and deletion for GitHub Actions.

#![allow(clippy::print_stdout)]

use crate::cli::SecretCommand;
use crate::client::Client;
use crate::output::print_json;
use crate::repository::detect_remote;
use anyhow::Context;

/// Run the `gor secret` subcommand.
///
/// # Errors
///
/// Returns an error if the command execution fails.
pub fn run(cmd: SecretCommand) -> anyhow::Result<()> {
    match cmd {
        SecretCommand::List {
            org,
            json,
            hostname,
        } => list(org.as_deref(), json, hostname.as_deref()),
        SecretCommand::Set {
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
        SecretCommand::Delete {
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
        format!("/orgs/{o}/actions/secrets?per_page=100")
    } else {
        let spec = detect_remote().ok_or_else(|| {
            anyhow::anyhow!(
                "could not detect repository; specify --org or run from a repo directory"
            )
        })?;
        format!(
            "/repos/{}/{}/actions/secrets?per_page=100",
            spec.owner, spec.repo
        )
    };

    let response = client.get(&path).context("failed to fetch secrets")?;
    let status = response.status();
    if !status.is_success() {
        anyhow::bail!("failed to list secrets: HTTP {status}");
    }

    let result: serde_json::Value = response.json().context("failed to parse response")?;
    let secrets: Vec<serde_json::Value> = result["secrets"]
        .as_array()
        .map_or_else(Vec::new, Clone::clone);

    if let Some(fields) = json {
        let fields_ref: Option<&[String]> = if fields.is_empty() {
            None
        } else {
            Some(&fields)
        };
        print_json(&secrets, fields_ref);
        return Ok(());
    }

    if secrets.is_empty() {
        println!("No secrets found.");
        return Ok(());
    }

    println!("{:<30}  UPDATED", "NAME");
    for s in &secrets {
        let name = s["name"].as_str().unwrap_or("—");
        let updated = s["updated_at"].as_str().map_or("—", |d| d);
        let name_truncated = crate::cmd::util::truncate(name, 30);
        println!("{name_truncated:<30}  {updated}");
    }

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
        anyhow::bail!("no secret value provided (use --body or --file)");
    };

    let body_value = serde_json::json!({"encrypted_value": value});

    let path = if let Some(o) = org {
        format!("/orgs/{o}/actions/secrets/{name}")
    } else {
        let spec = detect_remote().ok_or_else(|| {
            anyhow::anyhow!(
                "could not detect repository; specify --org or run from a repo directory"
            )
        })?;
        format!("/repos/{}/{}/actions/secrets/{name}", spec.owner, spec.repo)
    };

    let response = client
        .request(
            "PUT",
            &path,
            &[],
            Some(serde_json::to_vec(&body_value).context("serialize")?),
        )
        .context("failed to set secret")?;

    let status = response.status();
    if !status.is_success() {
        anyhow::bail!("failed to set secret '{name}': HTTP {status}");
    }

    println!("Secret '{name}' set.");
    Ok(())
}

fn delete(name: &str, org: Option<&str>, hostname: Option<&str>) -> anyhow::Result<()> {
    let host = hostname.unwrap_or("github.com");
    let client = Client::new(host).context("failed to create HTTP client")?;

    let path = if let Some(o) = org {
        format!("/orgs/{o}/actions/secrets/{name}")
    } else {
        let spec = detect_remote().ok_or_else(|| {
            anyhow::anyhow!(
                "could not detect repository; specify --org or run from a repo directory"
            )
        })?;
        format!("/repos/{}/{}/actions/secrets/{name}", spec.owner, spec.repo)
    };

    let response = client
        .request("DELETE", &path, &[], None)
        .context("failed to delete secret")?;

    let status = response.status();
    if status == 404 {
        anyhow::bail!("secret '{name}' not found");
    }
    if !status.is_success() {
        anyhow::bail!("failed to delete secret '{name}': HTTP {status}");
    }

    println!("Secret '{name}' deleted.");
    Ok(())
}
