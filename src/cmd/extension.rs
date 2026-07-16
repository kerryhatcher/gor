//! Implementation of the `gor extension` subcommand.
//!
//! Provides extension listing and installation.

#![allow(clippy::print_stdout)]

use crate::cli::ExtensionCommand;
use crate::client::Client;
use crate::output::print_json;
use anyhow::Context;

/// Run the `gor extension` subcommand.
///
/// # Errors
///
/// Returns an error if the command execution fails.
pub fn run(cmd: ExtensionCommand) -> anyhow::Result<()> {
    match cmd {
        ExtensionCommand::List { json, hostname } => list(json, hostname.as_deref()),
        ExtensionCommand::Install { name, hostname } => install(&name, hostname.as_deref()),
        ExtensionCommand::Remove { name, hostname } => remove(&name, hostname.as_deref()),
    }
}

fn list(json: Option<Vec<String>>, hostname: Option<&str>) -> anyhow::Result<()> {
    let host = hostname.unwrap_or("github.com");
    let client = Client::new(host).context("failed to create HTTP client")?;

    let response = client
        .get("/user/repos?type=owner&per_page=100&sort=full_name")
        .context("failed to fetch extensions")?;
    let status = response.status();
    if !status.is_success() {
        anyhow::bail!("failed to list extensions: HTTP {status}");
    }

    let repos: Vec<serde_json::Value> = response.json().context("failed to parse response")?;

    if let Some(fields) = json {
        let fields_ref: Option<&[String]> = if fields.is_empty() {
            None
        } else {
            Some(&fields)
        };
        print_json(&repos, fields_ref);
        return Ok(());
    }

    if repos.is_empty() {
        println!("No extensions found.");
        return Ok(());
    }

    println!("{:<30}  DESCRIPTION", "NAME");
    for r in &repos {
        let name = r["full_name"].as_str().unwrap_or("—");
        let desc = r["description"].as_str().unwrap_or("—");
        let name_truncated = crate::cmd::util::truncate(name, 30);
        let desc_truncated = crate::cmd::util::truncate(desc, 50);
        println!("{name_truncated:<30}  {desc_truncated}");
    }

    Ok(())
}

fn install(_name: &str, _hostname: Option<&str>) -> anyhow::Result<()> {
    anyhow::bail!(
        "extension installation is not yet implemented; use `gh extension install` instead"
    );
}

fn remove(name: &str, _hostname: Option<&str>) -> anyhow::Result<()> {
    let ext_dir = dirs::home_dir()
        .unwrap_or_default()
        .join(".local")
        .join("share")
        .join("gor")
        .join("extensions")
        .join(name);

    if !ext_dir.exists() {
        println!("Extension '{name}' is not installed.");
        return Ok(());
    }

    std::fs::remove_dir_all(&ext_dir)
        .with_context(|| format!("failed to remove extension '{name}'"))?;

    println!("Extension '{name}' removed.");
    Ok(())
}
