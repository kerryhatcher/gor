//! Implementation of the `gor org` subcommand.

#![allow(clippy::print_stdout)]

use crate::cli::OrgCommand;
use crate::cmd::util::truncate;
use crate::render::print_json;
use gor_core::Client;
use gor_core::org::{self, Organization};

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

fn build_client(hostname: Option<&str>) -> anyhow::Result<Client> {
    let host = hostname.unwrap_or("github.com");
    Client::new(host).map_err(|e| anyhow::anyhow!("failed to create HTTP client: {e}"))
}

fn list(limit: u32, json: Option<Vec<String>>, hostname: Option<&str>) -> anyhow::Result<()> {
    let client = build_client(hostname)?;

    let orgs = org::list(&client, limit)?;

    if let Some(fields) = json {
        let fields_ref: Option<&[String]> = if fields.is_empty() {
            None
        } else {
            Some(&fields)
        };
        let values: Vec<serde_json::Value> = orgs
            .into_iter()
            .map(|o| serde_json::to_value(o).unwrap_or_default())
            .collect();
        print_json(&values, fields_ref);
        return Ok(());
    }

    print_org_table(&orgs);
    Ok(())
}

fn view(
    org_name: &str,
    web: bool,
    json: Option<Vec<String>>,
    hostname: Option<&str>,
) -> anyhow::Result<()> {
    let client = build_client(hostname)?;
    let org_data = org::view(&client, org_name)?;

    if web {
        if let Some(url) = org_data["html_url"].as_str() {
            crate::cmd::browse::open_in_browser(url);
            return Ok(());
        }
    }

    if let Some(fields) = json {
        let fields_ref: Option<&[String]> = if fields.is_empty() {
            None
        } else {
            Some(&fields)
        };
        print_json(&org_data, fields_ref);
        return Ok(());
    }

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

fn print_org_table(orgs: &[Organization]) {
    if orgs.is_empty() {
        println!("No organizations found.");
        return;
    }

    let login_width = 20;
    let desc_width = 50;

    println!("{:<login_width$}  {:<desc_width$}", "LOGIN", "DESCRIPTION");

    for org in orgs {
        let login = &org.login;
        let desc_truncated = truncate(org.description.as_deref().unwrap_or("—"), desc_width);
        println!("{login:<login_width$}  {desc_truncated:<desc_width$}");
    }
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod tests {
    use super::*;
    use serde_json::json;

    fn make_org(login: &str, description: Option<&str>) -> Organization {
        serde_json::from_value(json!({
            "login": login,
            "description": description,
        }))
        .expect("valid org")
    }

    #[test]
    fn print_org_table_basic() {
        let orgs = vec![make_org("my-org", Some("My organization"))];
        print_org_table(&orgs);
    }

    #[test]
    fn print_org_table_empty() {
        let orgs: Vec<Organization> = vec![];
        print_org_table(&orgs);
    }
}
