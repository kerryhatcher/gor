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

fn list(
    org: Option<&str>,
    owner: Option<&str>,
    repo: Option<&str>,
    v2: bool,
    limit: u32,
    json: Option<Vec<String>>,
    hostname: Option<&str>,
) -> anyhow::Result<()> {
    let host = hostname.unwrap_or("github.com");
    let client = Client::new(host).context("failed to create HTTP client")?;

    if v2 {
        return list_v2(&client, org, owner, limit, json);
    }

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
        let title_truncated = crate::cmd::util::truncate(title, 30);
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
    let host = hostname.unwrap_or("github.com");
    let client = Client::new(host).context("failed to create HTTP client")?;

    let path = format!("/projects/{number}");
    let response = client.get(&path).context("failed to fetch project")?;

    let status = response.status();
    if status == reqwest::StatusCode::NOT_FOUND {
        anyhow::bail!("project #{number} not found");
    }
    if !status.is_success() {
        anyhow::bail!("failed to view project: HTTP {status}");
    }

    let project: serde_json::Value = response.json().context("failed to parse response")?;

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
    let host = hostname.unwrap_or("github.com");
    let client = Client::new(host).context("failed to create HTTP client")?;

    let (item_type, item_id) = if let Some(i) = issue {
        ("Issue", i)
    } else if let Some(pr) = pull_request {
        ("PullRequest", pr)
    } else {
        anyhow::bail!("specify --issue or --pr to add an item");
    };

    let body = serde_json::json!({
        "content_id": item_id,
        "content_type": item_type,
    });

    let path = format!("/projects/{project}/items");
    let body_bytes = serde_json::to_vec(&body).context("serialize")?;

    let response = client
        .request("POST", &path, &[], Some(body_bytes))
        .context("failed to add project item")?;

    let status = response.status();
    if !status.is_success() {
        let err: serde_json::Value = response.json().unwrap_or_default();
        let msg = err["message"].as_str().unwrap_or("add failed");
        anyhow::bail!("failed to add item to project #{project}: {msg}");
    }

    let result: serde_json::Value = response.json().context("failed to parse response")?;
    let item_id = result["id"].as_u64().unwrap_or(0);
    println!("Added {item_type} #{item_id} to project #{project} (item ID: {item_id})");

    Ok(())
}
