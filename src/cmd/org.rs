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
