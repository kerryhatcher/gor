//! Implementation of the `gor project` subcommand.

#![allow(clippy::print_stdout)]

use crate::cli::ProjectCommand;
use crate::cmd::util::truncate;
use crate::render::print_json;
use anyhow::Context;
use gor_core::Client;
use gor_core::project::{self, ProjectScope};
use gor_core::repository;

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
            v2,
            limit,
            json,
            hostname,
        } => list(
            org.as_deref(),
            owner.as_deref(),
            repo.as_deref(),
            v2,
            limit,
            json,
            hostname.as_deref(),
        ),
        ProjectCommand::View {
            number,
            org,
            owner,
            web,
            json,
            hostname,
        } => view(
            number,
            org.as_deref(),
            owner.as_deref(),
            web,
            json,
            hostname.as_deref(),
        ),
        ProjectCommand::ItemAdd {
            project,
            issue,
            pull_request,
            org,
            owner,
            hostname,
        } => item_add(
            project,
            issue,
            pull_request,
            org.as_deref(),
            owner.as_deref(),
            hostname.as_deref(),
        ),
    }
}

fn resolve_scope(
    org: Option<&str>,
    owner: Option<&str>,
    repo: Option<&str>,
) -> anyhow::Result<ProjectScope> {
    if let Some(o) = org {
        Ok(ProjectScope::Org(o.to_string()))
    } else if let Some(u) = owner {
        Ok(ProjectScope::User(u.to_string()))
    } else if let Some(r) = repo {
        let spec = repository::parse_repo_spec(r)?;
        Ok(ProjectScope::Repo(spec))
    } else {
        let spec = repository::detect_remote().ok_or_else(|| {
            anyhow::anyhow!("could not detect repository; specify --org, --owner, or --repo")
        })?;
        Ok(ProjectScope::Repo(spec))
    }
}

fn build_client(hostname: Option<&str>) -> anyhow::Result<Client> {
    let host = hostname.unwrap_or("github.com");
    Client::new(host).map_err(|e| anyhow::anyhow!("failed to create HTTP client: {e}"))
}

fn list(
    org: Option<&str>,
    owner: Option<&str>,
    repo: Option<&str>,
    v2: bool,
    limit: u32,
    json: Option<Vec<String>>,
    hostname: Option<&str>,
) -> anyhow::Result<()> {
    let client = build_client(hostname)?;

    if v2 {
        return list_v2(&client, org, owner, limit, json);
    }

    let scope = resolve_scope(org, owner, repo)?;
    let projects = project::list(&client, &scope, limit)?;

    if let Some(fields) = json {
        let fields_ref: Option<&[String]> = if fields.is_empty() {
            None
        } else {
            Some(&fields)
        };
        let values: Vec<serde_json::Value> = projects
            .into_iter()
            .map(|p| serde_json::to_value(p).unwrap_or_default())
            .collect();
        print_json(&values, fields_ref);
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
        let title_truncated = truncate(&p.name, 30);
        let state = p.state.as_deref().unwrap_or("—");
        let visibility = p.visibility.as_deref().unwrap_or("—");
        println!(
            "{:<8}  {title_truncated:<30}  {state:<10}  {visibility}",
            p.number
        );
    }

    Ok(())
}

/// List Projects V2 using the GraphQL API.
fn list_v2(
    client: &Client,
    org: Option<&str>,
    owner: Option<&str>,
    limit: u32,
    json: Option<Vec<String>>,
) -> anyhow::Result<()> {
    let (owner_type, login) = if let Some(o) = org {
        ("organization", o.to_string())
    } else if let Some(u) = owner {
        ("user", u.to_string())
    } else {
        anyhow::bail!("specify --org or --owner for Projects V2");
    };

    let query = format!(
        r#"{{
  {owner_type}(login: "{login}") {{
    projectsV2(first: {limit}) {{
      nodes {{
        number
        title
        closed
      }}
    }}
  }}
}}"#
    );

    let result = client
        .graphql(&query, None)
        .context("failed to query Projects V2")?;

    let projects: Vec<serde_json::Value> = result["data"][owner_type]["projectsV2"]["nodes"]
        .as_array()
        .map_or_else(Vec::new, Clone::clone);

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
        println!("No Projects V2 found.");
        return Ok(());
    }

    println!("{:<8}  {:<30}  CLOSED", "NUMBER", "TITLE");
    for p in &projects {
        let number = p["number"].as_u64().unwrap_or(0);
        let title = p["title"].as_str().unwrap_or("—");
        let closed = p["closed"].as_bool().unwrap_or(false);
        let title_truncated = truncate(title, 30);
        println!("{number:<8}  {title_truncated:<30}  {closed}");
    }

    Ok(())
}

fn view(
    number: u64,
    _org: Option<&str>,
    _owner: Option<&str>,
    web: bool,
    json: Option<Vec<String>>,
    hostname: Option<&str>,
) -> anyhow::Result<()> {
    let client = build_client(hostname)?;
    let project = project::view(&client, number)?;

    if web {
        if let Some(url) = project["html_url"].as_str() {
            println!("Open {url} in your browser");
            return Ok(());
        }
        anyhow::bail!("no URL found for project #{number}");
    }

    if let Some(fields) = json {
        let fields_ref: Option<&[String]> = if fields.is_empty() {
            None
        } else {
            Some(&fields)
        };
        print_json(&project, fields_ref);
        return Ok(());
    }

    let title = project["name"].as_str().unwrap_or("—");
    let body = project["body"].as_str().unwrap_or("—");
    let state = project["state"].as_str().unwrap_or("—");
    let creator = project["creator"]["login"].as_str().unwrap_or("—");
    let created = project["created_at"].as_str().unwrap_or("—");
    let updated = project["updated_at"].as_str().unwrap_or("—");

    println!("Project #{number}: {title}");
    println!("  State: {state}");
    println!("  Creator: {creator}");
    println!("  Created: {created}");
    println!("  Updated: {updated}");
    if body != "—" && !body.is_empty() {
        println!("  Body: {body}");
    }

    Ok(())
}

fn item_add(
    project: u64,
    issue: Option<u64>,
    pull_request: Option<u64>,
    _org: Option<&str>,
    _owner: Option<&str>,
    hostname: Option<&str>,
) -> anyhow::Result<()> {
    let client = build_client(hostname)?;

    let (item_type, item_id) = if let Some(i) = issue {
        ("Issue", i)
    } else if let Some(pr) = pull_request {
        ("PullRequest", pr)
    } else {
        anyhow::bail!("specify --issue or --pr to add an item");
    };

    let result = project::item_add(&client, project, item_id, item_type)?;
    let result_item_id = result["id"].as_u64().unwrap_or(0);
    println!("Added {item_type} #{item_id} to project #{project} (item ID: {result_item_id})");

    Ok(())
}
