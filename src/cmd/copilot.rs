//! Implementation of the `gor copilot` subcommand.
//!
//! Provides Copilot status and usage information.

#![allow(clippy::print_stdout)]

use crate::cli::CopilotCommand;
use crate::client::Client;
use crate::output::print_json;
use anyhow::Context;

/// Run the `gor copilot` subcommand.
///
/// # Errors
///
/// Returns an error if the command execution fails.
pub fn run(cmd: CopilotCommand) -> anyhow::Result<()> {
    match cmd {
        CopilotCommand::Status { json, hostname } => status(json, hostname.as_deref()),
        CopilotCommand::Usage {
            org,
            json,
            hostname,
        } => usage(org.as_deref(), json, hostname.as_deref()),
    }
}

/// Show Copilot subscription status for the authenticated user.
///
/// # Errors
///
/// Returns an error if the API request fails.
fn status(json: Option<Vec<String>>, hostname: Option<&str>) -> anyhow::Result<()> {
    let host = hostname.unwrap_or("github.com");
    let client = Client::new(host).context("failed to create HTTP client")?;

    let response = client
        .get("/user/copilot/subscription")
        .context("failed to fetch Copilot subscription")?;

    let status_code = response.status();
    if status_code == reqwest::StatusCode::NOT_FOUND {
        println!("Copilot is not enabled for your account.");
        return Ok(());
    }
    if !status_code.is_success() {
        anyhow::bail!("failed to fetch Copilot status: HTTP {status_code}");
    }

    let sub: serde_json::Value = response.json().context("failed to parse response")?;

    if let Some(fields) = json {
        let fields_ref: Option<&[String]> = if fields.is_empty() {
            None
        } else {
            Some(&fields)
        };
        print_json(&sub, fields_ref);
        return Ok(());
    }

    let plan = sub["plan"].as_str().unwrap_or("—");
    let seat = sub["seat_status"].as_str().unwrap_or("—");
    let renewal = sub["renewal_date"].as_str().unwrap_or("—");

    println!("Copilot Status");
    println!("  Plan: {plan}");
    println!("  Seat: {seat}");
    println!("  Renewal: {renewal}");

    Ok(())
}

/// Show Copilot usage statistics for an organization.
///
/// # Errors
///
/// Returns an error if the API request fails.
fn usage(
    org: Option<&str>,
    json: Option<Vec<String>>,
    hostname: Option<&str>,
) -> anyhow::Result<()> {
    let host = hostname.unwrap_or("github.com");
    let client = Client::new(host).context("failed to create HTTP client")?;

    let org_name = org.ok_or_else(|| anyhow::anyhow!("--org is required for usage statistics"))?;

    let path = format!("/orgs/{org_name}/copilot/usage");
    let response = client.get(&path).context("failed to fetch Copilot usage")?;

    let status_code = response.status();
    if status_code == reqwest::StatusCode::NOT_FOUND {
        println!("Copilot is not enabled for organization '{org_name}'.");
        return Ok(());
    }
    if !status_code.is_success() {
        anyhow::bail!("failed to fetch Copilot usage: HTTP {status_code}");
    }

    let usage_data: serde_json::Value = response.json().context("failed to parse response")?;

    if let Some(fields) = json {
        let fields_ref: Option<&[String]> = if fields.is_empty() {
            None
        } else {
            Some(&fields)
        };
        print_json(&usage_data, fields_ref);
        return Ok(());
    }

    // Display usage summary
    let total = usage_data["total_active_users"].as_u64().unwrap_or(0);
    let breakdown = usage_data["breakdown"]
        .as_array()
        .map_or_else(Vec::new, Clone::clone);

    println!("Copilot Usage for {org_name}");
    println!("  Total active users: {total}");

    if !breakdown.is_empty() {
        println!("  Breakdown:");
        for entry in &breakdown {
            let lang = entry["language"].as_str().unwrap_or("—");
            let editors = entry["editor_count"].as_u64().unwrap_or(0);
            println!("    {lang}: {editors} editors");
        }
    }

    Ok(())
}
