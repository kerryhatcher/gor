//! Implementation of the `gor project` subcommand.
//!
//! Provides project listing for organizations and repositories.

#![allow(clippy::print_stdout)]

use crate::cli::ProjectCommand;
use crate::client::Client;
use crate::output::print_json;
use crate::repository;
use anyhow::Context;

/// Run the `gor project` subcommand.
///
/// # Errors
///
/// Returns an error if the command execution fails.
pub fn run(cmd: ProjectCommand) -> anyhow::Result<()> {
    match cmd {
        ProjectCommand::List {
            org,
            owner,
            repo,
            limit,
            json,
            hostname,
        } => list(
            org.as_deref(),
            owner.as_deref(),
            repo.as_deref(),
            limit,
            json,
            hostname.as_deref(),
        ),
    }
}

fn list(
    org: Option<&str>,
    owner: Option<&str>,
    repo: Option<&str>,
    limit: u32,
    json: Option<Vec<String>>,
    hostname: Option<&str>,
) -> anyhow::Result<()> {
    let host = hostname.unwrap_or("github.com");
    let client = Client::new(host).context("failed to create HTTP client")?;

    let path = if let Some(o) = org {
        format!("/orgs/{o}/projects?per_page={}", limit.min(100))
    } else if let Some(u) = owner {
        format!("/users/{u}/projects?per_page={}", limit.min(100))
    } else if let Some(r) = repo {
        let spec = repository::parse_repo_spec(r).context("invalid repository spec")?;
        format!(
            "/repos/{}/{}/projects?per_page={}",
            spec.owner,
            spec.repo,
            limit.min(100)
        )
    } else {
        let spec = repository::detect_remote().ok_or_else(|| {
            anyhow::anyhow!("could not detect repository; specify --org, --owner, or --repo")
        })?;
        format!(
            "/repos/{}/{}/projects?per_page={}",
            spec.owner,
            spec.repo,
            limit.min(100)
        )
    };

    let response = client.get(&path).context("failed to fetch projects")?;
    let status = response.status();
    if !status.is_success() {
        anyhow::bail!("failed to list projects: HTTP {status}");
    }

    let mut projects: Vec<serde_json::Value> =
        response.json().context("failed to parse response")?;
    projects.truncate(limit as usize);

    if let Some(fields) = json {
        let fields_ref: Option<&[String]> = if fields.is_empty() {
            None
        } else {
            Some(&fields)
        };
        print_json(&projects, fields_ref);
        return Ok(());
    }

    if projects.is_empty() {
        println!("No projects found.");
        return Ok(());
    }

    println!(
        "{:<8}  {:<30}  {:<10}  VISIBILITY",
        "NUMBER", "TITLE", "STATE"
    );
    for p in &projects {
        let number = p["number"].as_u64().unwrap_or(0);
        let title = p["name"].as_str().unwrap_or("—");
        let state = p["state"].as_str().unwrap_or("—");
        let visibility = p["visibility"].as_str().unwrap_or("—");
        let title_truncated = crate::cmd::util::truncate(title, 30);
        println!("{number:<8}  {title_truncated:<30}  {state:<10}  {visibility}");
    }

    Ok(())
}
