//! Implementation of the `gor org` subcommand.
//!
//! Provides organization listing for the authenticated user.

#![allow(clippy::print_stdout)]

use crate::cli::OrgCommand;
use crate::client::Client;
use crate::output::print_json;
use anyhow::Context;

/// Run the `gor org` subcommand.
///
/// # Errors
///
/// Returns an error if the command execution fails.
pub fn run(cmd: OrgCommand) -> anyhow::Result<()> {
    match cmd {
        OrgCommand::List {
            limit,
            json,
            hostname,
        } => list(limit, json, hostname.as_deref()),
        OrgCommand::View {
            org,
            web,
            json,
            hostname,
        } => view(&org, web, json, hostname.as_deref()),
    }
}

/// Execute `gor org list`.
///
/// Lists organizations the authenticated user belongs to.
///
/// # Errors
///
/// Returns an error if the API request fails.
fn list(limit: u32, json: Option<Vec<String>>, hostname: Option<&str>) -> anyhow::Result<()> {
    let host = hostname.unwrap_or("github.com");
    let client = Client::new(host).context("failed to create HTTP client")?;

    let path = format!("/user/orgs?per_page={}", limit.min(100));
    let response = client.get(&path).context("failed to fetch organizations")?;

    let status = response.status();
    if !status.is_success() {
        anyhow::bail!("failed to list organizations: HTTP {status}");
    }

    let mut orgs: Vec<serde_json::Value> =
        response.json().context("failed to parse orgs response")?;

    orgs.truncate(limit as usize);

    if let Some(fields) = json {
        let fields_ref: Option<&[String]> = if fields.is_empty() {
            None
        } else {
            Some(&fields)
        };
        print_json(&orgs, fields_ref);
        return Ok(());
    }

    print_org_table(&orgs);
    Ok(())
}

/// Execute `gor org view`.
///
/// Views an organization's profile and metadata.
///
/// # Errors
///
/// Returns an error if the API request fails.
fn view(
    org: &str,
    web: bool,
    json: Option<Vec<String>>,
    hostname: Option<&str>,
) -> anyhow::Result<()> {
    let host = hostname.unwrap_or("github.com");
    let client = Client::new(host).context("failed to create HTTP client")?;

    let path = format!("/orgs/{org}");
    let response = client.get(&path).context("failed to fetch organization")?;

    let status = response.status();
    if status == reqwest::StatusCode::NOT_FOUND {
        anyhow::bail!("organization '{org}' not found");
    }
    if !status.is_success() {
        anyhow::bail!("failed to view organization: HTTP {status}");
    }

    let org_data: serde_json::Value = response.json().context("failed to parse response")?;

    // --web / -w: open in browser
    if web {
        if let Some(url) = org_data["html_url"].as_str() {
            crate::cmd::browse::open_in_browser(url);
            return Ok(());
        }
    }

    // --json: output as JSON
    if let Some(fields) = json {
        let fields_ref: Option<&[String]> = if fields.is_empty() {
            None
        } else {
            Some(&fields)
        };
        print_json(&org_data, fields_ref);
        return Ok(());
    }

    // Default: print details
    let name = org_data["name"].as_str().unwrap_or("—");
    let description = org_data["description"].as_str().unwrap_or("No description");
    let location = org_data["location"].as_str().unwrap_or("—");
    let blog = org_data["blog"].as_str().unwrap_or("—");
    let email = org_data["email"].as_str().unwrap_or("—");
    let members = org_data["members_count"].as_u64().unwrap_or(0);
    let repos = org_data["public_repos"].as_u64().unwrap_or(0);

    println!("  Name: {name}");
    println!("  Description: {description}");
    println!("  Location: {location}");
    println!("  Website: {blog}");
    println!("  Email: {email}");
    println!("  Members: {members}");
    println!("  Public repos: {repos}");

    Ok(())
}

/// Print a formatted organization list table.
fn print_org_table(orgs: &[serde_json::Value]) {
    if orgs.is_empty() {
        println!("No organizations found.");
        return;
    }

    let login_width = 20;
    let desc_width = 50;

    println!("{:<login_width$}  {:<desc_width$}", "LOGIN", "DESCRIPTION");

    for org in orgs {
        let login = org["login"].as_str().unwrap_or("—");
        let description = org["description"].as_str().unwrap_or("—");

        let desc_truncated = crate::cmd::util::truncate(description, desc_width);

        println!("{login:<login_width$}  {desc_truncated:<desc_width$}");
    }
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn print_org_table_basic() {
        let orgs = vec![json!({
            "login": "my-org",
            "description": "My organization"
        })];
        print_org_table(&orgs);
    }

    #[test]
    fn print_org_table_empty() {
        let orgs: Vec<serde_json::Value> = vec![];
        print_org_table(&orgs);
    }
}
