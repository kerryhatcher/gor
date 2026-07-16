//! Implementation of the `gor ruleset` subcommand.
//!
//! Provides repository ruleset listing and viewing.

#![allow(clippy::print_stdout)]

use crate::cli::RulesetCommand;
use crate::client::Client;
use crate::output::print_json;
use crate::repository::{detect_remote, parse_repo_spec};
use anyhow::Context;

/// Run the `gor ruleset` subcommand.
///
/// # Errors
///
/// Returns an error if the command execution fails.
pub fn run(cmd: RulesetCommand) -> anyhow::Result<()> {
    match cmd {
        RulesetCommand::List {
            repo,
            json,
            hostname,
        } => list(repo.as_deref(), json, hostname.as_deref()),
        RulesetCommand::View {
            id,
            repo,
            web,
            json,
            hostname,
        } => view(id, repo.as_deref(), web, json, hostname.as_deref()),
    }
}

fn list(
    repo: Option<&str>,
    json: Option<Vec<String>>,
    hostname: Option<&str>,
) -> anyhow::Result<()> {
    let spec = match repo {
        Some(s) => parse_repo_spec(s).context("invalid repository spec")?,
        None => detect_remote().ok_or_else(|| {
            anyhow::anyhow!("could not detect repository; specify OWNER/REPO with --repo")
        })?,
    };

    let host = hostname.unwrap_or("github.com");
    let client = Client::new(host).context("failed to create HTTP client")?;

    let path = format!("/repos/{}/{}/rulesets?per_page=100", spec.owner, spec.repo);

    let response = client.get(&path).context("failed to fetch rulesets")?;
    let status = response.status();
    if !status.is_success() {
        anyhow::bail!("failed to list rulesets: HTTP {status}");
    }

    let rulesets: Vec<serde_json::Value> = response.json().context("failed to parse response")?;

    if let Some(fields) = json {
        let fields_ref: Option<&[String]> = if fields.is_empty() {
            None
        } else {
            Some(&fields)
        };
        print_json(&rulesets, fields_ref);
        return Ok(());
    }

    if rulesets.is_empty() {
        println!("No rulesets found.");
        return Ok(());
    }

    println!("{:<8}  {:<30}  ENFORCEMENT", "ID", "NAME");
    for r in &rulesets {
        let id = r["id"].as_u64().unwrap_or(0);
        let name = r["name"].as_str().unwrap_or("—");
        let enforcement = r["enforcement"].as_str().unwrap_or("—");
        let name_truncated = crate::cmd::util::truncate(name, 30);
        println!("{id:<8}  {name_truncated:<30}  {enforcement}");
    }

    Ok(())
}

fn view(
    id: u32,
    repo: Option<&str>,
    web: bool,
    json: Option<Vec<String>>,
    hostname: Option<&str>,
) -> anyhow::Result<()> {
    let spec = match repo {
        Some(s) => parse_repo_spec(s).context("invalid repository spec")?,
        None => detect_remote().ok_or_else(|| {
            anyhow::anyhow!("could not detect repository; specify OWNER/REPO with --repo")
        })?,
    };

    let host = hostname.unwrap_or("github.com");
    let client = Client::new(host).context("failed to create HTTP client")?;

    let path = format!("/repos/{}/{}/rulesets/{id}", spec.owner, spec.repo);
    let response = client.get(&path).context("failed to fetch ruleset")?;

    let status = response.status();
    if status == reqwest::StatusCode::NOT_FOUND {
        anyhow::bail!("ruleset #{id} not found in repository '{spec}'");
    }
    if !status.is_success() {
        anyhow::bail!("failed to view ruleset: HTTP {status}");
    }

    let ruleset: serde_json::Value = response.json().context("failed to parse response")?;

    if web {
        let url = format!(
            "https://{host}/{}/{}/settings/rules/{id}",
            spec.owner, spec.repo
        );
        println!("Open {url} in your browser");
        return Ok(());
    }

    if let Some(fields) = json {
        let fields_ref: Option<&[String]> = if fields.is_empty() {
            None
        } else {
            Some(&fields)
        };
        print_json(&ruleset, fields_ref);
        return Ok(());
    }

    let name = ruleset["name"].as_str().unwrap_or("—");
    let enforcement = ruleset["enforcement"].as_str().unwrap_or("—");
    let target = ruleset["target"].as_str().unwrap_or("—");

    println!("  Name: {name}");
    println!("  Enforcement: {enforcement}");
    println!("  Target: {target}");

    if let Some(rules) = ruleset["rules"].as_array() {
        println!("  Rules:");
        for rule in rules {
            let rule_type = rule["type"].as_str().unwrap_or("—");
            println!("    - {rule_type}");
        }
    }

    Ok(())
}
